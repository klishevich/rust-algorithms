use std::cmp;
use std::collections::HashMap;
use std::str;

const BASE2: i32 = 2;

struct SnailfishNumber {
    left: Link,
    right: Link,
    val: u8,
    pub depth: u32,
    pub need_split: bool,
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
            need_split: false,
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
        match split_num(s_val) {
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
                self.need_split = left_snailfish.need_split || right_snailfish.need_split;
                self.left = Some(Box::new(left_snailfish));
                self.right = Some(Box::new(right_snailfish));
                return self;
            }
            None => {
                self.depth = 0;
                self.val = str::from_utf8(s_val).unwrap().parse().unwrap();
                self.need_split = self.val >= 10;
                self.left = None;
                self.right = None;
                return self;
            }
        }
    }

    pub fn sum(&mut self, s1: SnailfishNumber, s2: SnailfishNumber) -> &SnailfishNumber {
        self.depth = cmp::max(s1.depth, s2.depth) + 1;
        self.need_split = s1.need_split || s2.need_split;
        self.left = Some(Box::new(s1));
        self.right = Some(Box::new(s2));
        return self;
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
        {
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
            if left_pos_rev_vec.len() > 0 {
                self.explode_update_adjacent(left_pos_rev_vec, false, left_val);
            }
        }

        let mut right_pos_rev_vec: Vec<u8> = Vec::new();
        {
            let mut s1 = false;
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
            if right_pos_rev_vec.len() > 0 {
                self.explode_update_adjacent(right_pos_rev_vec, true, right_val);
            }
        }

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

    fn explode_update_adjacent(&mut self, mut pos_rev_vec: Vec<u8>, go_left: bool, val: u8) -> () {
        let mut go_left_dir = go_left;
        if pos_rev_vec.len() > 0 {
            let dir = pos_rev_vec.pop().unwrap();
            if dir == 0 {
                go_left_dir = true;
            } else {
                go_left_dir = false;
            }
        }
        let snailfish_opt = self.get_child(go_left_dir);
        match snailfish_opt {
            Some(snailfish) => {
                snailfish.explode_update_adjacent(pos_rev_vec, go_left, val);
                self.need_split = self.left.as_ref().unwrap().need_split
                    || self.right.as_ref().unwrap().need_split;
            }
            None => {
                self.val = self.val + val;
                if self.val > 9 {
                    self.need_split = true;
                }
            }
        }
        println!("depth {} need_split {}", self.depth, self.need_split);
    }

    pub fn split(&mut self) -> bool {
        if self.need_split == false {
            return false;
        }

        if self.depth == 0 {
            if self.val < 10 {
                panic!("something went wrong in split");
            } else {
                let left_val = ((self.val as f32) / 2.0).floor() as u8;
                println!("left_val {}", left_val);
                let need_split_left = left_val >= 10;
                let right_val = ((self.val as f32) / 2.0).ceil() as u8;
                println!("right_val {}", right_val);
                let need_split_right =  right_val >= 10;
                let left_snailfish: SnailfishNumber = SnailfishNumber {
                    left: None,
                    right: None,
                    val: left_val,
                    depth: 0,
                    need_split: need_split_left,
                };
                let right_snailfish: SnailfishNumber = SnailfishNumber {
                    left: None,
                    right: None,
                    val: right_val,
                    depth: 0,
                    need_split: need_split_right,
                };
                self.left = Some(Box::new(left_snailfish));
                self.right = Some(Box::new(right_snailfish));
                self.val = 0;
                self.depth = 1;
                self.need_split = need_split_left || need_split_right;
            }
        } else {
            let child: &mut Box<SnailfishNumber>;
            if self.left.as_ref().unwrap().need_split {
                child = self.left.as_mut().unwrap();
            } else {
                child = self.right.as_mut().unwrap();
            }
            child.split();
            self.depth = cmp::max(self.left.as_ref().unwrap().depth, self.right.as_ref().unwrap().depth) + 1;
            self.need_split = self.left.as_ref().unwrap().need_split || self.right.as_ref().unwrap().need_split;
        }
        return true;
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
            if self.val <= 9 {
                (self.val + 48) as char
            } else {
                (self.val - 10 + 97) as char
            }
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

    fn get_child(&mut self, go_left: bool) -> Option<&mut Box<SnailfishNumber>> {
        if go_left {
            return self.left.as_mut();
        } else {
            return self.right.as_mut();
        }
    }
}

fn main() {
    // EXPLODE
    // let mut snailfish = SnailfishNumber {
    //     ..Default::default()
    // };
    // snailfish.create(b"[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
    // let position = BASE2.pow(snailfish.depth) as u32;

    // let depth = snailfish.depth;
    // println!("Depth {}", depth);
    // let mut print_data_map: PrintMap = HashMap::new();
    // snailfish.get_print_data(&mut print_data_map, depth, 0, position);
    // SnailfishNumber::print(&print_data_map, depth);

    // snailfish.explode();
    // let depth2 = snailfish.depth;
    // println!("Depth2 {}", depth2);
    // let mut print_data_map2: PrintMap = HashMap::new();
    // snailfish.get_print_data(&mut print_data_map2, depth2, 0, position);
    // SnailfishNumber::print(&print_data_map2, depth2);

    // SPLIT
    // let mut snailfish = SnailfishNumber {
    //     ..Default::default()
    // };
    // snailfish.create(b"[[[[0,7],4],[15,[0,13]]],[1,1]]"); // expect [[[[0,7],4],[[7,8],[0,13]]],[1,1]]
    // let position = BASE2.pow(snailfish.depth) as u32;

    // let depth = snailfish.depth;
    // println!("Depth {}", depth);
    // let mut print_data_map: PrintMap = HashMap::new();
    // snailfish.get_print_data(&mut print_data_map, depth, 0, position);
    // SnailfishNumber::print(&print_data_map, depth);

    // snailfish.split();
    // let depth2 = snailfish.depth;
    // println!("Depth2 {}", depth2);
    // let mut print_data_map2: PrintMap = HashMap::new();
    // snailfish.get_print_data(&mut print_data_map2, depth2, 0, position);
    // SnailfishNumber::print(&print_data_map2, depth2);

    // SUM
    let mut s1 = SnailfishNumber {
        ..Default::default()
    };
    s1.create(b"[[[[4,3],4],4],[7,[[8,4],9]]]");
    let mut s2 = SnailfishNumber {
        ..Default::default()
    };
    let depth1 = s1.depth;
    println!("Depth1 {}", depth1);
    let mut print_data_map1: PrintMap = HashMap::new();
    let position1 = BASE2.pow(s1.depth) as u32;
    s1.get_print_data(&mut print_data_map1, depth1, 0, position1);
    SnailfishNumber::print(&print_data_map1, depth1);

    s2.create(b"[1,1]");
    let mut s_res = SnailfishNumber {
        ..Default::default()
    };
    let depth2 = s2.depth;
    println!("Depth2 {}", depth2);
    let mut print_data_map2: PrintMap = HashMap::new();
    let position2 = BASE2.pow(s2.depth) as u32;
    s2.get_print_data(&mut print_data_map2, depth2, 0, position2);
    SnailfishNumber::print(&print_data_map2, depth2);

    s_res.sum(s1, s2);
    let position = BASE2.pow(s_res.depth) as u32;
    let depth = s_res.depth;
    println!("Depth {}", depth);
    let mut print_data_map: PrintMap = HashMap::new();
    s_res.get_print_data(&mut print_data_map, depth, 0, position);
    SnailfishNumber::print(&print_data_map, depth);

    
}

mod test {
    use crate::split_num;
    use crate::PrintMap;
    use crate::SnailfishNumber;
    use std::collections::HashMap;
    use std::str;

    const BASE2: i32 = 2;

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
    fn explode_test1() {
        let initial_val = b"[[[[[9,8],1],2],3],4]";
        let expected_exploded_val = b"[[[[0,9],2],3],4]";
        let mut snailfish = SnailfishNumber {
            ..Default::default()
        };
        snailfish.create(initial_val);
        assert_eq!(snailfish.depth, 5);
        assert_eq!(snailfish.explode(), true);
        let depth = snailfish.depth;
        assert_eq!(depth, 4);
        let mut print_data_map: PrintMap = HashMap::new();
        let position = BASE2.pow(snailfish.depth) as u32;
        snailfish.get_print_data(&mut print_data_map, depth, 0, position);
        let mut snailfish2 = SnailfishNumber {
            ..Default::default()
        };
        snailfish2.create(expected_exploded_val);
        let depth2 = snailfish2.depth;
        assert_eq!(depth2, 4);
        let mut print_data_map2: PrintMap = HashMap::new();
        snailfish2.get_print_data(&mut print_data_map2, depth2, 0, position);
        assert_eq!(print_data_map, print_data_map2);
    }

    #[test]
    fn explode_test2() {
        let initial_val = b"[7,[6,[5,[4,[3,2]]]]]";
        let expected_exploded_val = b"[7,[6,[5,[7,0]]]]";
        let mut snailfish = SnailfishNumber {
            ..Default::default()
        };
        snailfish.create(initial_val);
        assert_eq!(snailfish.depth, 5);
        assert_eq!(snailfish.explode(), true);
        let depth = snailfish.depth;
        assert_eq!(depth, 4);
        let mut print_data_map: PrintMap = HashMap::new();
        let position = BASE2.pow(snailfish.depth) as u32;
        snailfish.get_print_data(&mut print_data_map, depth, 0, position);
        let mut snailfish2 = SnailfishNumber {
            ..Default::default()
        };
        snailfish2.create(expected_exploded_val);
        let depth2 = snailfish2.depth;
        assert_eq!(depth2, 4);
        let mut print_data_map2: PrintMap = HashMap::new();
        snailfish2.get_print_data(&mut print_data_map2, depth2, 0, position);
        assert_eq!(print_data_map, print_data_map2);
    }

    #[test]
    fn explode_test3() {
        let initial_val = b"[[6,[5,[4,[3,2]]]],1]";
        let expected_exploded_val = b"[[6,[5,[7,0]]],3]";
        let mut snailfish = SnailfishNumber {
            ..Default::default()
        };
        snailfish.create(initial_val);
        assert_eq!(snailfish.depth, 5);
        assert_eq!(snailfish.explode(), true);
        let depth = snailfish.depth;
        assert_eq!(depth, 4);
        let mut print_data_map: PrintMap = HashMap::new();
        let position = BASE2.pow(snailfish.depth) as u32;
        snailfish.get_print_data(&mut print_data_map, depth, 0, position);
        let mut snailfish2 = SnailfishNumber {
            ..Default::default()
        };
        snailfish2.create(expected_exploded_val);
        let depth2 = snailfish2.depth;
        assert_eq!(depth2, 4);
        let mut print_data_map2: PrintMap = HashMap::new();
        snailfish2.get_print_data(&mut print_data_map2, depth2, 0, position);
        assert_eq!(print_data_map, print_data_map2);
    }

    #[test]
    fn explode_test4() {
        let initial_val = b"[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]";
        let expected_exploded_val = b"[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]";
        let mut snailfish = SnailfishNumber {
            ..Default::default()
        };
        snailfish.create(initial_val);
        assert_eq!(snailfish.depth, 5);
        assert_eq!(snailfish.explode(), true);
        let depth = snailfish.depth;
        let mut print_data_map: PrintMap = HashMap::new();
        let position = BASE2.pow(snailfish.depth) as u32;
        snailfish.get_print_data(&mut print_data_map, depth, 0, position);
        let mut snailfish2 = SnailfishNumber {
            ..Default::default()
        };
        snailfish2.create(expected_exploded_val);
        let depth2 = snailfish2.depth;
        assert_eq!(depth2, depth);
        let mut print_data_map2: PrintMap = HashMap::new();
        snailfish2.get_print_data(&mut print_data_map2, depth2, 0, position);
        assert_eq!(print_data_map, print_data_map2);
    }

    #[test]
    fn explode_test5() {
        let initial_val = b"[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]";
        let expected_exploded_val = b"[[3,[2,[8,0]]],[9,[5,[7,0]]]]";
        let mut snailfish = SnailfishNumber {
            ..Default::default()
        };
        snailfish.create(initial_val);
        assert_eq!(snailfish.explode(), true);
        let depth = snailfish.depth;
        let mut print_data_map: PrintMap = HashMap::new();
        let position = BASE2.pow(snailfish.depth) as u32;
        snailfish.get_print_data(&mut print_data_map, depth, 0, position);
        let mut snailfish2 = SnailfishNumber {
            ..Default::default()
        };
        snailfish2.create(expected_exploded_val);
        let depth2 = snailfish2.depth;
        assert_eq!(depth2, depth);
        let mut print_data_map2: PrintMap = HashMap::new();
        snailfish2.get_print_data(&mut print_data_map2, depth2, 0, position);
        assert_eq!(print_data_map, print_data_map2);
    }

    #[test]
    fn split_test1() {
        let initial_val = b"[[[[0,7],4],[15,[0,13]]],[1,1]]";
        let expected_split_val = b"[[[[0,7],4],[[7,8],[0,13]]],[1,1]]";
        let mut snailfish = SnailfishNumber {
            ..Default::default()
        };
        snailfish.create(initial_val);
        assert_eq!(snailfish.split(), true);
        let depth = snailfish.depth;
        let mut print_data_map: PrintMap = HashMap::new();
        let position = BASE2.pow(snailfish.depth) as u32;
        snailfish.get_print_data(&mut print_data_map, depth, 0, position);
        let mut snailfish2 = SnailfishNumber {
            ..Default::default()
        };
        snailfish2.create(expected_split_val);
        let depth2 = snailfish2.depth;
        assert_eq!(depth2, depth);
        let mut print_data_map2: PrintMap = HashMap::new();
        snailfish2.get_print_data(&mut print_data_map2, depth2, 0, position);
        assert_eq!(print_data_map, print_data_map2);
    }

    #[test]
    fn split_test2() {
        let initial_val = b"[[[[0,7],4],[[7,8],[0,13]]],[1,1]]";
        let expected_split_val = b"[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]";
        let mut snailfish = SnailfishNumber {
            ..Default::default()
        };
        snailfish.create(initial_val);
        assert_eq!(snailfish.split(), true);
        let depth = snailfish.depth;
        let mut print_data_map: PrintMap = HashMap::new();
        let position = BASE2.pow(snailfish.depth) as u32;
        snailfish.get_print_data(&mut print_data_map, depth, 0, position);
        let mut snailfish2 = SnailfishNumber {
            ..Default::default()
        };
        snailfish2.create(expected_split_val);
        let depth2 = snailfish2.depth;
        assert_eq!(depth2, depth);
        let mut print_data_map2: PrintMap = HashMap::new();
        snailfish2.get_print_data(&mut print_data_map2, depth2, 0, position);
        assert_eq!(print_data_map, print_data_map2);
    }

    #[test]
    fn sum_test1() {
        let mut s1 = SnailfishNumber {
            ..Default::default()
        };
        s1.create(b"[[[[4,3],4],4],[7,[[8,4],9]]]");
        let mut s2 = SnailfishNumber {
            ..Default::default()
        };
    
        s2.create(b"[1,1]");
        let mut s_res = SnailfishNumber {
            ..Default::default()
        };
    
        s_res.sum(s1, s2);
        let position = BASE2.pow(s_res.depth) as u32;
        let depth = s_res.depth;
        let mut print_data_map: PrintMap = HashMap::new();
        s_res.get_print_data(&mut print_data_map, depth, 0, position);

        let mut s_exp_res = SnailfishNumber {
            ..Default::default()
        };
        s_exp_res.create(b"[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
        let mut print_res_data_map: PrintMap = HashMap::new();
        s_res.get_print_data(&mut print_res_data_map, s_exp_res.depth, 0, position);
        assert_eq!(print_data_map, print_res_data_map);
    }
}
