use std::fs;
use std::i64;
use std::str;

struct PacketParser {
    ver_sum: usize,
}

// RECURSION
impl PacketParser {
    pub fn run(&mut self, s: &str) -> usize {
        println!("-- PacketParser --");
        println!("str {}", s);
        let bytes_str = s.as_bytes();
        let ver_str = str::from_utf8(&bytes_str[0..3]).unwrap();
        // PARSE BOOL STRING
        let ver = usize::from_str_radix(ver_str, 2).unwrap();
        println!("ver {}", ver);
        self.ver_sum += ver;
        let type_str = str::from_utf8(&bytes_str[3..6]).unwrap();
        println!("type_str {}", type_str);
        let is_literal_type = type_str == "100";
        if is_literal_type {
            println!("it is literal type");
            return 0;
        } else {
            let length_id = str::from_utf8(&bytes_str[6..7]).unwrap();
            if length_id == "0" {
                println!("length_id 0 (15 bit length) total_length");
                let end_pos = 7 + 15;
                let total_length_str = str::from_utf8(&bytes_str[7..end_pos]).unwrap();
                // println!("total_length {}", total_length_str);
                let total_length = usize::from_str_radix(total_length_str, 2).unwrap();
                println!("total_length {}", total_length);
                let left_str = str::from_utf8(&bytes_str[end_pos..s.len()]).unwrap();
                println!("left_str {}", left_str);
                let sub_str = str::from_utf8(&bytes_str[end_pos..end_pos + total_length]).unwrap();
                PacketParser::run(self, sub_str);
                return 0;
            } else {
                println!("length_id 1 (11 bit length) sub_packets");
                let end_pos = 7 + 11;
                let sub_packets_str = str::from_utf8(&bytes_str[7..end_pos]).unwrap();
                // println!("sub_packets_str {}", sub_packets_str);
                let sub_packets = usize::from_str_radix(sub_packets_str, 2).unwrap();
                println!("sub_packets {}", sub_packets);
                let left_str = str::from_utf8(&bytes_str[end_pos..s.len()]).unwrap();
                println!("left_str {}", left_str);
                let sub_str = str::from_utf8(&bytes_str[end_pos..s.len()]).unwrap();
                PacketParser::run(self, sub_str);
                // for loop ...
                return 0;
            }
        }
    }
}

fn main() {
    let content = fs::read_to_string("src/data-test2.txt").expect("reading file error");
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

    let mut parser = PacketParser { ver_sum: 0 };
    parser.run(&b_str);
    println!("ver_sum {}", parser.ver_sum);
    // for c in b_str.chars() {

    // }
}
