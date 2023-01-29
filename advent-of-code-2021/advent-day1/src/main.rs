use std::fs;

fn main() {
    let content = fs::read_to_string("src/data01.txt").expect("some bug");
    let mut prev1: i32 = -1;
    let mut prev2: i32 = -1;
    let mut prev3: i32 = -1;
    let mut res_cnt = 0;
    for line in content.lines() {
        let cur: i32 = line.parse().unwrap();
        if prev3 != -1 {
            if cur > prev3 {
                res_cnt += 1;
            }
        }
        prev3 = prev2;
        prev2 = prev1;
        prev1 = cur;
    }
    println!("{}", res_cnt);
}
