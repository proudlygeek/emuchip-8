struct VM {
    opcode: u8,         // 1 byte opcodes
    memory: [u8; 4096], // 4KB of memory == 4096 Bytes
    v: [u8; 16],        // 16 8-bit registries (from V0 to VE)
    i: u8,              // Index Register, 8 bit
    pc: u8,             // Program counter, 8 bit
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
            opcode: 0x00,
            memory: [0; 4096],
            v: [0; 16],
            i: 0,
            pc: 0,
            stack: [0; 16],
            sp: 0,
            key: [0; 16],
            gfx: [0; 64 * 32],
            delay_timer: 0,
            sound_timer: 0xFF,
            draw_flag: false,
        }
    }

    fn load_game(&self, game: String) {
        // Load binary into memory
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
    let vm = VM::initialize();

    setup_graphics();
    setup_input();

    vm.load_game(String::from("pong"));

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
