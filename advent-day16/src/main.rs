use std::fs;
use std::i64;
use std::str;

struct PParser {
    ver_sum: u32,
    depth: u32,
}

// RECURSION
impl PParser {
    // REFERENCE TO MUTABLE INT
    fn read_int(bytes_str: &[u8], start: &mut usize, chars: usize) -> u32 {
        let end = *start + chars;
        let str = str::from_utf8(&bytes_str[*start..end]).unwrap();
        *start += chars;
        return u32::from_str_radix(str, 2).unwrap();
    }

    fn read_move(start: &mut usize, chars: usize) {
        *start += chars;
    }

    // DISCUSS!!
    // fn read_str(bytes_str: &[u8], start: &mut usize, chars: usize) -> &str {
    //     let end = *start + chars;
    //     let str = str::from_utf8(&bytes_str[*start..end]).unwrap();
    //     *start += chars;
    //     return str;
    // }

    fn update_res(acc_val: u64, new_val: u64, p_type: u32) -> u64 {
        if acc_val == u64::MAX {
            return new_val;
        }
        match p_type {
            0 => acc_val + new_val,
            1 => acc_val * new_val,
            2 => {
                if acc_val <= new_val {
                    acc_val
                } else {
                    new_val
                }
            } // min
            3 => {
                if acc_val >= new_val {
                    acc_val
                } else {
                    new_val
                }
            } // max
            5 => {
                if acc_val > new_val {
                    1
                } else {
                    0
                }
            } // greater than
            6 => {
                if acc_val < new_val {
                    1
                } else {
                    0
                }
            } // less than
            7 => {
                if acc_val == new_val {
                    1
                } else {
                    0
                }
            } // equal to
            _ => panic!("not correct type"),
        }
    }

    pub fn run(&mut self, bs: &[u8], bs_start: usize) -> (usize, u64) {
        self.depth += 1;
        println!("-- PacketParser depth {} --", self.depth);
        let mut pos: usize = bs_start;
        let p_ver = PParser::read_int(bs, &mut pos, 3);
        self.ver_sum += p_ver;
        let p_type = PParser::read_int(bs, &mut pos, 3);

        if p_type == 4 {
            let mut accum_val: String = "".to_string();
            loop {
                let first_bit = PParser::read_int(bs, &mut pos, 1);
                accum_val += str::from_utf8(&bs[pos..pos + 4]).unwrap();
                PParser::read_move(&mut pos, 4);
                if first_bit == 0 {
                    break;
                }
            }
            let val = u64::from_str_radix(&accum_val, 2).unwrap();
            self.depth -= 1;
            return (pos, val);
        } else {
            let length_id = PParser::read_int(bs, &mut pos, 1);
            if length_id == 0 {
                let bits_in_sub_packets: usize =
                    PParser::read_int(bs, &mut pos, 15).try_into().unwrap();
                let end_pos = pos + bits_in_sub_packets;
                let mut cur_val = u64::MAX;
                let mut prev_val: u64;
                loop {
                    prev_val = cur_val;
                    (pos, cur_val) = PParser::run(self, bs, pos);
                    cur_val = PParser::update_res(prev_val, cur_val, p_type);
                    if pos >= end_pos {
                        break;
                    }
                }
                self.depth -= 1;
                return (end_pos, cur_val);
            } else {
                let number_of_sub_packets = PParser::read_int(bs, &mut pos, 11);
                let mut cur_val = u64::MAX;
                let mut prev_val: u64;
                for _ in 0..number_of_sub_packets {
                    prev_val = cur_val;
                    (pos, cur_val) = PParser::run(self, bs, pos);
                    cur_val = PParser::update_res(prev_val, cur_val, p_type);
                }
                self.depth -= 1;
                return (pos, cur_val);
            }
        }
    }
}

fn task(file_name: &str) -> (u32, u64) {
    let content = fs::read_to_string(file_name).expect("reading file error");
    let str = content.lines().next().unwrap();
    // maybe string of bytes b""?
    let mut b_str = "".to_string();
    for c in str.chars() {
        let a = i64::from_str_radix(&c.to_string(), 16).unwrap();
        let b = format!("{a:b}");
        let mut b_str2 = "".to_string();
        let add_zeroes = 4 - b.chars().count();
        for _a in 0..add_zeroes {
            b_str2 += "0";
        }
        b_str2 += &b;
        b_str += &b_str2;
    }

    let mut parser = PParser {
        ver_sum: 0,
        depth: 0,
    };
    let bs = b_str.as_bytes();
    let (_, end_val) = parser.run(bs, 0);
    println!("ver_sum {}", parser.ver_sum);
    println!("end_val {}", end_val);
    return (parser.ver_sum, end_val);
}

fn main() {
    task("src/data-real.txt");
}

#[cfg(test)]
mod test;
