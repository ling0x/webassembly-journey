`cargo build --target wasm32-unknown-unknown --release`

`wasm2wat ./target/wasm32-unknown-unknown/release/calc.wasm`

`wasmtime --invoke add ./target/wasm32-unknown-unknown/release/calc.wasm 4 2`
