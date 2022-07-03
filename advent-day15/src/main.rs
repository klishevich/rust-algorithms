use std::fs;
use std::collections::HashMap;

type Matrix = Vec<Vec<i32>>;

struct ChitonsDfs {
    best_path_score: i32,
    rows: i32,
    cols: i32,
    visited: HashMap<i32, i32>
}

impl ChitonsDfs {
    pub fn run(&mut self, m: &Matrix, r: i32, c: i32, score: i32) -> () {
        let mut exit = false;
        match self.visited.get(&self.get_key(r, c)) {
            Some(val) => {
                if *val <= score {
                    exit = true;
                } else {
                    self.visited.insert(self.get_key(r, c), score);
                }
            },
            None => {
                self.visited.insert(self.get_key(r, c), score);
            }
        };
        if exit {
            return;
        }

        let r_usize: usize = r.try_into().unwrap();
        let c_usize: usize = c.try_into().unwrap(); 
        let new_score: i32 = score + m[r_usize][c_usize];
        if r == self.rows - 1 && c == self.cols - 1 {
            if new_score < self.best_path_score {
                self.best_path_score = new_score;
            }
            return ();
        }

        if score > self.best_path_score {
            return ();
        }
        
        let c_left = c - 1;
        let c_right = c + 1;
        let r_top = r - 1;
        let r_bottom = r + 1;

        if c_left >= 0 {
            ChitonsDfs::run(self, m, r, c_left, new_score);
        }
        if c_right < self.cols {
            ChitonsDfs::run(self, m, r, c_right, new_score);
        }
        if r_top >= 0 {
            ChitonsDfs::run(self, m, r_top, c, new_score);
        }
        if r_bottom < self.rows {
            ChitonsDfs::run(self, m, r_bottom, c, new_score);
        }
    }

    fn get_key(&self, r: i32, c: i32) -> i32 {
        return r * self.rows + c;
    }
}

fn main() {
    println!("Hello, world!");
    let content = fs::read_to_string("src/data-real.txt").expect("some bug");
    let matrix_rows = content.lines().count();
    let matrix_cols = content.lines().next().unwrap().chars().count();
    let mut matrix: Matrix = vec![vec![0; matrix_cols]; matrix_rows];

    for (index_line, line) in content.lines().enumerate() {
        for (index_ch, ch) in line.chars().enumerate() {
            // TRY_FROM
            // let val: usize = usize::try_from(ch.to_digit(10).unwrap()).unwrap();
            // TRY_INTO
            let val: i32 = ch.to_digit(10).unwrap().try_into().unwrap();
            matrix[index_line][index_ch] = val;
        }
    }

    for row in &matrix {
        println!("{:?}", row);
    }

    let mut chitons_dfs = ChitonsDfs {
        best_path_score: 600,
        rows: matrix_rows.try_into().unwrap(),
        cols: matrix_cols.try_into().unwrap(),
        visited: HashMap::new()
    };

    chitons_dfs.run(&matrix, 0, 0, 0);

    println!("result {}", chitons_dfs.best_path_score);
}
