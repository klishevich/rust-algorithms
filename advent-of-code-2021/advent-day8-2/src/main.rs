use std::collections::HashMap;
use std::fs;

fn str_to_number_code(s: String) -> i32 {
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
    let canonical_segments = vec!["abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg"];
    println!("{:?}", canonical_segments);

    let mut segment_count_map = HashMap::from([('a', 0), ('b', 0), ('c', 0), ('d', 0), ('e', 0), ('f', 0), ('g', 0)]);
    for s in &canonical_segments {
        for ch in s.chars() {
            *segment_count_map.get_mut(&ch).unwrap() += 1;
        }
    }

    for (key, value) in &segment_count_map { println!("{}: {}", key, value); }
    // g: 7
    // c: 8
    // b: 6 !!!
    // d: 7
    // a: 8
    // e: 4 !!!
    // f: 9 !!!
    for str in &canonical_segments {
        let new_string = str.to_string();
        let str_num_code = str_to_number_code(new_string);
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

fn get_chars_map(unique_signal_patterns_iter: std::iter::Enumerate<std::str::SplitWhitespace>) -> HashMap<char, char> {
    // 1. Count chars and find s1, s4, s7, s8
    let mut char_count_map = HashMap::from([('a', 0), ('b', 0), ('c', 0), ('d', 0), ('e', 0), ('f', 0), ('g', 0)]);
    let (mut str1, mut str4, mut str7, mut str8) = ("", "", "", "");
    for (_i, s) in unique_signal_patterns_iter {
        let len = s.len();
        if len == 2 { str1 = s };
        if len == 4 { str4 = s };
        if len == 3 { str7 = s };
        if len == 7 { str8 = s };
        for ch in s.chars() {
            *char_count_map.get_mut(&ch).unwrap() += 1;
        }
    }

    // 2. Determine b, e and f chars
    let (mut b, mut e, mut f) = ('0', '0', '0');
    for (key, value) in char_count_map {
        if value == 6 { b = key };
        if value == 4 { e = key };
        if value == 9 { f = key };
    }

    // 3. Determine c, d, a, g
    let c_str = str1.replace(f, "");
    let c: char = c_str.chars().next().unwrap();
    let d_str = str4.replace(&[b, c, f], "");
    let d: char = d_str.chars().next().unwrap();
    let a_str = str7.replace(&[c, f], "");
    let a: char = a_str.chars().next().unwrap();
    let g_str = str8.replace(&[a, b, c, d, e, f], "");
    let g: char = g_str.chars().next().unwrap();

    return HashMap::from([(a, 'a'), (b, 'b'), (c, 'c'), (d, 'd'), (e, 'e'), (f, 'f'), (g, 'g')]);
}

fn get_display_value(s: &str, map: &HashMap<char, char>) -> i32 {
    // Convert to canonical string
    let mut canonical_str = String::from("");
    for ch in s.chars() {
        let ch2 = map.get(&ch).unwrap();
        canonical_str.push(*ch2);
    }

    // Get string code
    let str_code = str_to_number_code(canonical_str);

    // Get number fron code
    let num = get_number_from_code(str_code);

    return num;
}

fn main() {
    // calc_canonical_segments();
    let base: i32 = 10;

    let filename = "src/data-real.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut total_sum: i32 = 0;

    for line in contents.lines() {
        let unique_signal_patterns_iter: std::iter::Enumerate<std::str::SplitWhitespace> = line.split("|").next().unwrap().split_whitespace().enumerate();

        let res_map = get_chars_map(unique_signal_patterns_iter);

        let mut line_num = 0;
        let four_digit_output_values_iter = line.split("|").last().unwrap().split_whitespace().enumerate();
        for (i, single_digit_value) in four_digit_output_values_iter {
            let num = get_display_value(single_digit_value, &res_map);
            line_num += base.pow(3 - i as u32) * num;
        }

        total_sum += line_num;
    }

    println!("totalSum {}", total_sum);
}
