mod vm;

use crate::vm::VM;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut debug = false;

    if args.len() > 1 {
        debug = args[1] == "--debug";
    }

    let mut vm = VM::initialize(debug);

    setup_graphics();
    setup_input();
    vm.load_fontset();
    vm.load_game(String::from("pong.rom"));

    loop {
        vm.emulate_cycle();

        if vm.draw_flag {
            draw_graphic();
        }

        vm.set_keys();
    }
}

fn setup_graphics() {}

fn setup_input() {}

fn draw_graphic() {}
