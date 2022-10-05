use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

type Point = (i32, i32, i32);

/**
 * Permutation value could be from 0 to 47
 */
fn get_permuted_point(p: Point, permutation: u8) -> Point {
    let p1: u8 = permutation % 6;
    println!("p1 {}", p1);
    let r1: u8 = permutation / 6;
    let p2: u8 = r1 % 4;
    println!("p2 {}", p2);
    let p3: u8 = r1 / 4;
    println!("p3 {}", p3);
    let mut x1: u8;
    let mut y1: u8;
    let mut z1: u8;

    let (x, y, z) = p;
    return (x, y, z);
}

fn main() {
    let content = fs::read_to_string("src/data01.txt").expect("some bug");
    let mut scanner_id: i32 = -1;
    let mut scanner_map: HashMap<i32, Vec<Point>> = HashMap::new();
    for (index, line) in content.lines().enumerate() {
        println!("index {}, line {}", index, line);
        if line.contains("scanner") {
            scanner_id += 1;
            scanner_map.insert(scanner_id, Vec::new());
        } else if line.is_empty() {
            continue;
        } else {
            let strs: Vec<&str> = line.split(",").collect();
            let first: i32 = strs[0].parse().unwrap();
            let second: i32 = strs[1].parse().unwrap();
            let third: i32 = strs[2].parse().unwrap();
            scanner_map
                .get_mut(&scanner_id)
                .unwrap()
                .push((first, second, third));
        }
    }

    // MAP SORT HASHMAP
    // for key in scanner_map.keys().sorted() {
    //     println!("{}", key);
    //     println!("{:?}", scanner_map[key]);
    // }

    let r = get_permuted_point((1,2,3), 47);
    println!("{:?}", r);
}
