# correcthorse

A toy chess engine

## Run

```sh
# Run the engine in UCI mode
cargo run

# run a perft test at depth 4 and print the node count
cargo run -r - --perft 4
```

## Tests

### Correctness

```sh
cargo build && node test/perft.ts
```
