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