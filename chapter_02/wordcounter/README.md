`rustup target add wasm32-wasip1`

`cargo build --target wasm32-wasip1`

`wasmtime --dir ~/Documents target/wasm32-wasip1/debug/wordcounter.wasm test.txt`
