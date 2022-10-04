use std::fs;
use std::collections::HashMap;

fn main() {
    let content = fs::read_to_string("src/data01.txt").expect("some bug");
    let mut scanner_id: i32 = -1;
    let mut scanner_map: HashMap<i32, Vec<(i32,i32)>> = HashMap::new();
    for (index, line) in content.lines().enumerate() {
        println!("index {}, line {}", index, line);
        if line.contains("scanner") {
            scanner_id += 1;
            scanner_map.insert(scanner_id, Vec::new());
        } else if line.is_empty() {
            continue;
        } else {
            let (left_str, right_str) = line.split_once(",").unwrap();
            let left_i32: i32 = left_str.parse().unwrap();
            let right_i32: i32 = right_str.parse().unwrap();
            scanner_map.get_mut(&scanner_id).unwrap().push((left_i32, right_i32));
        }
    }

    for (k,v) in scanner_map {
        println!("key {}", k);
        println!("{:?}", v);
    }
}
