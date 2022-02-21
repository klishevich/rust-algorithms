use std::fs;

fn main() {
    let filename = "src/data-real.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let vec_1_4_7_8 = vec![2, 4, 3, 7];
    let mut cnt_1_4_7_8: i32 = 0;
    for line in contents.lines() {
        let four_digit_output_value: &str =  line.split("|").last().unwrap();
        for single_digit_value in four_digit_output_value.split_whitespace() {
            if vec_1_4_7_8.contains(&single_digit_value.len()) {
                cnt_1_4_7_8 += 1;
            }
        }
    }

    println!("cnt_1_4_7: {}", cnt_1_4_7_8);
}
