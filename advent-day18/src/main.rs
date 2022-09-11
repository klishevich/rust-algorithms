use std::cmp;
use std::collections::HashMap;
use std::str;

const BASE2: i32 = 2;

struct SnailfishNumber {
    left: Link,
    right: Link,
    val: u8,
    pub depth: u32,
}

type Link = Option<Box<SnailfishNumber>>;
type PrintMap = HashMap<u32, Vec<(u32, char)>>;

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
            val: 100,
            depth: 0,
        }
    }
}

impl SnailfishNumber {
    // STATIC METHODS
    pub fn print(data: &PrintMap, depth: u32) {
        for i in 0..depth + 1 {
            let vertices_vec = data.get(&i).unwrap();
            let mut prev: u32 = 0;
            for (pos, val) in vertices_vec {
                let spaces: usize = (*pos - prev - 1) as usize;
                print!("{: <1$}", "", spaces);
                prev = *pos;
                print!("{}", val);
            }
            println!();
        }
    }

    // INSTANCE METHODS
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
                // COMPARE CMP MIN MAX
                self.depth = cmp::max(left_snailfish.depth, right_snailfish.depth) + 1;
                self.val = 100;
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

    /**
     * Returns true if exploded
     */
    pub fn explode(&mut self) -> bool {
        if self.depth > 5 {
            panic!("The snailfish number depth can not be more than 5");
        }
        if self.depth <= 3 {
            return false;
        }

        let mut pos_vec: Vec<u8> = Vec::new();
        let mut left_val: u8 = 100;
        let mut right_val: u8 = 100;
        self.explode_get_position(&mut pos_vec, &mut left_val, &mut right_val);
        println!("left_val {}, right_val {}", left_val, right_val);
        println!("{:?}", pos_vec);

        let mut left_pos_rev_vec: Vec<u8> = Vec::new();
        let mut s1 = false;
        for el in pos_vec.iter().rev() {
            if s1 == false {
                if *el == 1 {
                    left_pos_rev_vec.push(0);
                    s1 = true;
                }
            } else {
                left_pos_rev_vec.push(*el);
            }
        }
        println!("left_pos_rev_vec {:?}", left_pos_rev_vec);
        self.explode_update_adjacent(&left_pos_rev_vec, false, left_val);

        s1 = false;
        let mut right_pos_rev_vec: Vec<u8> = Vec::new();
        for el in pos_vec.iter().rev() {
            if s1 == false {
                if *el == 0 {
                    right_pos_rev_vec.push(1);
                    s1 = true;
                }
            } else {
                right_pos_rev_vec.push(*el);
            }
        }
        println!("right_pos_rev_vec {:?}", right_pos_rev_vec);
        self.explode_update_adjacent(&right_pos_rev_vec, true, right_val);

        return true;
    }

    fn explode_get_position(
        &mut self,
        pos_vec_out: &mut Vec<u8>,
        left_val_out: &mut u8,
        right_val_out: &mut u8,
    ) -> u32 {
        // println!("self.depth {}", self.depth);
        if self.depth == 1 {
            self.depth = 0;
            self.val = 0;
            *left_val_out = self.left.as_ref().unwrap().val;
            *right_val_out = self.right.as_ref().unwrap().val;
            self.left = None;
            self.right = None;
            return 0;
        } else {
            // MULTIPLE MUT VARIABLES DECLARATION
            let (mut left_depth, mut right_depth): (u32, u32) = (
                self.left.as_ref().unwrap().depth,
                self.right.as_ref().unwrap().depth,
            );
            // println!("left_depth {}, right_depth {}", left_depth, right_depth);
            if self.left.as_ref().unwrap().depth >= self.right.as_ref().unwrap().depth {
                pos_vec_out.push(0);
                left_depth = self.left.as_mut().unwrap().explode_get_position(
                    pos_vec_out,
                    left_val_out,
                    right_val_out,
                );
            } else {
                pos_vec_out.push(1);
                right_depth = self.right.as_mut().unwrap().explode_get_position(
                    pos_vec_out,
                    left_val_out,
                    right_val_out,
                );
            }
            // println!("left_depth {}, right_depth {}", left_depth, right_depth);
            self.depth = cmp::max(left_depth, right_depth) + 1;
            return self.depth;
        }
    }

    fn explode_update_adjacent(&mut self, pos_rev_vec: &Vec<u8>, go_left: bool, val: u8) -> () {
        let mut snailfish: &mut SnailfishNumber = self;
        for el in pos_rev_vec.iter().rev() {
            if *el == 0 {
                snailfish = snailfish.left.as_mut().unwrap();
            } else {
                snailfish = snailfish.right.as_mut().unwrap();
            }
        }
        let mut has_child = snailfish.has_child();
        if go_left {
            while has_child {
                snailfish = snailfish.left.as_mut().unwrap();
            }
        } else {
            while has_child {
                snailfish = snailfish.right.as_mut().unwrap();
            }
        }
        snailfish.val = snailfish.val + val;
    }

    // REFERENCE TO MUTABLE DATA
    // IMMUTABLE SELF
    pub fn get_print_data(
        &self,
        mut print_data_out: &mut PrintMap,
        total_depth: u32,
        cur_depth: u32,
        cur_position: u32,
    ) -> () {
        // println!(
        //     "cur_depth {} self.depth {} cur_position {}",
        //     cur_depth, self.depth, cur_position
        // );
        let prt_val: char = if !self.has_child() {
            (self.val + 48) as char
        } else {
            '*'
        };
        // println!("prt_val {}", prt_val);
        print_data_out
            .entry(cur_depth)
            .or_default()
            .push((cur_position, prt_val));
        if self.has_child() {
            let shift = BASE2.pow(total_depth - cur_depth - 1) as u32;
            // println!("shift {}", shift);
            let left_position = cur_position - shift;
            let left_ref = self.left.as_ref().unwrap();
            left_ref.get_print_data(
                &mut print_data_out,
                total_depth,
                cur_depth + 1,
                left_position,
            );
            let right_position = cur_position + shift;
            let right_ref = self.right.as_ref().unwrap();
            right_ref.get_print_data(
                &mut print_data_out,
                total_depth,
                cur_depth + 1,
                right_position,
            );
        }
    }

    // PRIVATE INSTANCE METHODS
    fn has_child(&self) -> bool {
        self.depth != 0
    }
}

fn main() {
    let mut snailfish = SnailfishNumber {
        ..Default::default()
    };
    // let res = snailfish.create(b"[1,2]");
    // let res = snailfish.create(b"[1,[2,3]]");
    // let res = snailfish.create(b"[[[[[9,8],1],2],3],4]");
    snailfish.create(b"[[6,[5,[4,[3,2]]]],1]");
    // let res = snailfish.create(b"[[6,[5,[7,0]]],3]");

    let position = BASE2.pow(snailfish.depth) as u32;

    let depth = snailfish.depth;
    println!("Depth {}", depth);
    let mut print_data_map: PrintMap = HashMap::new();
    snailfish.get_print_data(&mut print_data_map, depth, 0, position);
    SnailfishNumber::print(&print_data_map, depth);

    snailfish.explode();
    let depth2 = snailfish.depth;
    println!("Depth2 {}", depth2);
    let mut print_data_map2: PrintMap = HashMap::new();
    snailfish.get_print_data(&mut print_data_map2, depth2, 0, position);
    SnailfishNumber::print(&print_data_map2, depth2);
}

mod test {
    use crate::split_num;
    use crate::SnailfishNumber;
    use std::str;

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
