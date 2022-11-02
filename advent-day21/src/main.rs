use std::cmp;

// fn update_position(pos: &mut i32, points: &mut i32) {

// }

fn main() {
    let win_points = 1000;
    let die_sides = 100;
    let board_length = 10;

    let mut position1 = 6; // 4;
    let mut points1 = 0;

    let mut position2 = 7; // 8;
    let mut points2 = 0;

    let mut rolls_cnt = 0;
    let mut last_roll = 0;

    let mut roll_die_fn = || -> i32 {
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
        rolls_cnt += 3;
        if player1_turn {
            position1 = (position1 + rolls_sum) % board_length;
            if position1 == 0 {
                points1 += 10;
            } else {
                points1 += position1;
            }
        } else {
            position2 = (position2 + rolls_sum) % board_length;
            if position2 == 0 {
                points2 += 10;
            } else {
                points2 += position2;
            }
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
