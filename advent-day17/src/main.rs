use itertools::Itertools;
use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;

/**
 * @returns (x_max, is_valid, step_vec)
 */
fn get_x_max(v_x: i32, x1: i32, x2: i32) -> (i32, bool, Vec<i32>) {
    let mut x_max: i32 = 0;
    let mut is_valid = false;
    let mut step_vec: Vec<i32> = Vec::new();
    for i in 0..v_x {
        x_max += v_x - i;
        if x1 <= x_max && x_max <= x2 {
            is_valid = true;
            step_vec.push(i + 1);
        }
    }
    return (x_max, is_valid, step_vec);
}

/**
 * @returns Vec<(v_x, steps)>
 */
fn get_valid_v_x(x1: i32, x2: i32) -> Vec<(i32, Vec<i32>)> {
    let mut res_vec: Vec<(i32, Vec<i32>)> = Vec::new();
    let mut v_x = x2;
    let mut is_valid = true;
    let mut step_vec: Vec<i32> = vec![1];
    let mut x_max;
    loop {
        if is_valid {
            res_vec.push((v_x, step_vec));
        }
        v_x -= 1;
        (x_max, is_valid, step_vec) = get_x_max(v_x, x1, x2);
        if x_max < x1 {
            break;
        }
    }
    return res_vec;
}

/**
 * @returns HashMap<step, Vec<v_x>>
 */
fn get_valid_x_steps(x1: i32, x2: i32) -> HashMap<i32, Vec<i32>> {
    // HASH MAP
    let mut res_map: HashMap<i32, Vec<i32>> = HashMap::new();
    let valid_v_x_vec = get_valid_v_x(x1, x2);
    // REVERSE ITERATION
    for (v_x, steps) in valid_v_x_vec.iter().rev() {
        for step in steps {
            // INSERT HASH MAP OF DEFAULT
            res_map.entry(*step).or_default().push(*v_x);
        }
    }
    return res_map;
}

fn get_y_n(v_y: i32, n: i32) -> i32 {
    return n * v_y - n * (n - 1) / 2;
}

fn get_v_y(y_1: i32, y_2: i32, n: i32) -> (f32, f32) {
    let get_v_fn =
        |y_: i32, n_: i32| -> f32 { (y_ as f32) / (n_ as f32) + ((n_ as f32) - 1.0) / 2.0 };
    let y_ext = ((n * n) as f32) / 2.0;
    let mut v_min: f32 = get_v_fn(y_1, n);
    let mut v_max: f32 = get_v_fn(y_2, n);
    if v_min > v_max {
        let tmp = v_max;
        v_max = v_min;
        v_min = tmp;
    }
    return (v_min, v_max);
}

/**
 * @returns Vec<(v_y, steps)>
 */
fn get_valid_v_y(y1: i32, y2: i32) -> Vec<(i32, Vec<i32>)> {
    let y_max = y1.abs();
    let y_min = y2.abs();
    let mut res_vec: Vec<(i32, Vec<i32>)> = Vec::new();
    for v_y0 in 1..y_max + 1 {
        let v_y0_mirror = v_y0 - 1;
        let mut step = 0;
        let mut y = 0;
        let mut v_y = v_y0;
        let mut steps_vec: Vec<i32> = Vec::new();
        let mut steps_vec_mirror: Vec<i32> = Vec::new();
        loop {
            step += 1;
            y = y + v_y;
            if y > y_max {
                break;
            }
            v_y = v_y + 1;
            if y_min <= y && y <= y_max {
                steps_vec.push(step);
                steps_vec_mirror.push(step + v_y0_mirror * 2 + 1);
            }
        }
        if steps_vec.len() > 0 {
            res_vec.push((v_y0_mirror, steps_vec_mirror));
            res_vec.insert(0, (-v_y0, steps_vec));
        }
    }
    return res_vec;
}

fn part1() {
    let mut res_v: i32 = 0;
    for i in 1..1000 {
        let (v_min, v_max) = get_v_y(-136, -86, i);
        if v_min.ceil() <= v_max.floor() {
            res_v = v_max.floor() as i32;
        }
    }
    println!("res_v {}", res_v);
    let max_n = (2 * res_v + 1) / 2;
    println!("max_n {}", max_n);
    let y1 = get_y_n(res_v, max_n - 1);
    let y2 = get_y_n(res_v, max_n);
    let y3 = get_y_n(res_v, max_n + 1);
    println!("y1 {}, y2 {} y3 {}", y1, y2, y3);
}

