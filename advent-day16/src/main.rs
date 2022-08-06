use std::fs;
use std::i64;
use std::str;

struct PacketParser {
    ver_sum: u32,
    depth: u32,
}

// RECURSION
impl PacketParser {
    // REFERENCE TO MUTABLE INT
    fn read_int(bytes_str: &[u8], start: &mut usize, chars: usize) -> u32 {
        let end = *start + chars;
        let str = str::from_utf8(&bytes_str[*start..end]).unwrap();
        let val = u32::from_str_radix(str, 2).unwrap();
        *start += chars;
        return val;
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

    pub fn run(&mut self, s: &str) -> (usize, u64) {
        self.depth += 1;
        println!("-- PacketParser depth {} --", self.depth);
        let mut pos: usize = 0;
        let bs = s.as_bytes();
        let p_ver = PacketParser::read_int(bs, &mut pos, 3);
        self.ver_sum += p_ver;
        let p_type = PacketParser::read_int(bs, &mut pos, 3);

        if p_type == 4 {
            let mut accum_val: String = "".to_string();
            loop {
                let first_bit = PacketParser::read_int(bs, &mut pos, 1);
                accum_val += str::from_utf8(&bs[pos..pos + 4]).unwrap();
                PacketParser::read_move(&mut pos, 4);
                if first_bit == 0 {
                    break;
                }
            }

            let val = u64::from_str_radix(&accum_val, 2).unwrap();
            self.depth -= 1;
            return (pos, val);
        } else {
            let length_id = PacketParser::read_int(bs, &mut pos, 1);
            if length_id == 0 {
                let bits_in_sub_packets: usize =
                    PacketParser::read_int(bs, &mut pos, 15).try_into().unwrap();
                let end_pos = pos + bits_in_sub_packets;
                let mut end_acc_val = u64::MAX;
                loop {
                    let sub_str = str::from_utf8(&bs[pos..s.len()]).unwrap();
                    let (r_pos, r_val) = PacketParser::run(self, sub_str);
                    pos = pos + r_pos;
                    if end_acc_val == u64::MAX {
                        end_acc_val = r_val
                    } else {
                        end_acc_val = PacketParser::update_res(end_acc_val, r_val, p_type);
                    }
                    if pos >= end_pos {
                        break;
                    }
                }
                self.depth -= 1;
                return (end_pos, end_acc_val);
            } else {
                let number_of_sub_packets = PacketParser::read_int(bs, &mut pos, 11);
                let mut end_acc_val = u64::MAX;
                for _ in 0..number_of_sub_packets {
                    let sub_str = str::from_utf8(&bs[pos..s.len()]).unwrap();
                    let (r_pos, r_val) = PacketParser::run(self, sub_str);
                    pos = pos + r_pos;
                    if end_acc_val == u64::MAX {
                        end_acc_val = r_val
                    } else {
                        end_acc_val = PacketParser::update_res(end_acc_val, r_val, p_type);
                    }
                }
                self.depth -= 1;
                return (pos, end_acc_val);
            }
        }
    }
}

fn task(file_name: &str) -> (u32, u64) {
    let content = fs::read_to_string(file_name).expect("reading file error");
    let str = content.lines().next().unwrap();
    println!("str {}", str);
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

    let mut parser = PacketParser {
        ver_sum: 0,
        depth: 0,
    };
    let (_, end_val) = parser.run(&b_str);
    println!("ver_sum {}", parser.ver_sum);
    println!("end_val {}", end_val);
    return (parser.ver_sum, end_val);
}

fn main() {
    task("src/data-real.txt");
}

#[cfg(test)]
mod test;
