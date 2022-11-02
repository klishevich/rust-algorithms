use core::num;
use itertools::Itertools;
use std::char::MAX;
use std::collections::HashMap;
use std::fs;

type Point = (i32, i32, i32);

struct EdgeInfo {
    node_id: u8,
    from_origin: Point,
    to_origin: Point,
    permutation: u8,
    is_reverse_permutation: bool,
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
fn get_reverse_permuted_point(
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

fn point_to_origin_reverse(p: Point, origin: Point) -> Point {
    return (p.0 + origin.0, p.1 + origin.1, p.2 + origin.2);
}

fn points_equal(p1: Point, p2: Point) -> bool {
    if p1.0 == p2.0 && p1.1 == p2.1 && p1.2 == p2.2 {
        return true;
    }
    return false;
}

fn create_point_hash(p: &Point) -> String {
    let mut res: String = p.0.to_string();
    let space: String = "_".to_owned();
    res.push_str(&space);
    res.push_str(&p.1.to_string());
    res.push_str(&space);
    res.push_str(&p.2.to_string());
    return res;
}

fn create_scanner_map_from_file() -> HashMap<u8, Vec<Point>> {
    let content = fs::read_to_string("src/data01.txt").expect("some bug");
    let mut scanner_cnt: u8 = 0;
    let mut scanner_map: HashMap<u8, Vec<Point>> = HashMap::new();
    // CREATE SCANNER MAP FROM FILE
    for (_index, line) in content.lines().enumerate() {
        if line.contains("scanner") {
            scanner_cnt += 1;
            scanner_map.insert(scanner_cnt - 1, Vec::new());
        } else if line.is_empty() {
            continue;
        } else {
            let strs: Vec<&str> = line.split(",").collect();
            let first: i32 = strs[0].parse().unwrap();
            let second: i32 = strs[1].parse().unwrap();
            let third: i32 = strs[2].parse().unwrap();
            scanner_map
                .get_mut(&(scanner_cnt - 1))
                .unwrap()
                .push((first, second, third));
        }
    }
    return scanner_map;
}

fn fill_adjacency_map(scanner_map: &HashMap<u8, Vec<Point>>) -> HashMap<u8, HashMap<u8, EdgeInfo>> {
    let mut adjacency_map: HashMap<u8, HashMap<u8, EdgeInfo>> = HashMap::new();
    for key1 in scanner_map.keys().sorted() {
        for key2 in scanner_map.keys().sorted() {
            if *key1 < *key2 {
                let beacon_list1 = scanner_map.get(key1).unwrap();
                let beacon_list2 = scanner_map.get(key2).unwrap();
                let mut origin_found = false;
                for beacon_list2_origin in beacon_list2 { //b
                    if origin_found {
                        break;
                    }
                    for permutation in 0..48 {  // 48
                        let (change_sign_x, change_sign_y, change_sign_z, swap_yz, shift_all) =
                            decompose_number(permutation);
                        for beacon_list1_origin in beacon_list1 { //b
                            if origin_found {
                                break;
                            }
                            let mut number_of_matches = 0;
                            for beacon2 in beacon_list2 { // b
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
                                for beacon1 in beacon_list1 { //b
                                    let beacon1_new_origin =
                                        point_to_origin(*beacon1, *beacon_list1_origin);
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
                                    permutation: permutation,
                                };
                                adjacency_map
                                    .entry(*key1)
                                    .or_default()
                                    .insert(*key2, key1_key2_edge_info);
                                let key2_key1_edge_info = EdgeInfo {
                                    node_id: *key1,
                                    from_origin: *beacon_list2_origin,
                                    to_origin: *beacon_list1_origin,
                                    is_reverse_permutation: false,
                                    permutation: permutation,
                                };
                                adjacency_map
                                    .entry(*key2)
                                    .or_default()
                                    .insert(*key1, key2_key1_edge_info);
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
    return adjacency_map;
}

/**
 * Prim’s Minimum Spanning Tree (MST) | Greedy Algo-5
 * https://www.geeksforgeeks.org/prims-minimum-spanning-tree-mst-greedy-algo-5/
 */
fn prim_mst(adj_map: &HashMap<u8, HashMap<u8, EdgeInfo>>, scanner_cnt: u8) -> Vec<Vec<u8>> {
    // let mut included = vec![false; scanner_cnt as usize];
    let mut routes_res: Vec<Vec<u8>> = vec![];
    let max_val: u8 = 255;
    let cost = 1;
    let mut parent: Vec<u8> = vec![max_val; scanner_cnt as usize];
    let mut key: Vec<u8> = vec![max_val; scanner_cnt as usize];
    let mut mst_set: Vec<bool> = vec![false; scanner_cnt as usize];

    key[0] = 0;
    parent[0] = max_val;

    // Force me to use pure functions
    // Does not work if I use mst_set and key from the closure
    let min_key_fn = |mst_set2: &Vec<bool>, key2: &Vec<u8>| -> u8 {
        let mut min = max_val;
        let mut min_index = max_val;
        for v in 0..scanner_cnt {
            if mst_set2[v as usize] == false && key2[v as usize] < min {
                min = key2[v as usize];
                min_index = v;
            }
        }
        return min_index;
    };

    for _i in 0..scanner_cnt {
        let u = min_key_fn(&mst_set, &key);
        mst_set[u as usize] = true;

        let cur_node_paths = adj_map.get(&u).unwrap();

        for (_j, v) in cur_node_paths {
            let node_id = v.node_id as usize;
            if mst_set[node_id] == false && cost < key[node_id] {
                parent[node_id] = u;
                key[node_id] = cost;
            }
        }
    }

    println!("--prim_mst res--");
    println!("parent {:?}", parent);
    println!("key {:?}", key);
    println!("mst_set {:?}", mst_set);

    for i in 0..scanner_cnt {
        let mut r: Vec<u8> = Vec::new();
        let mut cur_node = i;
        for j in 0..scanner_cnt {
            r.push(cur_node);
            cur_node = parent[cur_node as usize];
            if cur_node == max_val {
                break;
            }
        }
        routes_res.push(r)
    }

    return routes_res;
}

fn get_resulting_map(
    routes_vec: &Vec<Vec<u8>>,
    adjacency_map: &HashMap<u8, HashMap<u8, EdgeInfo>>,
    scanner_map: &HashMap<u8, Vec<Point>>,
) -> HashMap<String, Point> {
    let mut res_map: HashMap<String, Point> = HashMap::new();
    // GET RESULTING BEACONS MAP
    for route in routes_vec {
        let scanner_id = route[0];
        let beacons = scanner_map.get(&scanner_id).unwrap();
        for beacon in beacons {
            let mut b: Point = (beacon.0, beacon.1, beacon.2);
            for i in 0..route.len() - 1 {
                let node_id_from = route[i];
                let node_id_to = route[i + 1];
                let edge_info_map = adjacency_map.get(&node_id_from).unwrap();
                let edge_info = edge_info_map.get(&node_id_to).unwrap();
                // new origin
                b = point_to_origin(b, edge_info.from_origin);
                // permuted point
                let (change_sign_x, change_sign_y, change_sign_z, swap_yz, shift_all) =
                    decompose_number(edge_info.permutation);
                if edge_info.is_reverse_permutation {
                    b = get_reverse_permuted_point(
                        b,
                        change_sign_x,
                        change_sign_y,
                        change_sign_z,
                        swap_yz,
                        shift_all,
                    )
                } else {
                    b = get_permuted_point(
                        b,
                        change_sign_x,
                        change_sign_y,
                        change_sign_z,
                        swap_yz,
                        shift_all,
                    )
                }
                b = point_to_origin_reverse(b, edge_info.to_origin);
            }
            res_map.entry(create_point_hash(&b)).or_insert(b);
        }
    }
    return res_map;
}

/**
 * Calculates distance using a taxicab geometry or a Manhattan geometry
 */
fn calc_taxicab_distance(p1: &Point, p2: &Point) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs() + (p1.2 - p2.2).abs()
}

fn main() {
    let scanner_map: HashMap<u8, Vec<Point>> = create_scanner_map_from_file();
    let scanner_cnt: u8 = scanner_map.len() as u8;
    // PRINT OUT scanner_map
    println!("--scanner_map--");
    for key in scanner_map.keys().sorted() {
        println!("{}", key);
        println!("{:?}", scanner_map[key]);
    }

    let adjacency_map = fill_adjacency_map(&scanner_map);
    // PRINT OUT adjacency_map
    println!("--adjacency_map--");
    for key in adjacency_map.keys().sorted() {
        println!(" {} ->", key);
        let val_map = adjacency_map.get(key).unwrap();
        for key2 in val_map.keys().sorted() {
            let ei = val_map.get(key2).unwrap();
            println!(
                "  {}: is_rev {}",
                key2, ei.is_reverse_permutation
            );
        }
    }

    let routes_vec = prim_mst(&adjacency_map, scanner_cnt);
    // PRINT OUT routes_vec
    println!("--routes_vec--");
    println!("{:?}", routes_vec);

    // PART 1
    let res_beacons_map = get_resulting_map(&routes_vec, &adjacency_map, &scanner_map);
    // PRINT OUT res_beacons_map
    println!("--res_beacons_map--");
    for key in res_beacons_map.keys().sorted() {
        let val = res_beacons_map.get(key).unwrap();
        println!("key {}, {} {} {}", key, val.0, val.1, val.2);
    }
    println!("beacons count {}", res_beacons_map.len());

    // PART 2
    let mut scanner_only_map: HashMap<u8, Vec<Point>> = HashMap::new();
    for i in 0..scanner_cnt {
        scanner_only_map.insert(i, vec![(0, 0, 0)]);
    }
    let res_scanner_map = get_resulting_map(&routes_vec, &adjacency_map, &scanner_only_map);
    // PRINT OUT res_beacons_map
    println!("--res_scanner_map--");
    for key in res_scanner_map.keys().sorted() {
        let val = res_scanner_map.get(key).unwrap();
        println!("key {}, {} {} {}", key, val.0, val.1, val.2);
    }

    let mut max_taxicab_distance = 0;
    for (_i, p1) in &res_scanner_map {
        for (_j, p2) in &res_scanner_map {
            if calc_taxicab_distance(p1, p2) > max_taxicab_distance {
                max_taxicab_distance = calc_taxicab_distance(p1, p2)
            }
        }
    }
    println!("max_taxicab_distance {}", max_taxicab_distance);
}

mod test {
    use crate::decompose_number;
    use crate::get_permuted_point;
    use crate::get_reverse_permuted_point;

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
        let res_rev = get_reverse_permuted_point(
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
        let res_rev = get_reverse_permuted_point(
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
        let res_rev = get_reverse_permuted_point(
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
        let res_rev = get_reverse_permuted_point(
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
