use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs;

fn main() {
    // HASHMAP
    let mut routes_map: HashMap<&str, Vec<&str>> = HashMap::new();

    let content = fs::read_to_string("src/data-test1.txt").expect("we have a bug");
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
    let mut res_paths: Vec<Vec<&str>> = Vec::new();

    // VEC DEQUE
    // (id, path, visited, visited_twice)
    let mut queue: VecDeque<(i32, Vec<&str>, HashMap<&str, i32>, bool)> = VecDeque::new();

    let start_routes_to = routes_map.get("start").unwrap();
    for el in start_routes_to {
        let mut visited_hash_map: HashMap<&str, i32> = HashMap::new();
        if el.chars().next().unwrap().is_lowercase() {
            visited_hash_map.insert(*el, 1);
        }
        // add element to queue
        queue.push_back((bfs_next_id, vec!["start", *el], visited_hash_map, false));
        bfs_next_id += 1;
    }

    while queue.len() > 0 {
        let (_cur_id, cur_path_vec, cur_visited_map, cur_visited_twice) = queue.pop_front().unwrap();
        let cur_str = cur_path_vec.last().unwrap();
        let routes_to = routes_map.get(cur_str).unwrap();
        for e in routes_to {
            let mut visited_times = 0;
            let visited_times_option = cur_visited_map.get(e);
            match visited_times_option {
                Some(visited_times2) => visited_times = *visited_times2,
                None => ()
            }
            if visited_times < 1 || (!cur_visited_twice && visited_times == 1) {
                let mut cur_path_clone_vec: std::vec::Vec<&str> = cur_path_vec.clone();
                cur_path_clone_vec.push(*e);
                if *e == "end" {
                    res_paths.push(cur_path_clone_vec);
                } else {
                    let mut cur_visited_clone_map = cur_visited_map.clone();
                    if e.chars().next().unwrap().is_lowercase() {
                        cur_visited_clone_map.insert(*e, visited_times + 1);
                    }

                    if (visited_times == 1 || cur_visited_twice == true) {
                        queue.push_back((bfs_next_id, cur_path_clone_vec, cur_visited_clone_map, true));
                    } else {
                        queue.push_back((bfs_next_id, cur_path_clone_vec, cur_visited_clone_map, false));
                    }
                    bfs_next_id += 1;
                }
            }
        }
    }

    for res in &res_paths {
        println!("{:?}", res);
    }
    println!("res_paths {}", res_paths.len());
}
