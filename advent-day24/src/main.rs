use std::fs;
use std::collections::HashMap;

type TCommandTypeArg1Arg2 = (u8, i32, i32);

fn parse_var(s: &str) -> i32 {
    match s {
        "w" => -1,
        "x" => -2,
        "y" => -3,
        "z" => -4,
        other => other.parse().unwrap()
    }
}

fn read_alu_commands_from_file(type_map: &HashMap<&str, u8>) -> Vec<TCommandTypeArg1Arg2> {
    let content = fs::read_to_string("src/data01.txt").expect("some bug");
    let mut res: Vec<TCommandTypeArg1Arg2> = Vec::new();
    for line in content.lines() {
        println!("line {}", line);
        let mut elements = line.split(" ").peekable();
        let t_str = elements.next().unwrap();
        let t: u8 = match type_map.get(t_str) {
            Some(v) => *v,
            None => 0
        };
        let a: i32 = parse_var(elements.next().unwrap()); 
        let mut b: i32 = 0;
        if !elements.peek().is_none() {
            b = parse_var(elements.next().unwrap());
        }
        res.push((t, a, b));
    }
    return res;
}

fn main() {
    let type_map: HashMap<&str, u8> = HashMap::from([("inp", 1), ("add", 2), ("mul", 3), ("div", 4), ("mod", 5), ("eql", 6)]);
    let commands = read_alu_commands_from_file(&type_map);
    for (t, a, b) in commands {
        println!("{} {} {}", t, a, b);
    }
    let input = "1";
}
