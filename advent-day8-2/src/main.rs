// use rand::prelude::*;
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
        for ch in s.chars() {
            *segment_count_map.get_mut(&ch).unwrap() += 1;
            // segmentCount[&c] += 1;
        }
    }
    return segment_count_map;
}

fn str_to_number_code(s: &str) -> i32 {
    let mut n: i32 = 90_000_000;
    for ch in s.chars() {
        if ch == 'a' {
            n += 1_000_000;
        } else if ch == 'b' {
            n += 100_000;
        } else if ch == 'c' {
            n += 10_000;
        } else if ch == 'd' {
            n += 1_000;
        } else if ch == 'e' {
            n += 100;
        } else if ch == 'f' {
            n += 10;
        } else if ch == 'g' {
            n += 1;
        }
    }
    return n;
}

fn str_to_number_code2(s: String) -> i32 {
    let mut n: i32 = 90_000_000;
    for ch in s.chars() {
        if ch == 'a' {
            n += 1_000_000;
        } else if ch == 'b' {
            n += 100_000;
        } else if ch == 'c' {
            n += 10_000;
        } else if ch == 'd' {
            n += 1_000;
        } else if ch == 'e' {
            n += 100;
        } else if ch == 'f' {
            n += 10;
        } else if ch == 'g' {
            n += 1;
        }
    }
    return n;
}

fn calc_canonical_segments() {
    // 0,1,2,3,4,5,6,7,8,9
    let canonical_segments = vec![
        "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
    ];
    println!("{:?}", canonical_segments);
    let segment_count_map = count_segments(&canonical_segments);
    output_map(&segment_count_map);
    // g: 7
    // c: 8
    // b: 6 !!!
    // d: 7
    // a: 8
    // e: 4 !!!
    // f: 9 !!!
    for str in canonical_segments {
        let str_num_code = str_to_number_code(str);
        println!("Segment {} is {}", str, str_num_code);
    }
    // Segment abcefg is 91110111
    // Segment cf is 90010010
    // Segment acdeg is 91011101
    // Segment acdfg is 91011011
    // Segment bcdf is 90111010
    // Segment abdfg is 91101011
    // Segment abdefg is 91101111
    // Segment acf is 91010010
    // Segment abcdefg is 91111111
    // Segment abcdfg is 91111011
}

fn get_chars_map(
    b: char,
    e: char,
    f: char,
    s1: &str,
    s4: &str,
    s7: &str,
    s8: &str,
) -> HashMap<char, char> {
    println!("get_chars_map BEGIN");
    let mut chars_map: HashMap<char, char> = HashMap::new();
    chars_map.insert(b, 'b');
    chars_map.insert(e, 'e');
    chars_map.insert(f, 'f');

    let s1a = s1.replace(f, "");
    println!("s1a {}", s1a);
    let c: char = s1a.chars().next().unwrap();
    println!("c <= {}", c);
    chars_map.insert(c, 'c');

    let s4a = s4.replace(b, "").replace(c, "").replace(f, "");
    println!("s4a {}", s4a);
    let d: char = s4a.chars().next().unwrap();
    chars_map.insert(d, 'd');
    println!("d <= {}", d);

    let s7a = s7.replace(c, "").replace(f, "");
    println!("s7a {}", s7a);
    let a: char = s7a.chars().next().unwrap();
    println!("a <= {}", a);
    chars_map.insert(a, 'a');

    let s8a = s8.replace(a, "").replace(b, "").replace(c, "").replace(d, "").replace(e, "").replace(f, "");
    println!("s8a {}", s8a);
    let g: char = s8a.chars().next().unwrap();
    println!("g <= {}", g);
    chars_map.insert(g, 'g');

    println!("get_chars_map END");
    return chars_map;
}

fn output_map(input: &HashMap<char, i32>) {
    for (key, value) in input {
        println!("{}: {}", key, value);
    }
}

fn str_to_canonical(s: &str, map: &HashMap<char, char>) -> String {
    let mut a_string = String::from("");
    for ch in s.chars() {
        let ch2 = map.get(&ch).unwrap();
        // println!("str_to_canonical {} -> {}", ch, ch2);
        a_string.push(*ch2);
    }
    return a_string;
}

fn get_number_from_code(code: i32) -> i32 {
    return match code {
        91110111 => 0,
        90010010 => 1,
        91011101 => 2,
        91011011 => 3,
        90111010 => 4,
        91101011 => 5,
        91101111 => 6,
        91010010 => 7,
        91111111 => 8,
        91111011 => 9,
        _ => -1,
    };
}

fn main() {
    // calc_canonical_segments();
    let base: i32 = 10;

    let filename = "src/data-test2.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut total_sum: i32 = 0;

    for line in contents.lines() {
        let unique_signal_patterns: &str = line.split("|").next().unwrap();
        let unique_signal_patterns_vec = unique_signal_patterns
            .split_whitespace()
            .collect::<Vec<&str>>();
        let char_count_map = count_segments(&unique_signal_patterns_vec);
        println!("output_map");
        output_map(&char_count_map);
        let mut b: char = '0';
        let mut e: char = '0';
        let mut f: char = '0';
        for (key, value) in &char_count_map {
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

        println!("unique_signal_patterns_vec BEGIN");
        for s in unique_signal_patterns_vec {
            println!("unique_signal_patterns_vec {}", s);
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
        println!("unique_signal_patterns_vec END");

        // RES MAP
        let res_map = get_chars_map(b, e, f, &str1, &str4, &str7, &str8);
        println!("RES MAP");
        for (key, value) in &res_map {
            println!("{} => {}", key, value);
        }

        let four_digit_output_value: &str = line.split("|").last().unwrap();
        let mut line_num = 0;
        for (i, single_digit_value) in four_digit_output_value.split_whitespace().enumerate() {
            // println!("single_digit_value {}", single_digit_value);
            let ss = str_to_canonical(single_digit_value, &res_map);
            // println!("ss {}", ss);
            let str_code = str_to_number_code2(ss);
            // println!("str_code {}", str_code);
            let num = get_number_from_code(str_code);
            // println!("num {}", num);

            line_num += base.pow(3 - i as u32) * num;
        }
        println!("lineNum {}", line_num);

        total_sum += line_num;
    }

    println!("totalSum {}", total_sum);
}
