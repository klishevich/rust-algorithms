use std::fs;
use std::i64;

fn main() {
    let content = fs::read_to_string("src/data-test1.txt").expect("reading file error");
    let str = content.lines().next().unwrap();
    println!("str {}", str);
    let mut b_str = "".to_string();
    for c in str.chars() {
        let a = i64::from_str_radix(&c.to_string(), 16).unwrap();
        let b = format!("{a:b}");
        println!("b {}", b);
        b_str += &b;
    }
    println!("{}", b_str);
}
