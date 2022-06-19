use std::fs;
use std::collections::HashMap;

fn main() {
    let content = fs::read_to_string("src/data-test1.txt").expect("we have a bug");

    let initial_line = content.lines().next().unwrap();
    let mut char_vec: Vec<char> = initial_line.chars().collect();
    println!("initial_line {}", &initial_line);

    let mut rules_map: HashMap<char, HashMap<char, char>> = HashMap::new();

    for (index, line) in content.lines().enumerate() {
        if index < 2 {
            continue;
        }
        let (left, right) = line.split_once(" -> ").unwrap();
        let char1 = left.chars().next().unwrap();
        let char2 = left.chars().nth(1).unwrap();
        let char3 = right.chars().next().unwrap();
        rules_map.entry(char1).or_default().insert(char2, char3);
    }

    // OUTPUT the HashMap
    // for (ch1, map1) in rules {
    //     println!("ch1 {}", ch1);
    //     for (el, map2) in map1 {
    //         println!("ch2 {}, ch3 {}", el, map2);
    //     }
    // }

    println!("BEFORE CHAR_VEC");
    for ch in &char_vec {
        print!("{}", ch);
    }

    for j in 0..40 {
        let mut i = 0;
        let mut len = char_vec.len();
        loop {
            let el = char_vec.get(i).unwrap();
            let next_el = char_vec.get(i+1).unwrap();
            let ins_char_opt = rules_map.entry(*el).or_default().get(next_el);
            match ins_char_opt {
                Some(ins_char) => {
                    char_vec.insert(i+1, *ins_char);
                    len = len + 1;
                    i = i + 2;
                },
                None => i = i+1
            };
            if i >= len - 1 {
                break;
            };
        }
    
        // println!("\nRESULT CHAR_VEC");
        // for ch in &char_vec {
        //     print!("{}", ch);
        // }
        println!("Step {} CHAR_VEC lenth {}", j, len);
    }

    let mut count_map: HashMap<char, i32> = HashMap::new();

    for ch in char_vec {
        *count_map.entry(ch).or_insert(0) += 1;
    }
    let mut max = std::i32::MIN;
    let mut min = std::i32::MAX;

    for (k,v) in count_map {
        if v < min {
            min = v;
        }
        if v > max {
            max = v;
        }
        println!("key {}, value {}", k, v);
    }

    println!("res {}", max - min);
}
