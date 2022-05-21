use std::fs;
use std::string;
use std::collections::HashMap;

fn main() {
    // HASHMAP
    let mut routes_map: HashMap<&str,Vec<&str>> = HashMap::new();

    let content = fs::read_to_string("src/data-test1.txt").expect("we have a bug");
    for line in content.lines() {
        // println!("new line");
        let (from_str, to_str) = line.split_once("-").unwrap();
        // println!("from_str {}", from_str);
        // println!("to_str {}", to_str);
        let routes_to_opt = routes_map.get_mut(from_str);
        match routes_to_opt {
            Some(routes_to) => routes_to.push(to_str),
            None => {
                let mut routes_to: Vec<&str> = Vec::new();
                routes_to.push(to_str);
                routes_map.insert(from_str, routes_to);
            }
        };
        // let routes_to_reversed_opt = routes_map.get_mut(from_str);
        // match routes_to_opt {
        //     Some(routes_to) => routes_to.push(to_str),
        //     None => {
        //         let mut routes_to: Vec<&str> = Vec::new();
        //         routes_to.push(to_str);
        //         routes_map.insert(from_str, routes_to);
        //     }
        // };
    }

    for (key, value) in routes_map {
        println!("key {}", key);
        println!("values {:?}", value);
    }
}
