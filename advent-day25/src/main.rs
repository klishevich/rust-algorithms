use std::fs;

fn next_state(matrix: &mut Vec<Vec<u8>>) -> bool {
    let mut is_updated = false;

    let rows_cnt = matrix.len();
    let cols_cnt = matrix[0].len();

    for i in 0..rows_cnt {
        let mut skip_next_j = false;
        let first_value = matrix[i][0];
        for j in 0..cols_cnt {
            if skip_next_j {
                skip_next_j = false;
                continue;
            }
            let next_j = if j < cols_cnt - 1 { j + 1 } else { 0 };
            let next_value = if j < cols_cnt - 1 { matrix[i][j + 1] } else { first_value };
            if matrix[i][j] == 1 && next_value == 0 {
                is_updated = true;
                matrix[i][j] = 0;
                matrix[i][next_j] = 1;
                skip_next_j = true;
            }
        }
    }

    for j in 0..cols_cnt {
        let mut skip_next_i = false;
        let first_value = matrix[0][j];
        for i in 0..rows_cnt {
            if skip_next_i {
                skip_next_i = false;
                continue;
            }
            let next_i = if i < rows_cnt - 1 { i + 1 } else { 0 };
            let next_value = if i < rows_cnt - 1 { matrix[i+1][j]} else { first_value };
            if matrix[i][j] == 2 && next_value == 0 {
                is_updated = true;
                matrix[i][j] = 0;
                matrix[next_i][j] = 2;
                skip_next_i = true;
            }
        }
    }

    return is_updated;
}

fn print_matrix(matrix: &Vec<Vec<u8>>) {
    println!("-------------------------");
    for r in matrix {
        for &e in r {
            if e == 2 {
                print!("v");
            } else if e == 1 {
                print!(">");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn main() {
    let content = fs::read_to_string("src/data-real.txt").expect("some bug");
    let mut matrix: Vec<Vec<u8>> = Vec::new();
    for (i, line) in content.lines().enumerate() {
        let mut v: Vec<u8> = Vec::new();
        for (j, el) in line.chars().enumerate() {
            if el == '>' {
                v.push(1);
            } else if el == 'v' {
                v.push(2);
            } else {
                v.push(0)
            }
        }
        matrix.push(v);
    }

    // print_matrix(&matrix);

    let mut steps = 0;
    while next_state(&mut matrix) {
        steps += 1;
    }
    // print_matrix(&matrix);
    println!("steps {}", steps);

}
