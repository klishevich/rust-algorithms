# rust-algorithms

## How to create a new rust project

```shell
cargo new --vcs=none advent-day8
```

## Build and run

1. `cd advent-day7`
2. `cargo run ` or (`cargo build` and `./target/debug/advent-day7`)

## Run release mode

`cargo run --release`

## Run tests

`cargo test`

## Clippy

`cargo clippy`

## Debug errors

`RUST_BACKTRACE=1 cargo run`
`RUST_BACKTRACE=full cargo run`

## Hints

### HashMap::from

```rust
HashMap::from([(char1, 1), (char2, 1)])
```

### try_from and try_into

```rust
// TRY_FROM
let val: usize = usize::try_from(ch.to_digit(10).unwrap()).unwrap();
// TRY_INTO
let start_val: i32 = ch.to_digit(10).unwrap().try_into().unwrap();
```

### Binary number formatting

```rust
let a = i64::from_str_radix(&c.to_string(), 16).unwrap();
// BINARY NUMBER FORMATTING
let b = format!("{a:b}");
let b = format!("{:0>4}", a);
let b = format!("{:04b}", a);
```

## install crate

add to Cargo.toml file 
```
[dependencies]
hex = "0.4.3"
```