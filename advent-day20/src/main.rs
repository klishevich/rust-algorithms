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

fn next_matrix(
    matrix: &Vec<Vec<bool>>,
    size: usize,
    enhancement_alg: &str,
    offset: usize,
    fill: bool,
) -> Vec<Vec<bool>> {
    let mut matrix2: Vec<Vec<bool>> = vec![vec![fill; size]; size];
    for i in offset..size - offset {
        for j in offset..size - offset {
            let num = get_number(matrix, i, j);
            let ch = enhancement_alg.as_bytes().get(num).unwrap();
            if *ch == 35 {
                matrix2[i][j] = true;
            } else {
                matrix2[i][j] = false;
            }
        }
    }
    return matrix2;
}

fn main() {
    let content = fs::read_to_string("src/data01.txt").expect("some bug");
    let enhancement_alg = content.lines().next().unwrap();
    println!("enhancement algorithm {}", enhancement_alg);

    let start_image_line = 2;
    let image_size = content.lines().nth(start_image_line).unwrap().len();
    println!("image size {}", image_size);

    let iterations = 50;
    let offset = iterations + 2;
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

    let first_char = enhancement_alg.as_bytes().get(0).unwrap();
    println!("first char {}", first_char);

    let mut fill = false;
    for _k in 0..iterations {
        image_matrix = next_matrix(&image_matrix, final_image_size, enhancement_alg, 1, fill);
        if *first_char == 35 {
            fill = !fill;
            for m in 0..final_image_size {
                image_matrix[0][m] = fill;
                image_matrix[final_image_size - 1][m] = fill;
                image_matrix[m][0] = fill;
                image_matrix[m][final_image_size - 1] = fill;
            }
        }

        println!();
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
    }

    let mut res = 0;
    for i in 0..final_image_size {
        for j in 0..final_image_size {
            if image_matrix[i][j] {
                res += 1;
            }
        }
    }

    println!("res {}", res);
}
