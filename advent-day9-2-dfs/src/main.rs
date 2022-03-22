use std::fs;

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

    let mut max_res: Vec<i32> = vec![0; 3];
    // cannot borrow `update_max_fn` as mutable, as it is not declared as mutable
    let mut update_max_fn = |val: i32| {
        let mut index = 100;
        for (i, el) in max_res.iter().enumerate().rev() {
            if val > *el {
                index = i;
                break;
            }
        }
        if index < 100 {
            let mut i = 0;
            while i < index {
                max_res[i] = max_res[i + 1];
                i = i + 1;
            }
            max_res[index] = val;
        }
    };
    for (i, line) in array2.iter().enumerate() {
        for (j, element) in line.iter().enumerate() {
            let left = if j > 0 { line[j - 1] } else { 10 };
            let right = if j < width - 1 { line[j + 1] } else { 10 };
            let top = if i > 0 { array2[i - 1][j] } else { 10 };
            let bottom = if i < height - 1 { array2[i + 1][j] } else { 10 };
            if element < &left && element < &right && element < &top && element < &bottom {
                let cnt = my_dfs.run(&array2, i, j);
                update_max_fn(cnt);
            }
        }
    }
    println!("max_res {:?}", max_res);
    println!("res {:?}", max_res[0] * max_res[1] * max_res[2]);
}
