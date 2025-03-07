# CPU 6502 Emulation in Rust

This is a project to build a 6502 CPU emulator in Rust. Eventually, I'd like to fully emulate the behavior of the classic 6502 microprocessor found in systems like the NES, Atari 2600, and Apple II.

## Status 

- Currently implements the basic CPU structure with registers (A, X, Y, PC, SP) and status flags (C, Z, I, D, B, V, N) packed into a single byte. That's it so far!

## Why? 

Mainly for fun, and to get better at Rust. The 6502 is a simple CPU to emulate, and I've seen people do it in other languages (like C, as in this [video](https://www.youtube.com/watch?v=qJgsuQoy9bc)) before, so I figured Rust would be fun. 