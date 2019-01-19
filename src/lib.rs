extern crate wasm_bindgen;

mod vm;

use vm::VM;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Emulator {
  vm: vm::VM,
}

#[wasm_bindgen]
impl Emulator {
  pub fn new() -> Emulator {
    Emulator {
      vm: VM::initialize(false),
    }
  }

  pub fn load_fontset(&mut self) {
    self.vm.load_fontset();
  }

  pub fn get_memory(&mut self) -> *const u8 {
    self.vm.memory.as_ptr()
  }

  pub fn get_gfx(&mut self) -> *const u8 {
    self.vm.gfx.as_ptr()
  }

  pub fn get_keys(&mut self) -> *const bool {
    self.vm.key.as_ptr()
  }

  pub fn tick(&mut self) {
    self.vm.emulate_cycle();
  }

  pub fn draw_flag(&mut self) -> bool {
    self.vm.draw_flag
  }

}
