use rand::prelude::*;
use std::collections::HashMap;
use std::fs;

fn count_segments(input: &Vec<&str>) -> HashMap<char, i32> {
    let mut segment_count_map = HashMap::from([
        ('a', 0),
        ('b', 0),
        ('c', 0),
        ('d', 0),
        ('e', 0),
        ('f', 0),
        ('g', 0),
    ]);
    for s in input {
        for c in s.chars() {
            *segment_count_map.get_mut(&c).unwrap() += 1;
            // segmentCount[&c] += 1;
        }
    }
    return segment_count_map;
}

fn get_chars_map(
    b: char,
    e: char,
    f: char,
    str1: &str,
    str4: &str,
    str7: &str,
    str8: &str,
) -> HashMap<char, char> {
    let mut chars_map: HashMap<char, char> = HashMap::new();
    chars_map.insert('b', b);
    chars_map.insert('e', e);
    chars_map.insert('f', f);
    let c: char = str1.replace(f, "").chars().next().unwrap();
    chars_map.insert('c', c);
    let d: char = str4
        .replace(b, "")
        .replace(c, "")
        .replace(f, "")
        .chars()
        .next()
        .unwrap();
    chars_map.insert('d', d);
    let a: char = str7.replace(c, "c").replace(f, "").chars().next().unwrap();
    chars_map.insert('a', a);
    let g: char = str8
        .replace(a, "")
        .replace(b, "")
        .replace(c, "")
        .replace(d, "")
        .replace(e, "")
        .replace(f, "")
        .chars()
        .next()
        .unwrap();
    chars_map.insert('g', g);
    return chars_map;
}

fn output_map(input: &HashMap<char, i32>) {
    for (key, value) in input {
        println!("{}: {}", key, value);
    }
}

fn convert_to_number(s: &str, map: HashMap<char, char>) {
    
    // s.chars().
}

fn main() {
    // let canonical_segments = vec!["cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg", "abcefg"];
    // println!("{:?}", canonical_segments);
    // let segment_count_map = count_segments(&canonical_segments);
    // output_map(&segment_count_map);
    // b = 6
    // e = 4
    // f = 9

    let filename = "src/data-test.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let sum: i32 = 0;

    for line in contents.lines() {
        let unique_signal_patterns: &str = line.split("|").next().unwrap();
        let unique_signal_patterns_vec = unique_signal_patterns
            .split_whitespace()
            .collect::<Vec<&str>>();
        let segment_count_map = count_segments(&unique_signal_patterns_vec);
        // output_map(&segment_count_map);
        let mut b: char = '0';
        let mut e: char = '0';
        let mut f: char = '0';
        for (key, value) in &segment_count_map {
            if *value == 6 {
                b = *key;
            } else if *value == 4 {
                e = *key;
            } else if *value == 9 {
                f = *key;
            }
        }
        println!("b = {}, e = {}, f = {}", b, e, f);

        let mut str1: &str = "";
        let mut str4: &str = "";
        let mut str7: &str = "";
        let mut str8: &str = "";

        for s in unique_signal_patterns_vec {
            if s.len() == 2 {
                str1 = s;
            } else if s.len() == 4 {
                str4 = s;
            } else if s.len() == 3 {
                str7 = s;
            } else if s.len() == 7 {
                str8 = s
            }
        }

        // RES MAP
        let res_map = get_chars_map(b, e, f, &str1, &str4, &str7, &str8);
        for (key, value) in res_map {
            println!("{}: {}", key, value);
        }

        let four_digit_output_value: &str = line.split("|").last().unwrap();
        for single_digit_value in four_digit_output_value.split_whitespace() {
            println!("single_digit_value {}", single_digit_value);
            // let mut decoded: String = String::new();
            // for c in single_digit_value.chars() {
            //     decoded.add(res_map[c]);
            // }
        }
    }

    // println!("cnt_1_4_7: {}", cnt_1_4_7);
    // let x: u8 = random();
    // println!("rand {}", x);
}
