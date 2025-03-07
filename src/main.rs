type Byte = u8;
type Word = u16;
const MAX_MEM: usize = 1024 * 64; // 65,536 bytes, 64Kb

struct Mem {
    data: Box<[Byte; MAX_MEM]>, // Using Box so it gets allocated on the heap and not the stack
}

struct CPU {
    pc: Word, //program counter
    sp: Byte, //stack pointer
    a: Byte,  //registers
    x: Byte,
    y: Byte,

    flags: Byte, // We use byte type with bitwise operations for status flags
}

impl Mem {
    fn new() -> Mem {
        Mem {
            data: Box::new([0; MAX_MEM]), // Initialize 64Kb with zeroes
        }
    }
    fn initialize(&mut self) {
        for i in 0..MAX_MEM {
            self.data[i] = 0;
        }
    }
    //read 1 byte
    fn read_byte(&self, address: Word) -> Byte {
        println!("Reading address: 0x{:04X} ({})", address, address);
        assert!((address as usize) < MAX_MEM);
        self.data[address as usize]
    }

    fn write_byte(&mut self, address: Word, value: Byte) {
        println!(
            "Writing value: 0x{:02X} at address: 0x{:04X} ({})",
            value, address, address
        );
        assert!((address as usize) < MAX_MEM);
        self.data[address as usize] = value;
    }
}

impl CPU {
    const C: Byte = 1 << 0; // Carry flag (bit 0)
    const Z: Byte = 1 << 1; // Zero flag (bit 1)
    const I: Byte = 1 << 2; // Interrupt disable (bit 2)
    const D: Byte = 1 << 3; // Decimal mode (bit 3)
    const B: Byte = 1 << 4; // Break command (bit 4)
    const V: Byte = 1 << 6; // Overflow flag (bit 6)
    const N: Byte = 1 << 7; // Negative flag (bit 7)

    fn new() -> CPU {
        CPU {
            pc: 0,
            sp: 0xFF,
            a: 0,
            x: 0,
            y: 0,
            flags: 0,
        }
    }

    fn reset(&mut self, memory: &Mem) {
        self.pc = self.read_word(memory, 0xFFFC); // reading reset vector
        self.sp = 0xFF; // Stack pointer starts at 0xFF
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.flags = 0;
    }

    fn read_word(&self, memory: &Mem, address: Word) -> Word {
        let low_byte: Word = memory.read_byte(address) as Word;
        let high_byte: Word = memory.read_byte(address.wrapping_add(1)) as Word;
        return (high_byte << 8) | low_byte;
    }

    fn print_status(&self) {
        println!("Status:");
        println!(
            "PC:\t0b{:08b} {:08b} \nSP:\t0b{:08b} \na:\t0b{:08b} \nx:\t0b{:08b} \ny:\t0b{:08b} \nFlags:\t0b{:08b}",
            self.pc >> 8,   // High byte
            self.pc & 0xFF, // Low byte
            self.sp,
            self.a,
            self.x,
            self.y,
            self.flags
        );
    }
    fn fetch_byte(&mut self, cycles: &mut u32, memory: &Mem) -> Byte {
        let data: Byte = memory.read_byte(self.pc);
        self.pc = self.pc.wrapping_add(1);
        *cycles = cycles.wrapping_sub(1);
        return data;
    }

    // LDA opcodes
    const INS_LDA_IM: Byte = 0xA9;
    const INS_LDA_ZP: Byte = 0xA5;
    const INS_LDA_ZPX: Byte = 0xB5;
    const INS_LDA_AB: Byte = 0xAD;
    const INS_LDA_ABX: Byte = 0xBD;
    const INS_LDA_ABY: Byte = 0xB9;
    const INS_LDA_IDX: Byte = 0xA9;
    const INS_LDA_IDY: Byte = 0xA9;
    
    
    

    fn execute(&mut self, mut cycles: u32, memory: &mut Mem) {
        while cycles > 0 {
            let instruction: Byte = self.fetch_byte(&mut cycles, memory);
            println!(
                "Fetched instruction: 0b{:08b} at PC: 0b{:08b} {:08b}",
                instruction,
                self.pc >> 8,
                self.pc & 0xFF
            );
        }
    }
}

fn main() {
    println!("Rust 6502 Emulator Start");
    let mut mem: Mem = Mem::new();
    let mut cpu: CPU = CPU::new();
    println!("CPU and Memory Initialized");

    mem.write_byte(0xFFFC, 0x00); // Low byte
    mem.write_byte(0xFFFD, 0x80); // High byte -> 0x8000
    mem.write_byte(0x8000, 0xA9); // LDA #$01
    mem.write_byte(0x8001, 0x01);

    println!("Resetting CPU");
    cpu.reset(&mem);
    println!("CPU Reset");
    println!("Printing CPU Status");
    cpu.print_status();

    println!("Executing 2 cycles");
    cpu.execute(2, &mut mem);
    println!("Rust 6502 Emulator End");
}
