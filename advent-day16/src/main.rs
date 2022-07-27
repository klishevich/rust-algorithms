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
        let version = &bytes_str[0..3];
        match str::from_utf8(version) {
            Ok(v) => {
                println!("version {:?}", v);
            },
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
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
        println!("b {}", b);
        b_str += &b;
    }
    println!("{}", b_str);

    let mut parser = PacketParser {
        length: 0
    };
    parser.run(&b_str);
    // for c in b_str.chars() {

    // }
}
