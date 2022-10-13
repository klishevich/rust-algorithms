use core::num;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

type Point = (i32, i32, i32);

struct EdgeInfo {
    node_id: u8,
    from_origin: Point,
    to_origin: Point,
    permutation: u8,
    is_reverse_permutation: bool
}

struct ScannerDfs {
    visited: HashMap<u8, bool>,
    result: HashMap<String,Point>
}

// RECURSION
impl ScannerDfs {
    pub fn run(&mut self, m: &HashMap<u8, Vec<EdgeInfo>>, scanners: u8, cur_node: u8) -> () {

    }
}

/**
 * The shift_all should accept one of the following value 0,1,2
 */
fn get_permuted_point(
    p: Point,
    change_sign_x: bool,
    change_sign_y: bool,
    change_sign_z: bool,
    swap_yz: bool,
    shift_all: u8,
) -> Point {
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

/**
 * The shift_all should accept one of the following value 0,1,2
 */
fn get_inverse_permuted_point(
    p: Point,
    change_sign_x: bool,
    change_sign_y: bool,
    change_sign_z: bool,
    swap_yz: bool,
    shift_all: u8,
) -> Point {
    let (mut x, mut y, mut z) = p;
    let shift = shift_all % 3;
    if shift == 2 {
        let tmp_x = x;
        x = z;
        z = y;
        y = tmp_x;
    } else if shift == 1 {
        let tmp_x = x;
        x = y;
        y = z;
        z = tmp_x;
    }

    if swap_yz {
        let tmp = y;
        y = z;
        z = tmp;
    }

    if change_sign_x {
        x = -x;
    }
    if change_sign_y {
        y = -y;
    }
    if change_sign_z {
        z = -z;
    }
    return (x, y, z);
}

/**
 * The num should accept value within range [0, 47]
 */
fn decompose_number(num: u8) -> (bool, bool, bool, bool, u8) {
    let change_sign_x = num % 2 == 1;
    let mut residue = num / 2;
    let change_sign_y = residue % 2 == 1;
    residue = residue / 2;
    let change_sign_z = residue % 2 == 1;
    residue = residue / 2;
    let swap_yz = residue % 2 == 1;
    let shift_all = residue / 2;
    return (
        change_sign_x,
        change_sign_y,
        change_sign_z,
        swap_yz,
        shift_all,
    );
}

fn point_to_origin(p: Point, origin: Point) -> Point {
    return (p.0 - origin.0, p.1 - origin.1, p.2 - origin.2);
}

fn points_equal(p1: Point, p2: Point) -> bool {
    if p1.0 == p2.0 && p1.1 == p2.1 && p1.2 == p2.2 {
        return true;
    }
    return false;
}

fn main() {
    let content = fs::read_to_string("src/data01.txt").expect("some bug");
    let mut scanner_id: u8 = 0;
    let mut scanner_map: HashMap<u8, Vec<Point>> = HashMap::new();
    for (index, line) in content.lines().enumerate() {
        if line.contains("scanner") {
            scanner_id += 1;
            scanner_map.insert(scanner_id - 1, Vec::new());
        } else if line.is_empty() {
            continue;
        } else {
            let strs: Vec<&str> = line.split(",").collect();
            let first: i32 = strs[0].parse().unwrap();
            let second: i32 = strs[1].parse().unwrap();
            let third: i32 = strs[2].parse().unwrap();
            scanner_map
                .get_mut(&(scanner_id-1))
                .unwrap()
                .push((first, second, third));
        }
    }

    // MAP SORT HASHMAP
    // for key in scanner_map.keys().sorted() {
    //     println!("{}", key);
    //     println!("{:?}", scanner_map[key]);
    // }

    let mut adjacency_map: HashMap<u8, Vec<EdgeInfo>> = HashMap::new();

    for key1 in scanner_map.keys().sorted() {
        for key2 in scanner_map.keys().sorted() {
            if *key1 < *key2 {
                let beacon_list1 = scanner_map.get(key1).unwrap();
                let beacon_list2 = scanner_map.get(key2).unwrap();
                let mut origin_found = false;
                for beacon_list1_origin in beacon_list1 {
                    if origin_found {
                        break;
                    }
                    for beacon_list2_origin in beacon_list2 {
                        if origin_found {
                            break;
                        }
                        for permutation in 0..48 {
                            let (change_sign_x, change_sign_y, change_sign_z, swap_yz, shift_all) =
                                decompose_number(permutation);
                            let mut number_of_matches = 0;
                            for beacon1 in beacon_list1 {
                                for beacon2 in beacon_list2 {
                                    let beacon1_new_origin =
                                        point_to_origin(*beacon1, *beacon_list1_origin);
                                    let beacon2_new_origin =
                                        point_to_origin(*beacon2, *beacon_list2_origin);
                                    // println!("test, beacon_list1_origin {:?}, beacon_list2_origin {:?}", beacon_list1_origin, beacon_list2_origin);
                                    let beacon2_new_origin_perm = get_permuted_point(
                                        beacon2_new_origin,
                                        change_sign_x,
                                        change_sign_y,
                                        change_sign_z,
                                        swap_yz,
                                        shift_all,
                                    );
                                    // println!("test, beacon_list1_origin {:?}, beacon2_new_origin_perm {:?}", beacon_list1_origin, beacon2_new_origin_perm);
                                    if points_equal(beacon1_new_origin, beacon2_new_origin_perm) {
                                        // println!("equal, beacon_list1_origin {:?}, beacon_list2_origin {:?}", beacon_list1_origin, beacon_list2_origin);
                                        number_of_matches += 1;
                                    }
                                }
                            }
                            if number_of_matches >= 12 {
                                // println!("   {} <-> {}", key1, key2);
                                // println!("!!! number_of_matches {}, beacon_list1_origin {:?}, beacon_list2_origin {:?}", number_of_matches, beacon_list1_origin, beacon_list2_origin);
                                // println!("change_sign_x {}, change_sign_y {}, change_sign_z {}, swap_yz {}, shift_all {}", change_sign_x,
                                // change_sign_y,
                                // change_sign_z,
                                // swap_yz,
                                // shift_all);
                                origin_found = true;
                                let key1_key2_edge_info = EdgeInfo {
                                    node_id: *key2,
                                    from_origin: *beacon_list1_origin,
                                    to_origin: *beacon_list2_origin,
                                    is_reverse_permutation: true,
                                    permutation: permutation
                                };
                                adjacency_map.entry(*key1).or_default().push(key1_key2_edge_info);
                                let key2_key1_edge_info = EdgeInfo {
                                    node_id: *key1,
                                    from_origin: *beacon_list2_origin,
                                    to_origin: *beacon_list1_origin,
                                    is_reverse_permutation: false,
                                    permutation: permutation
                                };
                                adjacency_map.entry(*key2).or_default().push(key2_key1_edge_info);
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    for key in adjacency_map.keys().sorted() {
        println!("  --{}--", key);
        let val_vec = adjacency_map.get(key).unwrap();
        for ei in val_vec {
            println!("node_id {} is_rev {}", ei.node_id, ei.is_reverse_permutation);
        }
    }

}

mod test {
    use crate::decompose_number;
    use crate::get_inverse_permuted_point;
    use crate::get_permuted_point;

    #[test]
    fn get_permuted_point_test() {
        let res = get_permuted_point((1, 2, 3), true, false, false, false, 1);
        assert_eq!(res, (3, -1, 2));
    }

    #[test]
    fn get_permuted_point_test1() {
        let res = get_permuted_point((1, 2, 3), false, false, false, false, 2);
        assert_eq!(res, (2, 3, 1));
    }

    #[test]
    fn decompose_number_test1() {
        let res = decompose_number(0);
        assert_eq!(res, (false, false, false, false, 0));
    }

    #[test]
    fn decompose_number_test2() {
        let res = decompose_number(47);
        assert_eq!(res, (true, true, true, true, 2));
    }

    #[test]
    fn get_reverse_permuted_point_test1() {
        let change_sign_x = false;
        let change_sign_y = false;
        let change_sign_z = false;
        let swap_yz = false;
        let shift_all: u8 = 1;
        let res = get_permuted_point(
            (1, 2, 3),
            change_sign_x,
            change_sign_y,
            change_sign_z,
            swap_yz,
            shift_all,
        );
        assert_eq!(res, (3, 1, 2));
        let res_rev = get_inverse_permuted_point(
            res,
            change_sign_x,
            change_sign_y,
            change_sign_z,
            swap_yz,
            shift_all,
        );
        assert_eq!(res_rev, (1, 2, 3));
    }

    #[test]
    fn get_reverse_permuted_point_test2() {
        let change_sign_x = false;
        let change_sign_y = false;
        let change_sign_z = false;
        let swap_yz = true;
        let shift_all: u8 = 1;
        let res = get_permuted_point(
            (1, 2, 3),
            change_sign_x,
            change_sign_y,
            change_sign_z,
            swap_yz,
            shift_all,
        );
        assert_eq!(res, (2, 1, 3));
        let res_rev = get_inverse_permuted_point(
            res,
            change_sign_x,
            change_sign_y,
            change_sign_z,
            swap_yz,
            shift_all,
        );
        assert_eq!(res_rev, (1, 2, 3));
    }

    #[test]
    fn get_reverse_permuted_point_test3() {
        let change_sign_x = true;
        let change_sign_y = false;
        let change_sign_z = false;
        let swap_yz = true;
        let shift_all: u8 = 1;
        let res = get_permuted_point(
            (1, 2, 3),
            change_sign_x,
            change_sign_y,
            change_sign_z,
            swap_yz,
            shift_all,
        );
        assert_eq!(res, (2, -1, 3));
        let res_rev = get_inverse_permuted_point(
            res,
            change_sign_x,
            change_sign_y,
            change_sign_z,
            swap_yz,
            shift_all,
        );
        assert_eq!(res_rev, (1, 2, 3));
    }

    #[test]
    fn get_reverse_permuted_point_test4() {
        let (change_sign_x, change_sign_y, change_sign_z, swap_yz, shift_all) =
            decompose_number(35);
        let res = get_permuted_point(
            (1, 2, 3),
            change_sign_x,
            change_sign_y,
            change_sign_z,
            swap_yz,
            shift_all,
        );
        let res_rev = get_inverse_permuted_point(
            res,
            change_sign_x,
            change_sign_y,
            change_sign_z,
            swap_yz,
            shift_all,
        );
        assert_eq!(res_rev, (1, 2, 3));
    }
}
