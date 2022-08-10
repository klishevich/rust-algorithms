fn task(x1: i32, x2: i32, y1: i32, y2: i32) -> i32 {
    println!("x1 {}, x2 {}, y1 {}, y2 {}", x1, x2, y1, y2);
    return 0;
}

fn main() {
    let get_x_max_fn = |v_x: i32| -> i32 { 
        let mut x_max = 0;
        for i in 0..v_x + 1 {
            x_max += v_x - i;
        }
        return x_max;
    };

    let res = task(20, 30, -10, -5);
    let v_x0 = 7;
    let v_y0 = 2;

    println!("x_max {}", get_x_max_fn(v_x0));
    println!("res {}", res);
}

mod test {
    use crate::task;

    #[test]
    fn test01() {
        assert_eq!(task(20, 30, -10, -5), 45);
    }
}
