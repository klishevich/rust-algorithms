use std::collections::HashMap;
use std::fs;

const UNDEF_VAR: usize = 666;
const UNDEF_I: i64 = 777;

// bool is var
type TCommandTypeVar1Var2Val2 = (u8, usize, usize, i64);

fn parse_var(s: &str) -> (usize, i64) {
    match s {
        "w" => (0, UNDEF_I),
        "x" => (1, UNDEF_I),
        "y" => (2, UNDEF_I),
        "z" => (3, UNDEF_I),
        other => (UNDEF_VAR, other.parse().unwrap()),
    }
}

fn read_alu_commands_from_file(file_name: &str) -> Vec<TCommandTypeVar1Var2Val2> {
    let type_map: HashMap<&str, u8> = HashMap::from([
        ("inp", 1),
        ("add", 2),
        ("mul", 3),
        ("div", 4),
        ("mod", 5),
        ("eql", 6),
    ]);
    let content = fs::read_to_string(file_name).expect("some bug");
    let mut res: Vec<TCommandTypeVar1Var2Val2> = Vec::new();
    for line in content.lines() {
        let mut elements = line.split(" ").peekable();
        let t_str = elements.next().unwrap();
        let t: u8 = match type_map.get(t_str) {
            Some(v) => *v,
            None => 0,
        };
        let (var_a, _val_a) = parse_var(elements.next().unwrap());
        let mut var_b: usize = UNDEF_VAR;
        let mut val_b: i64 = UNDEF_I;
        if !elements.peek().is_none() {
            (var_b, val_b) = parse_var(elements.next().unwrap());
        }
        res.push((t, var_a, var_b, val_b));
    }
    return res;
}

fn get_b_val(var_vec: &Vec<i64>, var_b: &usize, val_b: &i64) -> i64 {
    if *var_b == UNDEF_VAR {
        return *val_b;
    }
    return var_vec[*var_b];
}

fn exec_command(var_vec: &mut Vec<i64>, cmd: &TCommandTypeVar1Var2Val2, inp: i64) -> () {
    let (cmd_type, var_a, var_b, val_b) = cmd;
    match *cmd_type {
        1 => {
            var_vec[*var_a] = inp;
        }
        2 => {
            var_vec[*var_a] += get_b_val(var_vec, var_b, val_b);
        }
        3 => var_vec[*var_a] *= get_b_val(var_vec, var_b, val_b),
        4 => {
            var_vec[*var_a] = var_vec[*var_a] / get_b_val(var_vec, var_b, val_b);
        }
        5 => {
            var_vec[*var_a] = var_vec[*var_a] % get_b_val(var_vec, var_b, val_b);
        }
        6 => {
            if var_vec[*var_a] == get_b_val(var_vec, var_b, val_b) {
                var_vec[*var_a] = 1;
            } else {
                var_vec[*var_a] = 0;
            }
        }
        _ => {
            panic!("something went wrong on exec_command!")
        }
    };
    // println!("-- {:?}", var_vec);
}

fn exec_all_commands(
    var_vec: &mut Vec<i64>,
    inp_vec: &Vec<i64>,
    commands: &Vec<TCommandTypeVar1Var2Val2>,
) {
    let mut inp_idx = 0;
    for cmd in commands {
        if cmd.0 == 1 {
            let inp = inp_vec[inp_idx];
            inp_idx += 1;
            exec_command(var_vec, cmd, inp);
        } else {
            exec_command(var_vec, cmd, 0);
        }
    }
}

fn exec_all_commands2(
    var_vec: &mut Vec<i64>,
    inp_num: u64,
    commands: &Vec<TCommandTypeVar1Var2Val2>,
) {
    let inp_num2 = inp_num.to_string();
    let mut inp_chars_iter = inp_num2.chars();

    for cmd in commands {
        if cmd.0 == 1 {
            let cc = inp_chars_iter.next().unwrap();
            let inp: i64 = cc.to_digit(10).unwrap().try_into().unwrap();
            exec_command(var_vec, cmd, inp);
        } else {
            exec_command(var_vec, cmd, 0);
        }
    }
}

fn get_vectors(digit: u8, base: &Vec<i64>) -> Vec<Vec<i64>> {
    let mut res: Vec<Vec<i64>> = Vec::new();
    for i in 1..10 {
        let mut v: Vec<i64> = Vec::new();
        for j in 1..15 {
            if j == digit {
                v.push(i);
            } else {
                v.push(base[usize::try_from(j - 1).unwrap()]);
            }
        }
        res.push(v);
    }
    return res;
}

