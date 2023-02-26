use itertools::Itertools;

const INPUT: &str = include_str!("../assets/input.txt");

const NUMBER_CHAR_PER_CRATE: usize = 4;
const INSTRUCTION_WORD: &str = "move";

fn main() {
    let crates = INPUT.lines().take_while(|line| !line.is_empty());

    let number_of_stacks = crates.clone().last().unwrap().split_whitespace().count();
    let mut stacks = vec![vec![]; number_of_stacks];
    crates.for_each(|line| {
        line.char_indices()
            .filter(|(_, crate_symbol)| crate_symbol.is_alphabetic())
            .for_each(|(i, crate_symbol)| stacks[i / NUMBER_CHAR_PER_CRATE].push(crate_symbol))
    });
    stacks.iter_mut().for_each(|stack| stack.reverse());

    let mut moved_crates = vec![];
    INPUT
        .lines()
        .skip_while(|line| !line.contains(INSTRUCTION_WORD))
        .map(|line| {
            line.split_whitespace()
                .filter_map(|s| s.parse::<usize>().ok())
                .collect_tuple::<(usize, usize, usize)>()
                .map(|(number, from, to)| (number, from - 1, to - 1))
                .unwrap()
        })
        .for_each(|(number, from, to)| {
            let new_len = stacks[from].len() - number;
            moved_crates.extend(stacks[from].drain(new_len..));
            // uncomment for part 1
            // moved_crates.reverse();
            stacks[to].append(&mut moved_crates);
        });

    let result = stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>();

    println!("{result}");
}