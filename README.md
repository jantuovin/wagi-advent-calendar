# [WAGI advent calendar (using Rust)](https://jantuovin.fi/wagi-advent-calendar)

# Info
[WAGI: WebAssembly Gateway Interface](https://github.com/deislabs/wagi)

# Build
```
cargo build --target wasm32-wasi --release
```

# Run
```
wagi -c ./modules.toml
```