fn main() {
    let commands = read_alu_commands_from_file("src/data-real.txt");
    let max: u64 = 99999999999999;
    // for i in 0..10 {
    //     let j = max - i;
    //     let mut var_vec: Vec<i64> = vec![0; 4];
    //     exec_all_commands2(&mut var_vec, j, &commands);
    //     if var_vec[3] == 1 {
    //         println!("res {}", j);
    //     }
    //     // if i % 1000000 == 0 {
    //     //     println!("mln {}", i / 1000000);
    //     // }
    //     println!("res {}", var_vec[3]);
    // }

    // println!("-- 1 --");
    // for i in 11111111111111..91111111111111 {
    //     let mut var_vec: Vec<i64> = vec![0; 4];
    //     exec_all_commands2(&mut var_vec, i, &commands);
    //     println!("res {}", var_vec[3]);
    // }

    
    for pp in 1..2 {
        println!("---- pp {} -----", pp);
        // let base:     Vec<i64> = v[1, 2, 3, 4, 5, 6, 7, 8, 9,10,11,12,13,14];
        let mut base: Vec<i64> = vec![1, 1, 9, 1, 3, 8, 9, 9, 9, 7, 9, 5, 1, 1];
        // base[7]=pp;
        for k in 1..15 {
            println!("k {}", k);
            let vv = get_vectors(k, &base);
            // for v in &vv {
            //     println!("{:?}", v);
            // }
            for (i_v, v) in vv.iter().enumerate() {
                let mut var_vec: Vec<i64> = vec![0; 4];
                exec_all_commands(&mut var_vec, &v, &commands);
                println!("res {}: {}", i_v + 1, var_vec[3]);
            }
        }
    }
}

mod test {
    use crate::exec_all_commands;
    use crate::TCommandTypeVar1Var2Val2;
    use crate::UNDEF_VAR;

    #[test]
    fn test1() {
        let mut var_vec: Vec<i64> = vec![0; 4];
        let mut inp: Vec<i64> = vec![4];
        let commands: Vec<TCommandTypeVar1Var2Val2> =
            vec![(1, 0, UNDEF_VAR, 0), (3, 0, UNDEF_VAR, -1)];
        exec_all_commands(&mut var_vec, &mut inp, &commands);
        assert_eq!(var_vec[0], -4);
    }

    #[test]
    fn test2() {
        let mut var_vec: Vec<i64> = vec![0; 4];
        let mut inp: Vec<i64> = vec![1, 3];
        let commands: Vec<TCommandTypeVar1Var2Val2> = vec![
            (1, 3, UNDEF_VAR, 0),
            (1, 1, UNDEF_VAR, 0),
            (3, 3, UNDEF_VAR, 3),
            (6, 3, 1, 0),
        ];
        exec_all_commands(&mut var_vec, &mut inp, &commands);
        assert_eq!(var_vec[3], 1);
    }

    #[test]
    fn test2_1() {
        let mut var_vec: Vec<i64> = vec![0; 4];
        let mut inp: Vec<i64> = vec![3, 1];
        let commands: Vec<TCommandTypeVar1Var2Val2> = vec![
            (1, 3, UNDEF_VAR, 0),
            (1, 1, UNDEF_VAR, 0),
            (3, 3, UNDEF_VAR, 3),
            (6, 3, 1, 0),
        ];
        exec_all_commands(&mut var_vec, &mut inp, &commands);
        assert_eq!(var_vec[3], 0);
    }

    #[test]
    fn test3() {
        let mut var_vec: Vec<i64> = vec![0; 4];
        let mut inp: Vec<i64> = vec![15];
        let commands: Vec<TCommandTypeVar1Var2Val2> = vec![
            (1, 0, UNDEF_VAR, 0),
            (2, 3, 0, 0),
            (5, 3, UNDEF_VAR, 2),
            (4, 0, UNDEF_VAR, 2),
            (2, 2, 0, 2),
            (5, 2, UNDEF_VAR, 2),
            (4, 0, UNDEF_VAR, 2),
            (2, 1, 0, 0),
            (5, 1, UNDEF_VAR, 2),
            (4, 0, UNDEF_VAR, 2),
            (5, 0, UNDEF_VAR, 2),
        ];
        exec_all_commands(&mut var_vec, &mut inp, &commands);
        println!("{:?}", var_vec);
        assert_eq!(var_vec, vec![1, 1, 1, 1]);
    }
}