fn part2() {
    // let (x1, x2, y1, y2) = (2, 3, -3, -2);
    let (x1, x2, y1, y2) = (20, 30, -10, -5);
    // let (x1, x2, y1, y2) = (150, 193, -136, -86);

    let valid_v_y_vec = get_valid_v_y(y1, y2);
    println!("valid_v_y_vec {:?}", valid_v_y_vec);

    let mut valid_x_steps: HashMap<i32, Vec<i32>> = HashMap::new();
    let valid_v_x_vec = get_valid_v_x(x1, x2);
    println!("valid_v_x_vec {:?}", valid_v_x_vec);

    let mut infinitely_valid_x_steps: Vec<i32> = Vec::new();
    for (v_x, steps) in valid_v_x_vec.iter().rev() {
        for step in steps {
            valid_x_steps.entry(*step).or_default().push(*v_x);
            if v_x == step {
                infinitely_valid_x_steps.push(*v_x);
            }
        }
    }

    println!("valid_x_steps {:?}", valid_x_steps);
    println!("infinitely_valid_x_steps {:?}", infinitely_valid_x_steps);

    // let mut res: Vec<(i32, i32)> = Vec::new();
    // HASH SET
    let mut res: HashSet<String> = HashSet::new();

    for (v_y, steps) in valid_v_y_vec {
        for step in steps {
            let v_x_vec_opt = valid_x_steps.get(&step);
            match v_x_vec_opt {
                Some(v_x_vec) => {
                    for v_x in v_x_vec {
                        res.insert(v_x.to_string() + "_" + &v_y.to_string());
                        // res.push((*v_x, v_y));
                    }
                }
                None => ()
            };
            for v_x2 in &infinitely_valid_x_steps {
                if step > *v_x2 {
                    res.insert(v_x2.to_string() + "_" + &v_y.to_string());
                    // res.push((*v_x2, v_y));
                }
            }
        }
    }

    // println!("res {:?}", res);
    // res.sort_by(|a, b| a.cmp(b));
    // println!("sorted {:?}", res);

    println!("res {}", res.len());
}

fn main() {
    // part1();
    part2();
}

mod test {
    use crate::get_valid_v_x;
    use crate::get_valid_v_y;
    use crate::get_valid_x_steps;
    use crate::get_x_max;
    use crate::get_y_n;
    use std::collections::HashMap;

    #[test]
    fn get_x_max_test0() {
        assert_eq!(get_x_max(1, 2, 3), (1, false, vec![]));
    }
    #[test]
    fn get_x_max_test1() {
        assert_eq!(get_x_max(2, 2, 3), (3, true, vec![1, 2]));
    }
    #[test]
    fn get_x_max_test2() {
        assert_eq!(get_x_max(3, 2, 3), (6, true, vec![1]));
    }
    #[test]
    fn get_x_max_test3() {
        assert_eq!(get_x_max(4, 2, 3), (10, false, vec![]));
    }
    #[test]
    fn get_valid_v_x_test0() {
        assert_eq!(get_valid_v_x(2, 3), vec![(3, vec![1]), (2, vec![1, 2])])
    }
    #[test]
    fn get_valid_v_x_test1() {
        let v_x_vec = vec![
            (30, vec![1]),
            (29, vec![1]),
            (28, vec![1]),
            (27, vec![1]),
            (26, vec![1]),
            (25, vec![1]),
            (24, vec![1]),
            (23, vec![1]),
            (22, vec![1]),
            (21, vec![1]),
            (20, vec![1]),
            (15, vec![2]),
            (14, vec![2]),
            (13, vec![2]),
            (12, vec![2]),
            (11, vec![2, 3]),
            (10, vec![3]),
            (9, vec![3, 4]),
            (8, vec![3, 4, 5]),
            (7, vec![4, 5, 6, 7]),
            (6, vec![5, 6]),
        ];
        assert_eq!(get_valid_v_x(20, 30), v_x_vec);
    }
    #[test]
    fn get_valid_x_steps_test0() {
        let steps_map = HashMap::from([(1, vec![2, 3]), (2, vec![2])]);
        assert_eq!(get_valid_x_steps(2, 3), steps_map);
    }
    #[test]
    fn get_valid_x_steps_test1() {
        let steps_map = HashMap::from([
            (1, vec![20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]),
            (2, vec![11, 12, 13, 14, 15]),
            (3, vec![8, 9, 10, 11]),
            (4, vec![7, 8, 9]),
            (5, vec![6, 7, 8]),
            (6, vec![6, 7]),
            (7, vec![7]),
        ]);
        assert_eq!(get_valid_x_steps(20, 30), steps_map);
    }
    #[test]
    fn get_y_n_test0() {
        let res = get_y_n(3, 3);
        assert_eq!(res, 6);
    }
    #[test]
    fn get_y_n_test1() {
        let res = get_y_n(0, 3);
        assert_eq!(res, -3);
    }
    #[test]
    fn get_y_n_test2() {
        let res = get_y_n(-4, 2);
        assert_eq!(res, -9);
    }
    #[test]
    fn get_valid_v_y_test0() {
        let expected_res: Vec<(i32, Vec<i32>)> = vec![
            (-3, vec![1]),
            (-2, vec![1]),
            (-1, vec![2]),
            (0, vec![3]),
            (1, vec![4]),
            (2, vec![6]),
        ];
        assert_eq!(get_valid_v_y(-3, -2), expected_res);
    }
}
