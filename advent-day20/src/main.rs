use std::fs;

fn get_number(matrix: &Vec<Vec<bool>>, mi: usize, mj: usize) -> usize {
    let mut s: String = String::new();
    for i in -1..2 {
        for j in -1..2 {
            let i2 = mi as i32 + i;
            let j2 = mj as i32 + j;
            let val = matrix[i2 as usize][j2 as usize];
            if val {
                s.push('1');
            } else {
                // if s.len() != 0 {
                s.push('0');
                // }
            }
        }
    }
    let res = usize::from_str_radix(&s, 2).unwrap();
    return res;
}

fn next_matrix(matrix: &Vec<Vec<bool>>, size: usize, enhancement_alg: &str) -> Vec<Vec<bool>> {
    let mut matrix2: Vec<Vec<bool>> = vec![vec![false; size]; size];
    for i in 1..size - 1 {
        for j in 1..size - 1 {
            let num = get_number(matrix, i, j);
            let ch = enhancement_alg.as_bytes().get(num).unwrap();
            if *ch == 35 {
                matrix2[i][j] = true;
            }
        }
    }
    return matrix2;
}

fn main() {
    let content = fs::read_to_string("src/data-real.txt").expect("some bug");
    let enhancement_alg = content.lines().next().unwrap();
    println!("enhancement algorithm {}", enhancement_alg);

    let start_image_line = 2;
    let image_size = content.lines().nth(start_image_line).unwrap().len();
    println!("image size {}", image_size);

    let iterations = 2;
    let offset = iterations + 1;
    let final_image_size = image_size + 2 * offset;

    let mut image_matrix: Vec<Vec<bool>> = Vec::new();

    for _i in 0..offset {
        let row_vec = vec![false; final_image_size];
        image_matrix.push(row_vec);
    }

    for i in 0..image_size {
        let row_str = content.lines().nth(start_image_line + i).unwrap();
        let mut row_vec: Vec<bool> = Vec::new();
        for _j in 0..offset {
            row_vec.push(false);
        }
        for ch in row_str.chars() {
            if ch == '#' {
                row_vec.push(true)
            } else {
                row_vec.push(false)
            }
        }
        for _j in 0..offset {
            row_vec.push(false);
        }
        image_matrix.push(row_vec);
    }

    for _i in 0..offset {
        let row_vec = vec![false; final_image_size];
        image_matrix.push(row_vec);
    }

    for r in &image_matrix {
        for e in r {
            if *e {
                print!("1");
            } else {
                print!("0");
            }
        }
        println!("");
    }

    let m2 = next_matrix(&image_matrix, final_image_size, enhancement_alg);
    let m3 = next_matrix(&m2, final_image_size, enhancement_alg);
    for r in &m3 {
        for e in r {
            if *e {
                print!("1");
            } else {
                print!("0");
            }
        }
        println!("");
    }
    let mut res = 0;
    for row in m3 {
        for el in row {
            if el {
                res +=1;
            }
        }
    }
    println!("res {}", res);
}
