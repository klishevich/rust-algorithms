use std::fs;

fn main() {
    let content = fs::read_to_string("src/data-test.txt").expect("we have a bug");
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
    for row in dumbo_octopuses_matrix {
        println!("{:?}", row);
    }
}
