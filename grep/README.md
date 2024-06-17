# Simple grep

## How to run

```console
cargo run -- read ./src/main.rs
```

```console
./src/main.rs
    7: fn grep<R>(target: &str, reader: R) -> io::Result<()>
    9:     for (n, line_result) in reader.lines().enumerate() {
```