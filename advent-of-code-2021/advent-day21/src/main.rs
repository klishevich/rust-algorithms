use std::cmp;

type PosSumVar = Vec<Vec<u64>>;
const board: usize = 10;
const win_sum: usize = 21;

fn update_position(pos: &mut i32, points: &mut i32, rolls_sum: i32, board_length: i32) -> () {
    *pos = (*pos + rolls_sum) % board_length;
    if *pos == 0 {
        *points += 10;
    } else {
        *points += *pos;
    }
}

fn part1() {
    let win_points = 1000;
    let die_sides = 100;
    let board_length = 10;

    let mut position1 = 4;
    let mut points1 = 0;

    let mut position2 = 8;
    let mut points2 = 0;

    let mut rolls_cnt = 0;
    let mut last_roll = 0;

    let mut roll_die_fn = || -> i32 {
        rolls_cnt += 1;
        if last_roll < die_sides {
            last_roll = last_roll + 1;
        } else {
            last_roll = 1;
        }
        return last_roll;
    };

    let mut player1_turn: bool = false;
    while points1 < win_points && points2 < win_points {
        player1_turn = !player1_turn;
        let rolls_sum = roll_die_fn() + roll_die_fn() + roll_die_fn();
        if player1_turn {
            update_position(&mut position1, &mut points1, rolls_sum, board_length);
        } else {
            update_position(&mut position2, &mut points2, rolls_sum, board_length);
        }
    }
    println!("rolls cnt {}", rolls_cnt);
    println!("player 1 winner {}", player1_turn);
    println!("position 1 player {}", position1);
    println!("player 1 points {}", points1);
    println!("position 2 player {}", position2);
    println!("player 2 points {}", points2);
    let loser_points = cmp::min(points1, points2);
    println!("result {}", loser_points * rolls_cnt);
}

fn get_next_dp(m: PosSumVar) -> PosSumVar {
    let num_var_vec: Vec<(usize, u64)> =
        vec![(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
    let mut res: PosSumVar = vec![vec![0; win_sum + 1]; board + 1];
    for p in 1..board + 1 {
        for s in 0..win_sum {
            let v = m[p][s];
            if v > 0 {
                for (roll_num, roll_var) in &num_var_vec {
                    let mut p_new = p + roll_num;
                    if p_new > board {
                        p_new = p_new % board;
                    }
                    let mut s_new = s + p_new;
                    if s_new >= win_sum {
                        s_new = win_sum;
                    }
                    res[p_new][s_new] += v * roll_var;
                }
            }
        }
    }
    return res;
}

fn get_win(m: &PosSumVar) -> u64 {
    let mut win: u64 = 0;
    for p in 1..board + 1 {
        win += m[p][win_sum];
    }
    return win;
}

fn get_lose(m: &PosSumVar) -> u64 {
    let mut lose: u64 = 0;
    for p in 1..board + 1 {
        for s in 0..win_sum {
            lose += m[p][s];
        }
    }
    return lose;
}

fn print_m(m: &PosSumVar) -> () {
    // Print out dp matrix
    for i in 1..board + 1 {
        for j in 0..win_sum + 1 {
            if m[i][j] == 0 {
                print!("  . ");
            } else {
                // FORMAT right-justify
                print!("{:>3} ", m[i][j]);
            }
        }
        println!();
    }
}

fn main() {
    //    part1();
    let mut dp1: PosSumVar = vec![vec![0; win_sum + 1]; board + 1];
    dp1[6][0] = 1;

    let mut dp2: PosSumVar = vec![vec![0; win_sum + 1]; board + 1];
    dp2[7][0] = 1;

    let mut res_win1: u64 = 0;
    let mut res_win2: u64 = 0;

    for i in 0..11 {
        println!("step {}", i);
        dp1 = get_next_dp(dp1);
        println!("dp1");
        print_m(&dp1);
        let win1 = get_win(&dp1);
        println!("win1 {}", win1);
        let lose2 = get_lose(&dp2);
        println!("lose2 {}", lose2);
        res_win1 += win1 * lose2;
        dp2 = get_next_dp(dp2);
        print_m(&dp2);
        let win2 = get_win(&dp2);
        println!("win2 {}", win2);
        let lose1 = get_lose(&dp1);
        println!("lose1 {}", lose1);
        res_win2 += win2 * lose1;
    }

    println!("win1 {}, win2 {}", res_win1, res_win2);
}
