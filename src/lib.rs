use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn add(i: i32)->i32 {
    log("Calling JS console.log from Rust");
    i+3
}

#[wasm_bindgen]
pub fn hello_world() {
    alert("Hello, World!")
}
