use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

type Point = (i32, i32, i32);

fn get_permuted_point(p: Point, change_sign_x: bool, change_sign_y: bool, change_sign_z: bool, swap_yz: bool, shift_all: u8) -> Point {
    let (mut x, mut y, mut z) = p;
    if change_sign_x {
        x = -x;
    }
    if change_sign_y {
        y = -y;
    }
    if change_sign_z {
        z = -z;
    }
    if swap_yz {
        let tmp = y;
        y = z;
        z = tmp;
    }
    let shift = shift_all % 3;
    if shift == 1 {
        let tmp_x = x;
        x = z;
        z = y;
        y = tmp_x;
    } else if shift == 2 {
        let tmp_x = x;
        x = y;
        y = z;
        z = tmp_x;
    }
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

    let r = get_permuted_point((1,2,3), true, false, false, false, 1);
    println!("{:?}", r);
}

mod test {
    use crate::get_permuted_point;

    #[test]
    fn get_permuted_point_test() {
        let res = get_permuted_point((1,2,3), true, false, false, false, 1);
        assert_eq!(res, (3, -1, 2));
    }

    #[test]
    fn get_permuted_point_test1() {
        let res = get_permuted_point((1,2,3), false, false, false, false, 2);
        assert_eq!(res, (2, 3, 1));
    }
}