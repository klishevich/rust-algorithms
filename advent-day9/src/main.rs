use std::fs;

fn main() {
    let filename = "src/data-test0.txt";
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let height = content.lines().count();
    let width = content.lines().next().unwrap().chars().count();
    let mut array2: Vec<Vec<u32>> = vec![vec![0; width]; height];

    for (i, line) in content.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            array2[i][j] = c.to_digit(10).unwrap();
        }
    }

    for v in &array2 {
        println!("{:?}", v);
    }

    let mut res_sum: u32 = 0;
 
    for (i, line) in array2.iter().enumerate() {
        for (j, element) in line.iter().enumerate() {
            let left = if j > 0 { line[j-1] } else { 10 };
            let right = if j < width - 1 { line[j+1] } else { 10 };
            let top = if i > 0 { array2[i-1][j] } else { 10 };
            let bottom = if i < height - 1 { array2[i+1][j] } else { 10 };
            if element < &left && element < &right && element < &top && element < &bottom {
                res_sum += element + 1;
            }
        }
    }

    println!("result sum {}", res_sum);

}
