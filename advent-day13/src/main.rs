use std::fs;

fn main() {
    let content = fs::read_to_string("src/data-real.txt").expect("we have a bug");

    let mut max_x = 0;
    let mut max_y = 0;
    for line in content.lines() {
        if line.is_empty() {
            break;
        }
        let (x_str, y_str) = line.split_once(",").unwrap();
        let x_usize: usize = x_str.parse().unwrap();
        if x_usize > max_x {
            max_x = x_usize;
        }
        let y_usize: usize = y_str.parse().unwrap();
        if y_usize > max_y {
            max_y = y_usize;
        }
    }

    // CREATE MATRIX OF FIXED SIZE
    let mut matrix = vec![vec![0; max_x + 1]; max_y + 1];
    // CREATE DYNAMIC LENGTH VECTOR
    let mut fold_vec: Vec<(char, usize)> = Vec::new();

    let mut is_first_section = true;
    for line in content.lines() {
        if line.is_empty() {
            is_first_section = false;
            continue;
        }
        if is_first_section {
            let (x_str, y_str) = line.split_once(",").unwrap();
            let x_usize: usize = x_str.parse().unwrap();
            let y_usize: usize = y_str.parse().unwrap();
            matrix[y_usize][x_usize] = 1;
        } else {
            let (fold_type_str, fold_val) = line.split_once("=").unwrap();
            let fold_type = if fold_type_str == "fold along x" { 'x' } else { 'y' };
            fold_vec.push((fold_type, fold_val.parse().unwrap()));
        }
    }

    for v in &matrix {
        println!("{:?}", v);
    }

    for v in &fold_vec {
        println!("type: {}, val: {}", v.0, v.1);
    }

    let mut cur_rows = max_y + 1;
    println!("cur_rows: {}", cur_rows);
    let mut cur_columns = max_x + 1;
    println!("cur_columns: {}", cur_columns);

    let (fold_type, fold_val) = fold_vec[0];

    if fold_type == 'y' {
        for row_index in 0..cur_rows {
            if row_index == fold_val { break; }
            let mirror_row_index = 2 * fold_val - row_index;
            if mirror_row_index > cur_rows { panic!("mirror_row_index > cur_rows"); }
            for el_index in 0..cur_columns {
                if matrix[mirror_row_index][el_index] == 1 {
                    matrix[row_index][el_index] = 1;
                }
            }
        }
        cur_rows = fold_val;
    } else if fold_type == 'x' {
        for col_index in 0..cur_columns {
            if col_index == fold_val { break; }
            let mirror_col_index = 2 * fold_val - col_index;
            if mirror_col_index > cur_columns { panic!("mirror_col_index > cur_columns"); }
            for el_index in 0..cur_rows {
                if matrix[el_index][mirror_col_index] == 1 {
                    matrix[el_index][col_index] = 1;
                }
            }
        }
        cur_columns = fold_val;
    }

    for row_index in 0..cur_rows {
        println!("{:?}", matrix[row_index]);
    }
    
    let mut res = 0;

    for i in 0..cur_rows {
        for j in 0..cur_columns {
            if matrix[i][j] == 1 {
                res += 1;
            }
        }
    }

    println!("res {}", res);
}
