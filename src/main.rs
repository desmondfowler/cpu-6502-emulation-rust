// Following along with https://www.youtube.com/watch?v=qJgsuQoy9bc

type Byte = u8;
type Word = u16;
const MAX_MEM:  usize  = 1024 * 64; // 65,536 bytes, 64Kb

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
                 /*
                 Tutorial had each bit mapped out
                 c: bit,
                 z: bit,
                 i: bit,
                 d: bit,
                 b: bit,
                 v: bit,
                 n: bit,
                  */
}

impl Mem{
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

    fn reset(&mut self) {
        self.pc = 0xFFFC; // 6502 reset vector
        self.sp = 0xFF; // Stack pointer starts at 0xFF
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.flags = 0;

    }
}

fn main() {
    let mut mem = Mem::new();
    let mut cpu = CPU::new();

    cpu.reset();
    mem.initialize();
    println!("PC: 0x{:04X}, SP: 0x{:02X}", cpu.pc, cpu.sp);
}
