fn min_total_fuel1(f: fn(i32) -> i32, input: &Vec<i32>) -> i32 {
    return (*input.iter().min().unwrap() ..= *input.iter().max().unwrap())
      .map(|pos| input.iter().map(|x| f((pos-x).abs())).sum())
      .min().unwrap();
}

fn main() {
    let input = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

    let min_total_fuel = min_total_fuel1;
    let res_part1 = min_total_fuel(|d| d , &input);
    let res_part2 = min_total_fuel(|d| d * (d + 1) / 2 , &input);

    println!("Part 1: {}, Part 2: {}", res_part1, res_part2);
}
