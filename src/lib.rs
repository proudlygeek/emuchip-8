extern crate wasm_bindgen;
mod vm;

use vm::VM;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(a: i32, b: i32) -> i32 {
  a + b
}

#[wasm_bindgen]
pub struct Hello {
  hello: String,
  last: String,
}

#[wasm_bindgen]
impl Hello {
  pub fn new(name: String, last: String) -> Hello {
    Hello {
      hello: name,
      last: last,
    }
  }

  pub fn get_name(&self) -> String {
    self.hello.clone()
  }
}

#[wasm_bindgen]
impl VM {
  fn get_memory(&mut self) -> *const u8 {
    self.memory.as_ptr()
  }
}
