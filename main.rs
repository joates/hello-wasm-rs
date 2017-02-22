// to compile to wasm..
// rustc --target wasm32-unknown-emscripten main.rs -C link-arg="-s" -C link-arg="BINARYEN_METHOD='native-wasm,interpret-binary'"

fn main() { println!("Hello, Emscripten!"); }
