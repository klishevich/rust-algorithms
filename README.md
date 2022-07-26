# rust-algorithms

## How to create a new rust project

```shell
cargo new --vcs=none advent-day8
```

## Build and run

1. `cd advent-day7`
2. `cargo run ` or (`cargo build` and `./target/debug/advent-day7`)

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

## install crate

add to Cargo.toml file 