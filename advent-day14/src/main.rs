use std::fs;
use std::collections::HashMap;

type RulesResult = HashMap<char, HashMap<char, HashMap<char, i64>>>;

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
    let mut rule_results0: RulesResult = HashMap::new();
    for (index, line) in content.lines().enumerate() {
        if index < 2 {
            continue;
        }
        let (left, right) = line.split_once(" -> ").unwrap();
        let char1 = left.chars().next().unwrap();
        let char2 = left.chars().nth(1).unwrap();
        let char3 = right.chars().next().unwrap();
        rules_map.entry(char1).or_default().insert(char2, char3);
        rule_results0.entry(char1).or_default().insert(char2, HashMap::new());
    }
    println!("rules_map");
    for (ch1, map1) in &rules_map {
        println!("{}", ch1);
        for (ch2, ch3) in map1 {
            println!(" {} -> {}", ch2, ch3);
        }
    }

    let get_result_for_any_string_fn = |rule_results: &RulesResult, char_vec: Vec<char>, skip_edge_chars: bool | -> HashMap<char, i64> {
        let mut count_map: HashMap<char, i64> = HashMap::new();
        for i in 0..char_vec.len() {
            let el = char_vec.get(i).unwrap();
            *count_map.entry(*el).or_insert(0) += 1;
            if skip_edge_chars && (i == 0 || i == char_vec.len() - 1) {
                *count_map.entry(*el).or_insert(0) -= 1;
            }
            if i < char_vec.len() - 1 {
                let next_el = char_vec.get(i+1).unwrap();
                match rule_results.get(el) {
                    Some(prev_rule_el) => {
                        match prev_rule_el.get(next_el) {
                            Some(prev_rule_el2) => {
                                for (ch, cnt) in prev_rule_el2 {
                                    *count_map.entry(*ch).or_insert(0) += cnt;
                                }
                            },
                            None => ()
                        };
                    },
                    None => ()
                };
            }
        }
        return count_map
    };

    let get_result_fn = |prev_rule_results: RulesResult| -> RulesResult {
        let mut rule_results_next: RulesResult = HashMap::new();
        for (ch1, map1) in &rules_map {
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

                let count_map = get_result_for_any_string_fn(&prev_rule_results, char_vec, true);
                rule_results_next.entry(*ch1).or_default().insert(*ch2, count_map);
            }
        };
        return rule_results_next;
    };

    // CALC RULE_RESULTS
    let rule_results10 = get_result_fn(rule_results0);
    let rule_results20 = get_result_fn(rule_results10);
    let rule_results30 = get_result_fn(rule_results20);
    let rule_results40 = get_result_fn(rule_results30);

    println!("rule_results10");
    for (ch1, map1) in &rule_results40 {
        println!("{}", ch1);
        for (ch2, map2) in map1 {
            println!(" {}", ch2);
            for (ch3, cnt) in map2 {
                println!("  {} -> {}", ch3, cnt);
            }
        }
    }

    let initial_line_char_vec: Vec<char> = initial_line.chars().collect();
    println!("BEFORE CHAR_VEC");
    for ch in &initial_line_char_vec {
        print!("{}", ch);
    }
    let rrr = get_result_for_any_string_fn(&rule_results40, initial_line_char_vec, false);
    let mut max = std::i64::MIN;
    let mut min = std::i64::MAX;
    for (k,v) in rrr {
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

fn main() {
    part2();
}
