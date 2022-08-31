struct SnailfishNumber {
    left: Box<SnailfishNumber>,
    right: Box<SnailfishNumber>,
    left_val: i32,
    right_val: i32
}

fn explode(num: &str) -> String {
    // NEW STRING FROM
    let a = String::from("test");
    let b = a + num;
    return b;
}

fn main() {
    println!("Hello, world!");
}

mod test {
    use crate::explode;

    #[test]
    fn explode_test() {
        // assert_eq!(explode("1"), "test1");
        assert_eq!(explode("[[[[[9,8],1],2],3],4]"), "[[[[0,9],2],3],4]");
    }
}
