use std::fs;
use std::i64;
use std::str;

struct PacketParser {
    length: usize
}

// RECURSION
impl PacketParser {
    pub fn run(&mut self, s: &str) {
        let bytes_str = s.as_bytes();
        let ver_str = str::from_utf8(&bytes_str[0..3]).unwrap();
        println!("version {}", ver_str);
        let type_str = str::from_utf8(&bytes_str[3..6]).unwrap();
        println!("type_str {}", type_str);
        let is_literal_type = type_str == "100";
        if is_literal_type {
            panic!("add use case for literal type");
        } else {
            let length_id = str::from_utf8(&bytes_str[6..7]).unwrap();
            println!("length_id {}", length_id);
            if length_id == "0" {
                println!("15 bit length");
            } else {
                println!("11 bit length");
                let length = str::from_utf8(&bytes_str[7..7 + 11]).unwrap();
                println!("length {}", length);
                let length_int = isize::from_str_radix(length, 2).unwrap();
                println!("length_int {}", length_int);
            }
        }
    }
}

fn main() {
    let content = fs::read_to_string("src/data-test1.txt").expect("reading file error");
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
        println!("b_str2 {}", b_str2);
        b_str += &b_str2;
    }
    println!("{}", b_str);

    let mut parser = PacketParser {
        length: 0
    };
    parser.run(&b_str);
    // for c in b_str.chars() {

    // }
}
