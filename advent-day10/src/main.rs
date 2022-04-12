use std::fs;

fn get_open_brace(c: char) -> char {
    match c {
        ']' => '[',
        ')' => '(',
        '}' => '{',
        '>' => '<',
        _ => '?'
    }
}

fn get_cor_val(c: char) -> i32 {
    match c {
        ']' => 57,
        ')' => 3,
        '}' => 1197,
        '>' => 25137,
        _ => 0
    }
}

fn main() {
    let filename = "src/data-real.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let open_braces = vec!['(', '{', '[', '<'];
    let close_braces = vec![')', '}', ']', '>'];

    let mut illegal_char_vec: Vec<char> = Vec::new();

    for line in contents.lines() {
        let mut c_vec: Vec<char> = Vec::new();
        let mut is_corrupted = false;
        for c in line.chars(){
            if open_braces.contains(&c) {
                c_vec.push(c);
            } else {
                let index_option = c_vec.iter().rev().position(|&r| r == get_open_brace(c));
                if index_option == None || index_option.unwrap() != 0 {
                    illegal_char_vec.push(c);
                    is_corrupted = true;
                    break;
                } else {
                    c_vec.remove(c_vec.len() - 1);
                }
            }
        }
        if is_corrupted {
            println!("- The line is corrupted -");
            println!("{}", line);
        }
        let res = illegal_char_vec.iter().fold(0, |sum, &c| sum + get_cor_val(c));
        println!("res {}", res);
    }
}
