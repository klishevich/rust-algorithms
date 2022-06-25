use std::fs;
use std::collections::HashMap;

fn part1() {
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

    println!("BEFORE CHAR_VEC");
    for ch in &char_vec {
        print!("{}", ch);
    }

    let steps = 10;

    for j in 0..steps {
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

        println!("Step {} CHAR_VEC length {}", j, len);
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

fn part2() {
    let content = fs::read_to_string("src/data-test1.txt").expect("we have a bug");

    // INITIAL POLYMER
    let initial_line = content.lines().next().unwrap();
    println!("initial_line {}", &initial_line);

    // RULES
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
    println!("rules_map");
    for (ch1, map1) in &rules_map {
        println!("{}", ch1);
        for (ch2, ch3) in map1 {
            println!(" {} -> {}", ch2, ch3);
        }
    }

    // CALC RULE_RESULTS
    let mut rule_results10: HashMap<char, HashMap<char, HashMap<char, i64>>> = HashMap::new();
    for (ch1, map1) in &rules_map {
        println!("{}", ch1);
        for (ch2, _) in map1 {
            let mut char_vec: Vec<char> = vec![*ch1, *ch2];
            let steps = 10;
            for _j in 0..steps {
                let mut i = 0;
                let mut len = char_vec.len();
                loop {
                    let el = char_vec.get(i).unwrap();
                    let next_el = char_vec.get(i+1).unwrap();
                    match rules_map.get(el) {
                        Some(char_map1) => {
                            match char_map1.get(next_el) {
                                Some(ins_char) => {
                                    char_vec.insert(i+1, *ins_char);
                                    len = len + 1;
                                    i = i + 2;
                                },
                                None => i = i + 1
                            };
                        },
                        None => i = i + 1
                    };
                    if i >= len - 1 {
                        break;
                    };
                }
            }
            let mut count_map: HashMap<char, i64> = HashMap::new();
            for ch in char_vec {
                // MAP ENTRY OR INSERT
                *count_map.entry(ch).or_insert(0) += 1;
            }
            rule_results10.entry(*ch1).or_default().insert(*ch2, count_map);
        }
    }

    println!("rule_results10");
    for (ch1, map1) in &rule_results10 {
        println!("{}", ch1);
        for (ch2, map2) in map1 {
            println!(" {}", ch2);
            for (ch3, cnt) in map2 {
                println!("  {} -> {}", ch3, cnt);
            }
        }
    }
}

fn main() {
    part2();
}
