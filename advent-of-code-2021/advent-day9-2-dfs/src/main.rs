use std::fs;
use priority_queue::PriorityQueue;

struct MyDfs {
    width: usize,
    height: usize,
    visited: Vec<Vec<bool>>,
}

impl MyDfs {
    pub fn run(&mut self, array2: &Vec<Vec<u32>>, i: usize, j: usize) -> i32 {
        let mut res: i32 = 0;
        if self.visited[i][j] || array2[i][j] == 9 {
            return res;
        } else {
            res += 1;
            self.visited[i][j] = true;
        }
        if j > 0 {
            res += MyDfs::run(self, array2, i, j - 1);
        }
        if j < self.width - 1 {
            res += MyDfs::run(self, array2, i, j + 1);
        }
        if i > 0 {
            res += MyDfs::run(self, array2, i - 1, j);
        }
        if i < self.height - 1 {
            res += MyDfs::run(self, array2, i + 1, j);
        }
        return res;
    }
}

fn main() {
    let filename = "src/data-real.txt";
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let height = content.lines().count();
    let width = content.lines().next().unwrap().chars().count();
    let mut array2: Vec<Vec<u32>> = vec![vec![0; width]; height];
    let visited: Vec<Vec<bool>> = vec![vec![false; width]; height];

    for (i, line) in content.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            array2[i][j] = c.to_digit(10).unwrap();
        }
    }

    let mut my_dfs = MyDfs {
        width,
        height,
        visited,
    };

    let mut pq = PriorityQueue::new();


    for (i, line) in array2.iter().enumerate() {
        for (j, element) in line.iter().enumerate() {
            let left = if j > 0 { line[j - 1] } else { 10 };
            let right = if j < width - 1 { line[j + 1] } else { 10 };
            let top = if i > 0 { array2[i - 1][j] } else { 10 };
            let bottom = if i < height - 1 { array2[i + 1][j] } else { 10 };
            if element < &left && element < &right && element < &top && element < &bottom {
                let cnt = my_dfs.run(&array2, i, j);
                pq.push(cnt, -cnt);
                if pq.len() > 3 {
                    pq.pop();
                }
            }
        }
    }

    println!("pq len {}", pq.len());
    let result = pq.iter().fold(1, |a, (c, _)| a * c);
    println!("res {:?}", result);
}
