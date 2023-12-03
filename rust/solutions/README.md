# Advent of Code 2023 - Solutions in Rust

## Development

1. Generate a solution template for a new day

```
cargo generate --path ../template --name day-XX
```

Add the package to the root workspace in `Cargo.toml`

```
members = [
    "day-01",

    ...

    "day-XX"
]
```

2. Enter the new folder and watch tests

```
cd day-XX
cargo watch -x "test -- --nocapture"
```

3. Run the solution on the contents of input.txt

```
cargo run --bin part-X
```

## Editor

I recommend opening the `solutions` folder in VSCode and using the `rust-analyzer` extension
