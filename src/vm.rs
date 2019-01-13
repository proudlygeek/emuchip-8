use std::fs;

pub struct VM {
    opcode: u16,            // 2 bytes opcodes
    memory: [u8; 4096],     // 4KB of memory == 4096 bytes
    v: [u8; 16],            // 16 8-bit registers (from V0 to VE)
    i: u16,                 // Index register, 2 bytes
    pc: u16,                // Program counter, 2 bytes
    stack: [u16; 16],       // Stack
    sp: u16,                // Stack Pointer
    key: [u8; 16],          // 1 byte for each input direction + controls
    pub gfx: [u8; 64 * 32], // Graphics is 64x32 pixels resolution, 1 byte each
    delay_timer: u8,        // Timer for events
    sound_timer: u8,        // Timer for emitting sounds. When zero, sound is emitted
    pub draw_flag: bool,    // Flush graphic
    debug: bool,            // Debug mode
}

impl VM {
    pub fn initialize(debug: bool) -> VM {
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
            debug: debug,
        }
    }

    pub fn load_game(&mut self, path: String) {
        let buffer = fs::read(path).unwrap();

        for i in 0..buffer.len() {
            self.memory[512 + i] = buffer[i];
        }
    }

    pub fn load_fontset(&mut self) {
        let fontset: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];

        for i in 0..80 {
            self.memory[i] = fontset[i];
        }
    }

    pub fn debug_memory(&self) {
        for i in (0..self.memory.len()).step_by(8) {
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

    pub fn debug_registers(&self) {
        for i in 0..16 {
            println!("V{:X}: {:X}", i, self.v[i as usize]);
        }

        println!("I: {:X}", self.i);
        println!("pc: 0x{:02X}", self.pc);
        println!("s[sp]: 0x{:02X}", self.stack[self.sp as usize]);
    }

    pub fn emulate_cycle(&mut self) {
        // Fetch Opcode
        // -----------
        // Left bitshift + bitwise or = merge two bytes
        self.opcode = ((self.memory[self.pc as usize] as u16) << 8)
            | (self.memory[self.pc as usize + 1]) as u16;

        println!("Opcode: 0x{:04X}", self.opcode);

        let op_1 = (self.opcode & 0xF000) >> 12;
        let op_2 = (self.opcode & 0x0F00) >> 8;
        let op_3 = (self.opcode & 0x00F0) >> 4;
        let op_4 = self.opcode & 0x000F;

        // Decode and Execute Opcode
        match (op_1, op_2, op_3, op_4) {
            (0x0, 0x0, 0xE, 0xE) => self.ret(),
            (0x1, _, _, _) => self.jp_addr(),
            (0x2, _, _, _) => self.call_addr(),
            (0x3, _, _, _) => self.se_vx_byte(),
            (0x6, _, _, _) => self.ld_vx_byte(),
            (0x7, _, _, _) => self.add_vx_byte(),
            (0xA, _, _, _) => self.ld_i_addr(),
            (0xD, _, _, _) => self.drw_vx_vy_n(),
            (0xF, _, 0x0, 0x7) => self.ld_vx_dt(),
            (0xF, _, 0x1, 0x5) => self.ld_dt_vx(),
            (0xF, _, 0x2, 0x9) => self.ld_f_vx(),
            (0xF, _, 0x3, 0x3) => self.ld_b_vx(),
            (0xF, _, 0x6, 0x5) => self.ld_vx_i(),
            _ => self.unsupported_opcode(),
        }

        // Update timers
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;

            if self.sound_timer == 0 {
                println!("Sound");
            }
        }
    }

    pub fn set_keys(&self) {
        // Store key press state (Press and Release)
    }

    fn ret(&mut self) {
        println!("RET\n");
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize] + 2;
    }

    fn jp_addr(&mut self) {
        let addr = self.opcode & 0x0FFF;

        println!("JP {:X}\n", addr);

        self.pc = addr;
    }

    fn call_addr(&mut self) {
        let subroutine_address = self.opcode & 0xFFF;

        println!("CALL {:X}\n", subroutine_address);

        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = subroutine_address;
    }

    fn se_vx_byte(&mut self) {
        let x = (self.opcode & 0x0F00) >> 8;
        let byte = (self.opcode & 0x00FF) as u8;

        println!("SE V{}, {:X}\n", x, byte);

        if self.v[x as usize] == byte {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    fn ld_f_vx(&mut self) {
        let x = (self.opcode & 0x0F00) >> 8;

        println!("LD F, V{}\n", x);

        self.i = (self.v[x as usize] * 0x5) as u16;
        self.pc += 2;
    }

    fn ld_b_vx(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8;

        println!("LD B, V{}\n", vx);

        self.memory[self.i as usize] = self.v[vx as usize] / 100;
        self.memory[(self.i + 1) as usize] = (self.v[vx as usize] / 10) % 10;
        self.memory[(self.i + 2) as usize] = (self.v[vx as usize] % 100) % 10;
        self.pc += 2;
    }

    fn ld_vx_i(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8;

        println!("LD V{}, [I]\n", vx);

        for v in 0..vx {
            self.v[v as usize] = self.memory[(self.i + v) as usize];
        }

        self.pc += 2;
    }

    fn ld_vx_byte(&mut self) {
        let v = (self.opcode & 0x0F00) >> 8;
        let value = (self.opcode & 0x00FF) as u8;

        println!("LD V{}, {:X}\n", v, value);

        self.v[v as usize] = value;
        self.pc += 2;
    }

    fn add_vx_byte(&mut self) {
        let x = (self.opcode & 0x0F00) >> 8;
        let byte = (self.opcode & 0x00FF) as u8;

        println!("ADD V{}, {:X}\n", x, byte);

        self.v[x as usize] = self.v[x as usize] + byte;
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

    fn ld_vx_dt(&mut self) {
        let x = (self.opcode & 0x0F00) >> 8;

        println!("LD V{}, DT\n", x);

        self.v[x as usize] = self.delay_timer;
        self.pc += 2;
    }

    fn ld_dt_vx(&mut self) {
        let x = (self.opcode & 0x0F00) >> 8;

        println!("LD DT, V{}\n", x);

        self.delay_timer = self.v[x as usize];
        self.pc += 2;
    }

    fn unsupported_opcode(&self) {
        if self.debug {
            self.debug_memory();
            self.debug_registers();
        }

        panic!("Opcode not handled: 0x{:04X}", self.opcode);
    }
}
