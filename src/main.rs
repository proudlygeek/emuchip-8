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

    if args.len() > 1 {
        debug = args[1] == "--debug";
    }

    let mut vm = VM::initialize(debug);
    vm.load_fontset();
    vm.load_game(String::from("pong.rom"));

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("EmuChip-8", 800, 600)
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
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        // The rest of the game loop goes here...
        vm.emulate_cycle();

        if vm.draw_flag {
            draw_graphic(&vm, &mut canvas, 12);
        }
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
