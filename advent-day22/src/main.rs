use regex;
use std::collections::HashMap;
use std::fs;

type TCube = HashMap<i32, HashMap<i32, HashMap<i32, bool>>>;
type TOperationOnXxYyZz = (bool, i32, i32, i32, i32, i32, i32);

fn read_from_file() -> Vec<TOperationOnXxYyZz> {
    let content = fs::read_to_string("src/data1.txt").expect("some bug");
    let mut res: Vec<TOperationOnXxYyZz> = Vec::new();
    for line in content.lines() {
        println!("line {}", line);
        let mut a = line.split(" ");
        let command_opt = a.next();

        let on_command = match command_opt {
            Some(s) => {
                if s == "on" {
                    true
                } else {
                    false
                }
            }
            None => false,
        };
        let intervals = a.next().unwrap();
        let mut intervals2 = intervals.split(",");
        let re = regex::Regex::new(r"=|\.\.").unwrap();
        let x_interval = intervals2.next().unwrap();
        let mut x_split = re.split(x_interval);
        let x1: i32 = x_split.nth(1).unwrap().parse().unwrap();
        let x2: i32 = x_split.next().unwrap().parse().unwrap();
        println!("x1 {}, x2 {}", x1, x2);
        let y_interval = intervals2.next().unwrap();
        let mut y_split = re.split(y_interval);
        let y1: i32 = y_split.nth(1).unwrap().parse().unwrap();
        let y2: i32 = y_split.next().unwrap().parse().unwrap();
        println!("y1 {}, y2 {}", y1, y2);
        let z_interval = intervals2.next().unwrap();
        let mut z_split = re.split(z_interval);
        let z1: i32 = z_split.nth(1).unwrap().parse().unwrap();
        let z2: i32 = z_split.next().unwrap().parse().unwrap();
        println!("z1 {}, z2 {}", z1, z2);
        res.push((on_command, x1, x2, y1, y2, z1, z2));
    }
    return res;
}

fn main() {
    let mut cube: TCube = HashMap::new();
    let commands = read_from_file();
    println!("{:?}", commands);
}
