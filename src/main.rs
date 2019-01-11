use std::fs;

struct VM {
    opcode: u8,         // 1 byte opcodes
    memory: [u8; 4096], // 4KB of memory == 4096 bytes
    v: [u8; 16],        // 16 8-bit registries (from V0 to VE)
    i: u16,             // Index Register, 2 bytes
    pc: u16,            // Program counter, 2 bytes
    stack: [u8; 16],    // Stack
    sp: u8,             // Stack Pointer
    key: [u8; 16],      // 1 byte for each input direction + controls
    gfx: [u8; 64 * 32], // Graphics is 64x32 pixels resolution, 1 byte each
    delay_timer: u8,    // Timer for events
    sound_timer: u8,    // Timer for emitting sounds. When zero, sound is emitted
    draw_flag: bool,    // Flush graphic
}

impl VM {
    fn initialize() -> VM {
        VM {
            pc: 0x200,
            opcode: 0,
            memory: [0; 4096],
            v: [0; 16],
            i: 0,
            stack: [0; 16],
            sp: 0,
            key: [0; 16],
            gfx: [0; 64 * 32],
            delay_timer: 0,
            sound_timer: 0xFF,
            draw_flag: false,
        }
    }

    fn load_game(&mut self, path: String) {
        let buffer = fs::read(path).unwrap();

        for i in 0..buffer.len() {
            self.memory[512 + i] = buffer[i];
        }
    }

    fn debug_memory(&self) {
        for i in (0..self.memory.len()).step_by(16) {
            println!(
                "0x{:02X} | {:02X}{:02X} {:02X}{:02X} {:02X}{:02X} {:02X}{:02X}",
                i,
                self.memory[i],
                self.memory[i + 1],
                self.memory[i + 2],
                self.memory[i + 3],
                self.memory[i + 4],
                self.memory[i + 5],
                self.memory[i + 6],
                self.memory[i + 7],
            );
        }
    }

    fn emulate_cycle(&self) {
        // Emulate machine cycle
        // Fetch Opcode
        // Decode Opcode
        // Execute Opcode

        // Update timers
    }

    fn set_keys(&self) {
        // Store key press state (Press and Release)
    }
}

fn main() {
    let mut vm = VM::initialize();

    setup_graphics();
    setup_input();

    vm.load_game(String::from("pong.rom"));
    vm.debug_memory();

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
