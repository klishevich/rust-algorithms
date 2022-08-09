fn task(x1: i32, x2: i32, y1: i32, y2: i32) -> i32 {
    println!("x1 {}, x2 {}, y1 {}, y2 {}", x1, x2, y1, y2);
    return 0;
}

fn main() {
    let res = task(20, 30, -10, -5);
    println!("res {}", res);
}

mod test {
    use crate::task;

    #[test]
    fn test01() {
        assert_eq!(task(20, 30, -10, -5), 45);
    }
}