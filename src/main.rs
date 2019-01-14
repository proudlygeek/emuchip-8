extern crate sdl2;

mod vm;

use crate::vm::VM;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::env;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut debug = false;

    if args.len() > 2 {
        debug = args[2] == "--debug";
    }

    let mut vm = VM::initialize(debug);
    vm.load_fontset();
    vm.load_game(&args[1]);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("EmuChip-8", 640, 320)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => handle_key_down(&mut vm, keycode),
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => handle_key_up(&mut vm, keycode),
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 600));
        // The rest of the game loop goes here...
        vm.emulate_cycle();

        if vm.draw_flag {
            draw_graphic(&vm, &mut canvas, 10);
        }
    }

    if debug {
        vm.debug_memory();
        vm.debug_registers();
    }
}

fn draw_graphic(vm: &VM, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, scale: u32) {
    for i in 0..64 * 32 {
        let pixel = vm.gfx[i];
        let x = (i % 64) * scale as usize;
        let y = (i / 64) * scale as usize;

        canvas.set_draw_color(Color::RGB(0, 0, 0));

        if pixel == 1 {
            canvas.set_draw_color(Color::RGB(255, 255, 255));
        }

        let _ = canvas.fill_rect(Rect::new(x as i32, y as i32, scale, scale));
    }

    canvas.present();
}

pub fn handle_key_down(vm: &mut vm::VM, keycode: Keycode) {
    match keycode {
        Keycode::Num1 => vm.key[0x1] = true,
        Keycode::Num2 => vm.key[0x2] = true,
        Keycode::Num3 => vm.key[0x3] = true,
        Keycode::Num4 => vm.key[0xC] = true,
        Keycode::Q => vm.key[0x4] = true,
        Keycode::W => vm.key[0x5] = true,
        Keycode::E => vm.key[0x6] = true,
        Keycode::R => vm.key[0xD] = true,
        Keycode::A => vm.key[0x7] = true,
        Keycode::S => vm.key[0x8] = true,
        Keycode::D => vm.key[0x9] = true,
        Keycode::F => vm.key[0xE] = true,
        Keycode::Z => vm.key[0xA] = true,
        Keycode::X => vm.key[0x0] = true,
        Keycode::C => vm.key[0xB] = true,
        Keycode::V => vm.key[0xF] = true,
        _ => (),
    }
}

pub fn handle_key_up(vm: &mut VM, keycode: Keycode) {
    match keycode {
        Keycode::Num1 => vm.key[0x1] = false,
        Keycode::Num2 => vm.key[0x2] = false,
        Keycode::Num3 => vm.key[0x3] = false,
        Keycode::Num4 => vm.key[0xC] = false,
        Keycode::Q => vm.key[0x4] = false,
        Keycode::W => vm.key[0x5] = false,
        Keycode::E => vm.key[0x6] = false,
        Keycode::R => vm.key[0xD] = false,
        Keycode::A => vm.key[0x7] = false,
        Keycode::S => vm.key[0x8] = false,
        Keycode::D => vm.key[0x9] = false,
        Keycode::F => vm.key[0xE] = false,
        Keycode::Z => vm.key[0xA] = false,
        Keycode::X => vm.key[0x0] = false,
        Keycode::C => vm.key[0xB] = false,
        Keycode::V => vm.key[0xF] = false,
        _ => (),
    }
}
