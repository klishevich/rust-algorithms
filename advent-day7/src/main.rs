use std::cmp;

fn min_total_fuel1(f: fn(i32) -> i32, input: &Vec<i32>) -> i32 {
    let mut min_sum = i32::MAX;
    let mut pos = *input.iter().min().unwrap();
    while pos <= *input.iter().max().unwrap() {
        let sum = input.iter().fold(0, |sum, &x| sum + f((pos-x).abs()));
        min_sum = cmp::min(min_sum, sum);
        pos+=1;
    }
    return min_sum;
}

fn main() {
    let input_test = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
    let input = input_test;
    println!("{:?}", input);
    let dist_fn_part1 = |d: i32| -> i32 { d };
    let dist_fn_part2 = |d: i32| -> i32 { d * (d + 1) / 2 };

    let min_total_fuel = min_total_fuel1;
    let res_part1 = min_total_fuel(dist_fn_part1, &input);
    let res_part2 = min_total_fuel(dist_fn_part2, &input);

    println!("format {} arguments", "some");
    println!("Part 1: {}, Part 2: {}", res_part1, res_part2);
}
