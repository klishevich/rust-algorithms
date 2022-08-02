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

    pub fn run(&mut self, s: &str) -> usize {
        self.depth += 1;
        println!("-- PacketParser depth {} --", self.depth);
        println!("str {}", s);
        let mut pos: usize = 0;
        let bs = s.as_bytes();
        let p_ver = PacketParser::read_int(bs, &mut pos, 3);
        println!("ver {}", p_ver);
        self.ver_sum += p_ver;
        let p_type = PacketParser::read_int(bs, &mut pos, 3);
        println!("type {}", p_type);

        if p_type == 4 {
            println!("LITERAL");
            // LOOP
            let mut accum_val: String = "".to_string();
            loop {
                let first_bit = PacketParser::read_int(bs, &mut pos, 1);
                accum_val += str::from_utf8(&bs[pos..pos + 4]).unwrap();
                PacketParser::read_move(&mut pos, 4);
                if first_bit == 0 {
                    break;
                }
            }
            println!("literal end pos {}, accum_val {}", pos, accum_val);
            self.depth -= 1;
            return pos;
        } else {
            let length_id = str::from_utf8(&bs[6..7]).unwrap();
            if length_id == "0" {
                // println!("length_id 0 (15 bit length) BITS_IN_SUB_PACKETS");
                let length_end_pos = 7 + 15;
                let bits_in_sub_packets_str = str::from_utf8(&bs[7..length_end_pos]).unwrap();
                let bits_in_sub_packets =
                    usize::from_str_radix(bits_in_sub_packets_str, 2).unwrap();
                println!("BITS_IN_SUB_PACKETS {}", bits_in_sub_packets);
                let left_str = str::from_utf8(&bs[length_end_pos..s.len()]).unwrap();
                println!("lft {}", left_str);
                let mut start_index = length_end_pos;
                println!("start_index {}", start_index);
                let end_index = length_end_pos + bits_in_sub_packets;
                loop {
                    println!("-loop depth {}", self.depth);
                    let sub_str = str::from_utf8(&bs[start_index..end_index]).unwrap();
                    start_index = start_index + PacketParser::run(self, sub_str);
                    // println!("start_index {}", start_index);
                    if start_index >= bits_in_sub_packets {
                        break;
                    }
                }

                // let hardcoded_num = 2;
                // for i in 0..hardcoded_num {
                //     let start = length_end_pos + i*bits_in_sub_packet;
                //     let end = start + bits_in_sub_packet;
                //     let sub_str = str::from_utf8(&bytes_str[start..end]).unwrap();
                //     PacketParser::run(self, sub_str);
                // }
                self.depth -= 1;
                return end_index; // length_end_pos + hardcoded_num * bits_in_sub_packet;
            } else {
                // println!("length_id 1 (11 bit length) NUMBER_OF_SUB_PACKETS");
                let end_pos = 7 + 11;
                let sub_packets_str = str::from_utf8(&bs[7..end_pos]).unwrap();
                // println!("sub_packets_str {}", sub_packets_str);
                let sub_packets = usize::from_str_radix(sub_packets_str, 2).unwrap();
                println!("NUMBER_OF_SUB_PACKETS {}", sub_packets);
                let mut start_pos = end_pos;
                for _ in 0..sub_packets {
                    // TODO: refactor
                    let sub_str = str::from_utf8(&bs[start_pos..s.len()]).unwrap();
                    // println!("lft {}", sub_str);
                    start_pos = PacketParser::run(self, sub_str);
                }
                self.depth -= 1;
                return usize::MAX;
            }
        }
    }
}

fn main() {
    let content = fs::read_to_string("src/data-test01.txt").expect("reading file error");
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
        // println!("b_str2 {}", b_str2);
        b_str += &b_str2;
    }

    let mut parser = PacketParser {
        ver_sum: 0,
        depth: 0,
    };
    parser.run(&b_str);
    println!("ver_sum {}", parser.ver_sum);
}
