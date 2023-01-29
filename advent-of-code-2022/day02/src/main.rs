fn main() {
    let input = getTestInput();
    println!("{}", input);
    let temp = input.split("\n");
    println!("{:?}", temp);
}

fn getTestInput<'life>() -> &'life str {
    &r"A Y
B X
C Z"
}