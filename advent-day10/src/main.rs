use std::fs;

fn get_open_brace(c: char) -> char {
    match c {
        ']' => '[',
        ')' => '(',
        '}' => '{',
        '>' => '<',
        _ => '?',
    }
}

fn get_part1_val(c: char) -> i32 {
    match c {
        ']' => 57,
        ')' => 3,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn get_part2_val(c: char) -> i64 {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => 0,
    }
}

fn part1() {
    let filename = "src/data-real.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let open_braces = vec!['(', '{', '[', '<'];

    let mut illegal_char_vec: Vec<char> = Vec::new();

    for line in contents.lines() {
        let mut c_vec: Vec<char> = Vec::new();
        let mut is_corrupted = false;
        for c in line.chars() {
            if open_braces.contains(&c) {
                c_vec.push(c);
            } else {
                let index_option = c_vec.iter().rev().position(|&r| r == get_open_brace(c));
                if index_option == None || index_option.unwrap() != 0 {
                    illegal_char_vec.push(c);
                    is_corrupted = true;
                    break;
                } else {
                    c_vec.pop();
                }
            }
        }
        if is_corrupted {
            println!("- The line is corrupted -");
            println!("{}", line);
        }
        let res = illegal_char_vec
            .iter()
            .fold(0, |sum, &c| sum + get_part1_val(c));
        println!("res {}", res);
    }
}

fn part2() {
    let filename = "src/data-test0.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let open_braces = vec!['(', '{', '[', '<'];

    let mut scores_vec: Vec<i64> = Vec::new();

    for line in contents.lines() {
        let mut c_vec: Vec<char> = Vec::new();
        let mut is_corrupted = false;
        for c in line.chars() {
            if open_braces.contains(&c) {
                c_vec.push(c);
            } else {
                let index_option = c_vec.iter().rev().position(|&r| r == get_open_brace(c));
                if index_option == None || index_option.unwrap() != 0 {
                    is_corrupted = true;
                    break;
                } else {
                    c_vec.remove(c_vec.len() - 1);
                }
            }
        }
        if !is_corrupted {
            let score = c_vec
                .into_iter()
                .rev()
                .fold(0, |acc, c| acc * 5 + get_part2_val(c));
            scores_vec.push(score);
        }
    }

    scores_vec.sort_unstable();

    println!("Res {}", scores_vec[(scores_vec.len() - 1) / 2]);
}

fn main() {
    part1();
}
