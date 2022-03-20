use std::fs;
use std::vec;
use uuid::Uuid;
use std::collections::HashSet;

struct Group {
    sum: i32,
    intervals: Vec<(i32,i32)>,
    merge_group_id: Uuid
}

impl Group {
    fn intersect(&self, intervals: &Vec<(i32,i32)>) -> bool {
        let test_intersect_interval = |interval_begin: i32, interval_end: i32| -> bool {
            let mut s_last_interval_end: i32 = -1;
            for s_interval in &self.intervals {
                let s_interval_begin = s_interval.0;
                if s_interval_begin > interval_end {
                    break;
                }
                s_last_interval_end = s_interval.1;
            }
            if s_last_interval_end == -1 {
                return false;
            } else if s_last_interval_end >= interval_begin {
                return true;
            } else {
                return false;
            }
        };

        for interval in intervals {
            if test_intersect_interval(interval.0, interval.1) {
                return true;
            }
        }
        
        return false;
    }
}

fn get_groups(v: &Vec<u32>) -> Vec<Group> {
    let mut groups_vec: Vec<Group> = vec![];
    let mut start: i32 = -1;
    for (i, n) in v.iter().enumerate() {
        let index = i as i32;
        if *n == 9 {
            if start != -1 {
                groups_vec.push(Group {
                    sum: index - start,
                    intervals: vec![(start, index - 1)],
                    merge_group_id: Uuid::new_v4()
                });
                start = -1;
            }
        } else {
            if start == -1 {
                start = index;
            }
        }
    }
    if start != -1 {
        let end = v.len() as i32; 
        groups_vec.push(Group {
            sum: end - start,
            intervals: vec![(start, end - 1)],
            merge_group_id: Uuid::new_v4()
        });
    }
    return groups_vec;
}

/**
 * Return tuple with finished and next groups
 */
fn merge_groups(prev: &Vec<Group>, mut curr: Vec<Group>) -> (Vec<Group>, Vec<Group>) {
    let mut finished_groups_vec: Vec<Group> = vec![];
    let mut next_groups_vec: Vec<Group> = vec![];
    for (i, curr_group) in curr.iter_mut().enumerate() {
        for (j, prev_group) in prev.iter().enumerate() {
            if prev_group.intersect(&curr_group.intervals) {
                curr_group.merge_group_id = prev_group.merge_group_id;
            }
        }
    }

    // let prev_ids: Vec<Uuid> = prev.into_iter().map(|prev_group| prev_group.merge_group_id).collect::<HashSet<_>>().into_iter().collect();
    let mut prev_id: Uuid = Uuid::new_v4();
    let mut prev_sum:i32 = -1;
    let mut gr_accum: Group;
    // for (i, curr_group) in curr.iter().enumerate() {
    //     let curr_id = curr_group.merge_group_id;
    //     if curr_id != prev_id {
    //         let mut intervals: Vec<(i32, i32)> = vec!();
    //         for inter in curr_group.intervals {
    //             intervals.push(inter);
    //         }
    //         gr_accum = Group {
    //             sum: curr_group.sum,
    //             intervals: curr_group.intervals,
    //             merge_group_id: Uuid::new_v4()
    //         }
    //     }
    // }

    for (i, grp) in curr.iter().enumerate() {
        println!("Group {}, id {}", i, grp.merge_group_id);
    }
    return (finished_groups_vec, curr);
}

fn main() {
    let filename = "src/data-test0.txt";
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let height = content.lines().count();
    let width = content.lines().next().unwrap().chars().count();
    let mut array2: Vec<Vec<u32>> = vec![vec![0; width]; height];

    for (i, line) in content.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            array2[i][j] = c.to_digit(10).unwrap();
        }
    }

    for v in &array2 {
        println!("{:?}", v);
    }
 
    let mut prev_groups: Vec<Group> = vec![];
    let mut groups: Vec<Group>;
    for (i, line) in array2.iter().enumerate() {
        groups = get_groups(line);
        println!("Group {}", i);
        for e in &groups {
            println!("Sum {}, start {}, end {}", e.sum, e.intervals[0].0, e.intervals[0].1);
        }
        if i > 0 {
            println!("Merged");
            let res = merge_groups(&prev_groups, groups);
            prev_groups = res.1;
            // for e in res.1 {
            //     println!("Sum {}, start {}, end {}", e.sum, e.intervals[0].0, e.intervals[0].1);
            // }
        }
        //let b = &mut *groups; // a isn't available from now on
    }

}
