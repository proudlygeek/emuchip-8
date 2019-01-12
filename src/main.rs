use std::fs;

struct VM {
    opcode: u16,        // 2 bytes opcodes
    memory: [u8; 4096], // 4KB of memory == 4096 bytes
    v: [u8; 16],        // 16 8-bit registers (from V0 to VE)
    i: u16,             // Index register, 2 bytes
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

    fn emulate_cycle(&mut self) {
        // Fetch Opcode
        // -----------
        // Left bitshift + bitwise or = merge two bytes
        self.opcode = ((self.memory[self.pc as usize] as u16) << 8)
            | (self.memory[self.pc as usize + 1]) as u16;

        println!("Opcode: {:X}", self.opcode);

        // Decode and Execute Opcode
        match self.opcode & 0xF000 {
            0x6000 => self.set_vx_to_value(
                ((self.opcode & 0x0F00) >> 8) as u8,
                (self.opcode & 0x00FF) as u8,
            ),
            0xA000 => self.set_i_to_address(self.opcode & 0x0FFF),
            0xD000 => self.draw_sprite_vx_vy(
                ((self.opcode & 0x0F00) >> 8) as u8,
                ((self.opcode & 0x00F0) >> 4) as u8,
                (self.opcode & 0x000F) as u8,
            ),
            v => panic!("Opcode not handled: {:X}", v),
        }

        // Update timers
    }

    fn set_keys(&self) {
        // Store key press state (Press and Release)
    }

    fn set_vx_to_value(&mut self, v: u8, value: u8) {
        println!("Assign V{} = {:X}\n", v, value);
        self.v[v as usize] = value;
        self.pc += 2;
    }

    fn set_i_to_address(&mut self, value: u16) {
        println!("MEM I = {:X}\n", value);
        self.i = value;
        self.pc += 2;
    }

    fn draw_sprite_vx_vy(&mut self, x: u8, y: u8, height: u8) {
        println!("draw(V{},V{},{})\n", x, y, height);
        self.v[0xF] = 0; // Reset register VF

        for yline in 0..height {
            let pixel = self.memory[(self.i + yline as u16) as usize];

            for xline in 0..8 {
                if (pixel & (0x80 >> xline)) != 0 {
                    if self.gfx[(x + xline + ((y + yline) * 64)) as usize] == 1 {
                        self.v[0xF] = 1; // Collision detected, set register VF
                    }

                    self.gfx[(x + xline + ((y + yline) * 64)) as usize] ^= 1; // Set pixel value using XOR
                }
            }
        }

        self.draw_flag = true;
        self.pc += 2;
    }
}

fn main() {
    let mut vm = VM::initialize();

    setup_graphics();
    setup_input();

    vm.load_game(String::from("pong.rom"));
    // vm.debug_memory();

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
