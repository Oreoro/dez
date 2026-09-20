# Cli

## Testing

You can test your changes to the `cli` crate by first building the main dez binary:

```
cargo build -p dez
```

And then building and running the `cli` crate with the following parameters:

```
 cargo run -p cli -- --dez ./target/debug/dez.exe
```
