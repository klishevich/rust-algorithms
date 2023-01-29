use itertools::Itertools;
use regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

type TCube = HashMap<i32, HashMap<i32, HashMap<i32, bool>>>;
type TOperationOnXxYyZz = (bool, i32, i32, i32, i32, i32, i32);

fn read_from_file(file: &str) -> Vec<TOperationOnXxYyZz> {
    let content = fs::read_to_string(file).expect("some bug");
    let mut res: Vec<TOperationOnXxYyZz> = Vec::new();
    for line in content.lines() {
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
        let y_interval = intervals2.next().unwrap();
        let mut y_split = re.split(y_interval);
        let y1: i32 = y_split.nth(1).unwrap().parse().unwrap();
        let y2: i32 = y_split.next().unwrap().parse().unwrap();
        let z_interval = intervals2.next().unwrap();
        let mut z_split = re.split(z_interval);
        let z1: i32 = z_split.nth(1).unwrap().parse().unwrap();
        let z2: i32 = z_split.next().unwrap().parse().unwrap();
        res.push((on_command, x1, x2, y1, y2, z1, z2));
    }
    return res;
}

fn exec_command(cube: &mut TCube, command: &TOperationOnXxYyZz) {
    let (on, x1, x2, y1, y2, z1, z2) = command;
    if *x1 < -50 || 50 < *x2 || *y1 < -50 || 50 < *y2 || *z1 < -50 || 50 < *z2 {
        return;
    }
    for x in *x1..*x2 + 1 {
        for y in *y1..*y2 + 1 {
            for z in *z1..*z2 + 1 {
                // println!("x {}, y {}, z {}", x, y, z);
                cube.entry(x)
                    .or_default()
                    .entry(y)
                    .or_default()
                    .insert(z, *on);
            }
        }
    }
}

fn calc_intersections(commands: &Vec<TOperationOnXxYyZz>) -> (i32, i32) {
    let mut intersections_no = 0;
    let mut intersections_yes = 0;
    for (i1, c1) in commands.iter().enumerate() {
        let mut has_intersection = false;
        let c1v1 = c1.1;
        let c1v2 = c1.2;
        for (i2, c2) in commands.iter().enumerate() {
            let c2v1 = c2.1;
            let c2v2 = c2.2;
            if i1 != i2 {
                if (c2v1 <= c1v1 && c1v1 <= c2v2) || (c2v1 <= c1v2 && c1v2 <= c2v2) {
                    has_intersection = true;
                    break;
                }
            }
        }
        if has_intersection {
            intersections_yes += 1;
        } else {
            intersections_no += 1;
        }
    }
    return (intersections_no, intersections_yes);
}

fn get_state_for_dot(x: i32, y: i32, z: i32, commands: &Vec<TOperationOnXxYyZz>) -> bool {
    let mut is_on: bool = false;
    for (on, x1, x2, y1, y2, z1, z2) in commands {
        if *x1 <= x && x <= *x2 && *y1 <= y && y <= *y2 && *z1 <= z && z <= *z2 {
            is_on = *on;
        }
    }
    return is_on;
}

fn part1(file: &str) {
    let commands = read_from_file(file);

    let mut cube: TCube = HashMap::new();
    for command in &commands {
        exec_command(&mut cube, &command);
    }

    let mut on_cnt = 0;
    for (_x, map2) in &cube {
        for (_y, map3) in map2 {
            for (_z, on) in map3 {
                if *on == true {
                    on_cnt += 1;
                }
            }
        }
    }

    println!("on_cnt {}", on_cnt);
}

fn part2(file: &str) {
    let commands = read_from_file(file);
    println!("{:?}", commands);

    let mut x_set: HashSet<i32> = HashSet::new();
    let mut y_set: HashSet<i32> = HashSet::new();
    let mut z_set: HashSet<i32> = HashSet::new();

    for (_, x1, x2, y1, y2, z1, z2) in &commands {
        x_set.insert(*x1);
        x_set.insert(*x2 + 1);
        y_set.insert(*y1);
        y_set.insert(*y2 + 1);
        z_set.insert(*z1);
        z_set.insert(*z2 + 1);
    }

    let mut res: u64 = 0;
    // SORT HASH SET Itertools crate
    let x_vector: Vec<&i32> = x_set.iter().sorted().collect();
    let y_vector: Vec<&i32> = y_set.iter().sorted().collect();
    let z_vector: Vec<&i32> = z_set.iter().sorted().collect();
    println!("x_vector {:?}", x_vector);
    println!("y_vector {:?}", y_vector);
    println!("z_vector {:?}", z_vector);

    for x_i in 0..x_vector.len() - 1 {
        let x1 = *x_vector[x_i];
        let x2 = *x_vector[x_i + 1];
        for y_i in 0..y_vector.len() - 1 {
            let y1 = *y_vector[y_i];
            let y2 = *y_vector[y_i + 1];
            for z_i in 0..z_vector.len() - 1{
                let z1 = *z_vector[z_i];
                let z2 = *z_vector[z_i + 1];
                let is_on = get_state_for_dot(x1, y1, z1, &commands);
                if is_on {
                    let x_size: u64 = (x2 - x1).try_into().unwrap();
                    let y_size: u64 = (y2 - y1).try_into().unwrap();
                    let z_size: u64 = (z2 - z1).try_into().unwrap();
                    res += x_size * y_size * z_size;
                }
            }
        }
    }
    println!("res {}", res);
}

fn main() {
    // part1("src/data2_1.txt");
    part2("src/data3.txt");
}
