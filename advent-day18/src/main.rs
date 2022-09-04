use std::str;

struct SnailfishNumber {
    left: Link,
    right: Link,
    left_val: i32,
    right_val: i32,
}

type Link = Option<Box<SnailfishNumber>>;

// impl SnailfishNumber {
//     pub fn create(&mut self, &str) {

//     }
// }

// type BStr<'a> = (&'a [u8], usize);

// SPLIT STRING ARRAY OF BYTES
fn split_num(s_val: &[u8]) -> (&[u8], &[u8]) {
    // let (s_val, s_size) = s;
    let mut braces_cnt = 0;
    for (i, c) in s_val.iter().enumerate() {
        if *c == ',' as u8 && braces_cnt == 1 {
            let left = &s_val[1..i];
            let right = &s_val[i + 1..s_val.len() - 1];
            return (&left, &right);
        } else if *c == '[' as u8 {
            braces_cnt += 1;
        } else if *c == ']' as u8 {
            braces_cnt -= 1;
        }
    }
    panic!("String is not correct");
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
    use crate::split_num;
    use std::str;

    #[test]
    fn split_num_test() {
        let (left, right) = split_num(b"[[[[[9,8],1],2],3],4]");
        // CONVERT u8 ARRAY TO STR
        let left_str = str::from_utf8(left).unwrap();
        println!("{}", left_str);
        let right_str = str::from_utf8(right).unwrap();
        println!("{}", right_str);
        assert_eq!(left, b"[[[[9,8],1],2],3]");
        assert_eq!(right, b"4");
    }
    #[test]
    fn split_num_test1() {
        let (left, right) = split_num(b"[4,[[[[9,8],1],2],3]]");
        assert_eq!(left, b"4");
        assert_eq!(right, b"[[[[9,8],1],2],3]");
    }

    #[test]
    fn explode_test() {
        // TODO
        // assert_eq!(explode("1"), "test1");
        // assert_eq!(explode("[[[[[9,8],1],2],3],4]"), "[[[[0,9],2],3],4]");
    }
}
