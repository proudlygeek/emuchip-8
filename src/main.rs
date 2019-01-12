use std::fs;

struct VM {
    opcode: u16,        // 2 bytes opcodes
    memory: [u8; 4096], // 4KB of memory == 4096 bytes
    v: [u8; 16],        // 16 8-bit registers (from V0 to VE)
    i: u16,             // Index register, 2 bytes
    pc: u16,            // Program counter, 2 bytes
    stack: [u16; 16],   // Stack
    sp: u16,            // Stack Pointer
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

    fn debug_registers(&self) {
        for i in 0..16 {
            println!("V{:X}: {:X}", i, self.v[i as usize]);
        }

        println!("I: {:X}", self.i);
    }

    fn emulate_cycle(&mut self) {
        // Fetch Opcode
        // -----------
        // Left bitshift + bitwise or = merge two bytes
        self.opcode = ((self.memory[self.pc as usize] as u16) << 8)
            | (self.memory[self.pc as usize + 1]) as u16;

        println!("Opcode: {:X}", self.opcode);

        let op_1 = (self.opcode & 0xF000) >> 12;
        let op_2 = (self.opcode & 0x0F00) >> 8;
        let op_3 = (self.opcode & 0x00F0) >> 4;
        let op_4 = self.opcode & 0x000F;

        // Decode and Execute Opcode
        match (op_1, op_2, op_3, op_4) {
            (0x2, _, _, _) => self.call_addr(),
            (0x6, _, _, _) => self.ld_vx_byte(),
            (0xA, _, _, _) => self.ld_i_addr(),
            (0xD, _, _, _) => self.drw_vx_vy_n(),
            (0xF, _, 0x3, 0x3) => self.ld_b_vx(),
            _ => self.unsupported_opcode(),
        }

        // Update timers
    }

    fn set_keys(&self) {
        // Store key press state (Press and Release)
    }

    fn call_addr(&mut self) {
        let subroutine_address = self.opcode & 0x0FFF;

        println!("CALL {:X}\n", subroutine_address);

        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = subroutine_address;
    }

    fn ld_b_vx(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8;

        println!("LD B, V{}", vx);

        self.memory[self.i as usize] = self.v[vx as usize] / 100;
        self.memory[(self.i + 1) as usize] = (self.v[vx as usize] / 10) % 10;
        self.memory[(self.i + 2) as usize] = (self.v[vx as usize] % 100) % 10;

        self.pc += 2;
    }

    fn ld_vx_byte(&mut self) {
        let v = (self.opcode & 0x0F00) >> 8;
        let value = (self.opcode & 0x00FF) as u8;

        println!("LD V{}, {:X}\n", v, value);

        self.v[v as usize] = value;
        self.pc += 2;
    }

    fn ld_i_addr(&mut self) {
        let value = self.opcode & 0x0FFF;

        println!("LD I, {:X}\n", value);
        self.i = value;
        self.pc += 2;
    }

    fn drw_vx_vy_n(&mut self) {
        let vx = self.v[((self.opcode & 0x0F00) >> 8) as usize];
        let vy = self.v[((self.opcode & 0x00F0) >> 4) as usize];
        let rows = self.opcode & 0x000F;

        println!("DRW V{}, V{}, {}\n", vx, vy, rows);

        self.v[0xF] = 0; // Reset register VF

        for y in 0..rows {
            let pixel = self.memory[(self.i + y) as usize];

            for x in 0..8 {
                if (pixel & (0x80 >> x)) != 0 {
                    let current_position =
                        ((vx as u16 + x as u16) + ((vy as u16 + y) * 64)) % (32 * 64);

                    if self.gfx[current_position as usize] == 1 {
                        self.v[0xF] = 1; // Collision detected, set register VF
                    }

                    self.gfx[current_position as usize] ^= 1; // Set pixel value using XOR
                }
            }
        }

        self.draw_flag = true;
        self.pc += 2;
    }

    fn unsupported_opcode(&self) {
        // self.debug_memory();
        // self.debug_registers();
        panic!("Opcode not handled: {:X}", self.opcode);
    }
}

fn main() {
    let mut vm = VM::initialize();

    setup_graphics();
    setup_input();

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
