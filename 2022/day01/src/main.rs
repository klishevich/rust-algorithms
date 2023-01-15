use std::fs;

fn main() {
    // println!("Hello, world!");
    let content = fs::read_to_string("src/data-real.txt").expect("some bug");
    let mut matrix: Vec<Vec<u32>> = Vec::new();
    let mut v: Vec<u32> = Vec::new();
    for el in content.lines() {
        if el == "" {
            matrix.push(v);
            v = Vec::new();
        } else {
            let t: u32 = el.parse().unwrap();
            v.push(t);
        }
        println!("{}", el);
    }
    matrix.push(v);
    println!("{:?}", matrix);
    let mut max1: u32 = 0;
    let mut max2: u32 = 0;
    let mut max3: u32 = 0;
    for m in matrix {
        let mut n_sum: u32 = 0;
        for n in m {
            n_sum += n;
        }
        println!("n_sum {}", n_sum);
        if (n_sum > max1) {
            max3 = max2;
            max2 = max1;
            max1 = n_sum;
        } else if (n_sum > max2) {
            max3 = max2;
            max2 = n_sum;
        } else if (n_sum > max3) {
            max3 = n_sum;
        }
    }
    println!("{}", max1 + max2 + max3);
}
