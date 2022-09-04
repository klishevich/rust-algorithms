use std::cmp;
use std::str;

struct SnailfishNumber {
    left: Link,
    right: Link,
    val: i32,
    pub depth: i32,
}

type Link = Option<Box<SnailfishNumber>>;

// SPLIT STRING ARRAY OF BYTES
fn split_num(s_val: &[u8]) -> Option<(&[u8], &[u8])> {
    // let (s_val, s_size) = s;
    let mut braces_cnt = 0;
    for (i, c) in s_val.iter().enumerate() {
        if *c == ',' as u8 && braces_cnt == 1 {
            let left = &s_val[1..i];
            let right = &s_val[i + 1..s_val.len() - 1];
            return Some((&left, &right));
        } else if *c == '[' as u8 {
            braces_cnt += 1;
        } else if *c == ']' as u8 {
            braces_cnt -= 1;
        }
    }
    return None;
}

impl Default for SnailfishNumber {
    fn default() -> Self {
        SnailfishNumber {
            left: None,
            right: None,
            val: -1,
            depth: -1,
        }
    }
}

impl SnailfishNumber {
    pub fn create(&mut self, s_val: &[u8]) -> &SnailfishNumber {
        let split = split_num(s_val);
        match split {
            Some((left, right)) => {
                let mut left_snailfish = SnailfishNumber {
                    ..Default::default()
                };
                left_snailfish.create(left);
                let mut right_snailfish = SnailfishNumber {
                    ..Default::default()
                };
                right_snailfish.create(right);
                self.depth = cmp::max(left_snailfish.depth, right_snailfish.depth) + 1;
                self.val = -1;
                self.left = Some(Box::new(left_snailfish));
                self.right = Some(Box::new(right_snailfish));
                return self;
            }
            None => {
                self.depth = 0;
                self.val = str::from_utf8(s_val).unwrap().parse().unwrap();
                self.left = None;
                self.right = None;
                return self;
            }
        }
    }
}

fn explode(num: &str) -> String {
    // NEW STRING FROM
    let a = String::from("test");
    let b = a + num;
    return b;
}

fn main() {
    println!("Hello, world!");
    let mut snailfish = SnailfishNumber {
        ..Default::default()
    };
    let res = snailfish.create(b"[[[[[9,8],1],2],3],4]");
    println!("Depth {}", res.depth);
}

mod test {
    use std::str;
    use crate::explode;
    use crate::split_num;
    use crate::SnailfishNumber;

    #[test]
    fn split_num_test() {
        let (left, right) = split_num(b"[[[[[9,8],1],2],3],4]").unwrap();
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
        let (left, right) = split_num(b"[4,[[[[9,8],1],2],3]]").unwrap();
        assert_eq!(left, b"4");
        assert_eq!(right, b"[[[[9,8],1],2],3]");
    }

    #[test]
    fn snailfish_number_depth_test() {
        let mut snailfish = SnailfishNumber {
            ..Default::default()
        };
        let res = snailfish.create(b"[1,2]");
        assert_eq!(res.depth, 1);
    }

    #[test]
    fn snailfish_number_depth_test1() {
        let mut snailfish = SnailfishNumber {
            ..Default::default()
        };
        let res = snailfish.create(b"[1,[2,3]]");
        assert_eq!(res.depth, 2);
    }

    #[test]
    fn snailfish_number_depth_test2() {
        let mut snailfish = SnailfishNumber {
            ..Default::default()
        };
        let res = snailfish.create(b"[[[[0,9],2],3],[4,5]]");
        assert_eq!(res.depth, 4);
    }

    #[test]
    fn explode_test() {
        // TODO
        // assert_eq!(explode("1"), "test1");
        // assert_eq!(explode("[[[[[9,8],1],2],3],4]"), "[[[[0,9],2],3],4]");
    }
}
