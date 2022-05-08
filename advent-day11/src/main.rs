use std::fs;

fn main() {
    let content = fs::read_to_string("src/data-real.txt").expect("we have a bug");
    let mut dumbo_octopuses_matrix: Vec<Vec<u8>> = Vec::new(); 
    for line in content.lines() {
        let mut column_vec: Vec<u8> = Vec::new();
        // BAD: dumbo_octopuses_matrix.push(column_vec);
        for c in line.chars() {
            let num_option = c.to_digit(10);
            if num_option == None {
                panic!("all values should be numbers!");
            } else {
                let num = num_option.unwrap();
                let try_num_u8 = u8::try_from(num);
                match try_num_u8 {
                    Ok(v) => {
                        column_vec.push(v);
                    }
                    Err(e) => {
                        panic!("Value is not u8!")
                    }
                };
            }
        }
        // GOOD
        dumbo_octopuses_matrix.push(column_vec);
        // println!("{}", line);
    }

    let rows = dumbo_octopuses_matrix.len() as i32;
    if rows == 0 {
        panic!("Number of rows should not be zero!")
    }
    let cols = dumbo_octopuses_matrix[0].len() as i32;
    println!("ROWS {}, COLS {}", rows, cols);

    let mut flashes = 0;

    for _ in 0..100 {
        let mut flashed = false;
        for (_, row) in dumbo_octopuses_matrix.iter_mut().enumerate() {
            for (_, el) in row.iter_mut().enumerate() {
                if *el == 9 {
                    *el = 0;
                    flashed = true;
                    flashes += 1;
                } else {
                    *el = *el + 1;
                }
            }
        }
        println!();
        for row in &dumbo_octopuses_matrix {
            println!("{:?}", row);
        }
    
        let mut loop_cnt = 0;
        let mut is_last_loop = false;
        loop {
            if flashed {
                flashed = false;
            } else {
                if !is_last_loop {
                    is_last_loop = true;
                } else {
                    break;
                }
            }
    
            loop_cnt +=1;
            println!("Loop {}", loop_cnt);
    
            // COUNT ADJACENT FLASHES
            let mut adjacent_flashed_cnt_vec: Vec<(usize, usize, u8)> = Vec::new();
            for (row_index, row) in dumbo_octopuses_matrix.iter().enumerate() {
                for (el_index, el) in row.iter().enumerate() {
                    if *el != 0 {
                        // println!("-- row_index {}, el_index {} --", row_index, el_index);
                        let mut adjacent_flashed: u8 = 0;
                        for i in -1..2 {
                            for j in -1..2 {
                                if i != 0 || j != 0 {
                                    // println!("---- i {}, j {} ----", i, j);
                                    let adj_row = row_index as i32 + i;
                                    let adj_el = el_index as i32 + j;
                                    // println!("------ adj_row {}, adj_el {} ------", adj_row, adj_el);
                                    if adj_row >=0 && adj_row < rows && adj_el >= 0 && adj_el < cols {
                                        // println!("dumbo_octopuses_matrix val {}", dumbo_octopuses_matrix[adj_row as usize][adj_el as usize]);
                                        if dumbo_octopuses_matrix[adj_row as usize][adj_el as usize] == 0 {
                                            adjacent_flashed += 1;
                                        }
                                    }
                                }
                            }
                        }
                        if adjacent_flashed != 0 {
                            // println!("-- adjacent_flashed row {}, col {}, adjacent flashed {} --", row_index, el_index, adjacent_flashed);
                            adjacent_flashed_cnt_vec.push((row_index, el_index, adjacent_flashed));
                        }
                    }
                }
            }
    
            // UPDATE CELLS
            for (row_index, el_index, adjacent_flashed) in adjacent_flashed_cnt_vec {
                let cur_value = dumbo_octopuses_matrix[row_index][el_index];
                if is_last_loop {
                    dumbo_octopuses_matrix[row_index][el_index] = adjacent_flashed + cur_value;
                } else {
                    if cur_value != 0 && adjacent_flashed + cur_value > 9 {
                        dumbo_octopuses_matrix[row_index][el_index] = 0;
                        flashes += 1;
                        flashed = true;
                    }
                }
                
            }
        }
    }

    println!("\n---");
    for row in &dumbo_octopuses_matrix {
        println!("{:?}", row);
    }

    println!("Flashes {}", flashes);
}
