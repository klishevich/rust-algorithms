use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs;

fn main() {
    // HASHMAP
    let mut routes_map: HashMap<&str, Vec<&str>> = HashMap::new();

    let content = fs::read_to_string("src/data-test2.txt").expect("we have a bug");
    for line in content.lines() {
        // println!("new line");
        let (left_str, right_str) = line.split_once("-").unwrap();
        if left_str != "end" && right_str != "start" {
            let routes_to_opt = routes_map.get_mut(left_str);
            match routes_to_opt {
                Some(routes_to) => routes_to.push(right_str),
                None => {
                    routes_map.insert(left_str, vec![right_str]);
                }
            };
        }
        if right_str != "end" && left_str != "start" {
            let routes_to_reversed_opt = routes_map.get_mut(right_str);
            match routes_to_reversed_opt {
                Some(routes_to) => routes_to.push(left_str),
                None => {
                    routes_map.insert(right_str, vec![left_str]);
                }
            };
        }
    }

    // for (key, value) in &routes_map {
    //     print!("{} : ", key);
    //     println!("{:?}", value);
    // }

    let mut bfs_next_id: i32 = 0;
    // let mut bfs_paths: HashMap<i32, Vec<&str>> = HashMap::new();
    // let mut bfs_visited: HashMap<i32, HashMap<&str, bool>> = HashMap::new();
    let mut res_paths: Vec<Vec<&str>> = Vec::new();

    // VEC DEQUE
    // (id, path, visited)
    let mut queue: VecDeque<(i32, Vec<&str>, HashMap<&str, bool>)> = VecDeque::new();

    let start_routes_to = routes_map.get("start").unwrap();
    for el in start_routes_to {
        let mut visited_hash_map = HashMap::new();
        if el.chars().next().unwrap().is_lowercase() {
            visited_hash_map.insert(*el, true);
        }
        // add element to queue
        queue.push_back((bfs_next_id, vec!["start", *el], visited_hash_map));
        bfs_next_id += 1;
    }

    while queue.len() > 0 {
        let (cur_id, cur_path_vec, cur_visited_map) = queue.pop_front().unwrap();
        let cur_str = cur_path_vec.last().unwrap();
        let routes_to = routes_map.get(cur_str).unwrap();
        for e in routes_to {
            let is_visited = cur_visited_map.get(e);
            match is_visited {
                // Do nothing if visited
                Some(_v) => (),
                None => {
                    let mut cur_path_clone_vec: std::vec::Vec<&str> = cur_path_vec.clone();
                    cur_path_clone_vec.push(*e);
                    if *e == "end" {
                        res_paths.push(cur_path_clone_vec);
                    } else {
                        let mut cur_visited_clone_map = cur_visited_map.clone();
                        if e.chars().next().unwrap().is_lowercase() {
                            cur_visited_clone_map.insert(*e, true);
                        }
                        queue.push_back((bfs_next_id, cur_path_clone_vec, cur_visited_clone_map));
                        bfs_next_id += 1;
                    }
                }
            }
            // TERNARY
            // let id = if index == 0 { cur_id } else { bfs_next_id };
        }
    }

    println!("res_paths {}", res_paths.len());
    for res in &res_paths {
        println!("{:?}", res);
    }
}
