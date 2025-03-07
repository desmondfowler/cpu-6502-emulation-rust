// Following along with https://www.youtube.com/watch?v=qJgsuQoy9bc

type Byte = u8;
type Word = u16;

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

impl CPU {
    const C: Byte = 1 << 0; // Carry flag (bit 0)
    const Z: Byte = 1 << 1; // Zero flag (bit 1)
    const I: Byte = 1 << 2; // Interrupt disable (bit 2)
    const D: Byte = 1 << 3; // Decimal mode (bit 3)
    const B: Byte = 1 << 4; // Break command (bit 4)
    const V: Byte = 1 << 6; // Overflow flag (bit 6)
    const N: Byte = 1 << 7; // Negative flag (bit 7)
}

fn main() {
    println!("Hello, world!");
}
