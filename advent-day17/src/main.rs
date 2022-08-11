fn task(x1: i32, x2: i32, y1: i32, y2: i32) -> i32 {
    println!("x1 {}, x2 {}, y1 {}, y2 {}", x1, x2, y1, y2);
    // returns x_max, is_valid, step
    let get_x_max_fn = |v_x: i32, x1: i32, x2: i32| -> (i32, bool, Vec<i32>) { 
        let mut x_max: i32 = 0;
        let mut is_valid = false;
        let mut step_vec: Vec<i32> = Vec::new();
        for i in 0..v_x + 1 {
            x_max += v_x - i;
            if x1 <= x_max && x_max <= x2 {
                is_valid = true;
                step_vec.push(i + 1);
            }
        }
        return (x_max, is_valid, step_vec);
    };

    // let get_x_min_fn = |v_x: i32| -> i32 { v_x };

    // returns v_x and step
    let get_valid_v_x = |x1: i32, x2: i32| -> Vec<(i32, Vec<i32>)> {
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
            (x_max, is_valid, step_vec) = get_x_max_fn(v_x, x1, x2);
            if x_max < x1 {
                break;
            }
        }
        return res_vec;
    };

    let res = get_valid_v_x(x1, x2);
    println!("RES {:?}", res);
    return 0;
}

fn main() {
    let res = task(20, 30, -10, -5);
    // let v_x0 = 7;
    // let v_y0 = 2;

    // println!("x_max {}", get_x_max_fn(v_x0));
    // println!("res {}", res);
}

mod test {
    use crate::task;

    #[test]
    fn test01() {
        assert_eq!(task(20, 30, -10, -5), 45);
    }
}
