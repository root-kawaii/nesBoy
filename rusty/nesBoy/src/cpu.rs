use crate::bus::Bus;
use lazy_static::lazy_static;
// use std::collections::HashMap;

// #[derive(Debug)]
// #[allow(non_camel_case_types)]
// pub enum AddressingMode {
//     Immediate,
//     Accumulator,
//     ZeroPage,
//     ZeroPage_X,
//     ZeroPage_Y,
//     Absolute,
//     Absolute_X,
//     Absolute_X_PageCross,
//     Absolute_Y,
//     Absolute_Y_PageCross,
//     Indirect_X,
//     Indirect_Y,
//     Indirect_Y_PageCross,
//     NoneAddressing,
// }

// pub struct OpsCode {
//     pub code: u8,
//     pub mnemonic: &'static str,
//     pub len: u8,
//     pub cycles: u8,
//     pub mode: AddressingMode,
// }
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum FLAGS6502 {
    C = 1 << 0, // Carry
    Z = 1 << 1, // Zero
    I = 1 << 2, // Interrupt Disable
    D = 1 << 3, // Decimal Mode (not used on NES)
    B = 1 << 4, // Break
    U = 1 << 5, // Unused (always 1)
    V = 1 << 6, // Overflow
    N = 1 << 7, // Negative
}

lazy_static! {
    pub static ref NON_READABLE_ADDR: Vec<u16> = vec!(
        0x2001, 0x2002, 0x2003, 0x2004, 0x2005, 0x2006, 0x2007, 0x4016, 0x4017
    );
}

pub struct Cpu {
    // CPU registers
    pub a: u8,   // Accumulator
    pub x: u8,   // X register
    pub y: u8,   // Y register
    pub sp: u8,  // Stack pointer
    pub pc: u16, // Program counter
    pub p: u8,   // Status register

    // 64KB memory (internal RAM)
    pub memory: [u8; 0x10000],
    pub bus: Bus,
}

impl Cpu {
    pub fn new(bus: Bus) -> Self {
        Cpu {
            a: 0,
            x: 0,
            y: 0,
            sp: 0xFD,
            pc: 0xC000,
            p: 0x24,
            memory: [0; 0x10000],
            bus,
        }
    }

    pub fn reset(&mut self) {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.sp = 0xFD;
        self.pc = 0xC000;
        self.p = 0x24;
    }

    pub fn step(&mut self) {
        // Fetch opcode and execute
        println!("CPU Step: PC = {}", self.pc);
        let opcode = self.bus.read(self.pc);
        println!("Executing opcode: {}, {}", opcode, self.pc);
        self.pc = self.pc.wrapping_add(1);
        self.execute(opcode);
    }

    pub fn execute(&mut self, opcode: u8) {
        print!("op cod{} \n", opcode);
        // Execute instruction based on opcode
        match opcode {
            // BRK - Break/Software Interrupt
            0x00 => {
                self.pc = self.pc.wrapping_add(1);
                // Push PC to stack
                self.bus.write(0x100 + self.sp as u16, (self.pc >> 8) as u8);
                self.sp = self.sp.wrapping_sub(1);
                self.bus
                    .write(0x100 + self.sp as u16, (self.pc & 0xFF) as u8);
                self.sp = self.sp.wrapping_sub(1);
                // Push status register with B flag set
                self.bus.write(0x100 + self.sp as u16, self.p | 0x30);
                self.sp = self.sp.wrapping_sub(1);
                // Set interrupt disable flag
                self.set_flag(FLAGS6502::I, true);
                // Load PC from IRQ/BRK vector at 0xFFFE/0xFFFF
                let lo = self.bus.read(0xFFFE) as u16;
                let hi = self.bus.read(0xFFFF) as u16;
                self.pc = lo | (hi << 8);
            }

            // LDA - Load Accumulator
            0xA9 => {
                // LDA Immediate
                let value = self.bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                self.a = value;
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
            }
            0xA5 => {
                // LDA Zero Page
                let addr = self.bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                self.a = self.bus.read(addr as u16);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
            }
            0xB5 => {
                // LDA Zero Page,X
                let addr = self.bus.read(self.pc).wrapping_add(self.x);
                self.pc = self.pc.wrapping_add(1);
                self.a = self.bus.read(addr as u16);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
            }
            0xAD => {
                // LDA Absolute
                let lo = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = lo | (hi << 8);
                self.a = self.bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
            }
            0xBD => {
                // LDA Absolute,X
                let lo = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = (lo | (hi << 8)).wrapping_add(self.x as u16);
                self.a = self.bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
            }
            0xB9 => {
                // LDA Absolute,Y
                let lo = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = (lo | (hi << 8)).wrapping_add(self.y as u16);
                self.a = self.bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
            }
            0xA1 => {
                // LDA (Indirect,X)
                let zp = self.bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                let lo = self.bus.read(zp.wrapping_add(self.x) as u16) as u16;
                let hi = self
                    .bus
                    .read(zp.wrapping_add(self.x).wrapping_add(1) as u16)
                    as u16;
                let addr = lo | (hi << 8);
                self.a = self.bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
            }
            0xB1 => {
                // LDA (Indirect),Y
                let zp = self.bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                let lo = self.bus.read(zp as u16) as u16;
                let hi = self.bus.read(zp.wrapping_add(1) as u16) as u16;
                let addr = (lo | (hi << 8)).wrapping_add(self.y as u16);
                self.a = self.bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
            }

            // LDX - Load X Register
            0xA2 => {
                // LDX Immediate
                self.x = self.bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                self.set_flag(FLAGS6502::Z, self.x == 0x00);
                self.set_flag(FLAGS6502::N, (self.x & 0x80) != 0);
            }
            0xA6 => {
                // LDX Zero Page
                let addr = self.bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                self.x = self.bus.read(addr as u16);
                self.set_flag(FLAGS6502::Z, self.x == 0x00);
                self.set_flag(FLAGS6502::N, (self.x & 0x80) != 0);
            }
            0xB6 => {
                // LDX Zero Page,Y
                let addr = self.bus.read(self.pc).wrapping_add(self.y);
                self.pc = self.pc.wrapping_add(1);
                self.x = self.bus.read(addr as u16);
                self.set_flag(FLAGS6502::Z, self.x == 0x00);
                self.set_flag(FLAGS6502::N, (self.x & 0x80) != 0);
            }
            0xAE => {
                // LDX Absolute
                let lo = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = lo | (hi << 8);
                self.x = self.bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.x == 0x00);
                self.set_flag(FLAGS6502::N, (self.x & 0x80) != 0);
            }
            0xBE => {
                // LDX Absolute,Y
                let lo = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = (lo | (hi << 8)).wrapping_add(self.y as u16);
                self.x = self.bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.x == 0x00);
                self.set_flag(FLAGS6502::N, (self.x & 0x80) != 0);
            }

            // LDY - Load Y Register
            0xA0 => {
                // LDY Immediate
                self.y = self.bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                self.set_flag(FLAGS6502::Z, self.y == 0x00);
                self.set_flag(FLAGS6502::N, (self.y & 0x80) != 0);
            }
            0xA4 => {
                // LDY Zero Page
                let addr = self.bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                self.y = self.bus.read(addr as u16);
                self.set_flag(FLAGS6502::Z, self.y == 0x00);
                self.set_flag(FLAGS6502::N, (self.y & 0x80) != 0);
            }
            0xB4 => {
                // LDY Zero Page,X
                let addr = self.bus.read(self.pc).wrapping_add(self.x);
                self.pc = self.pc.wrapping_add(1);
                self.y = self.bus.read(addr as u16);
                self.set_flag(FLAGS6502::Z, self.y == 0x00);
                self.set_flag(FLAGS6502::N, (self.y & 0x80) != 0);
            }
            0xAC => {
                // LDY Absolute
                let lo = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = lo | (hi << 8);
                self.y = self.bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.y == 0x00);
                self.set_flag(FLAGS6502::N, (self.y & 0x80) != 0);
            }
            0xBC => {
                // LDY Absolute,X
                let lo = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = (lo | (hi << 8)).wrapping_add(self.x as u16);
                self.y = self.bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.y == 0x00);
                self.set_flag(FLAGS6502::N, (self.y & 0x80) != 0);
            }

            // STA - Store Accumulator
            0x85 => {
                // STA Zero Page
                let addr = self.bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                self.bus.write(addr as u16, self.a);
            }
            0x95 => {
                // STA Zero Page,X
                let addr = self.bus.read(self.pc).wrapping_add(self.x);
                self.pc = self.pc.wrapping_add(1);
                self.bus.write(addr as u16, self.a);
            }
            0x8D => {
                // STA Absolute
                let lo = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = lo | (hi << 8);
                self.bus.write(addr, self.a);
            }
            0x9D => {
                // STA Absolute,X
                let lo = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = (lo | (hi << 8)).wrapping_add(self.x as u16);
                self.bus.write(addr, self.a);
            }
            0x99 => {
                // STA Absolute,Y
                let lo = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = (lo | (hi << 8)).wrapping_add(self.y as u16);
                self.bus.write(addr, self.a);
            }
            0x81 => {
                // STA (Indirect,X)
                let zp = self.bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                let lo = self.bus.read(zp.wrapping_add(self.x) as u16) as u16;
                let hi = self
                    .bus
                    .read(zp.wrapping_add(self.x).wrapping_add(1) as u16)
                    as u16;
                let addr = lo | (hi << 8);
                self.bus.write(addr, self.a);
            }
            0x91 => {
                // STA (Indirect),Y
                let zp = self.bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                let lo = self.bus.read(zp as u16) as u16;
                let hi = self.bus.read(zp.wrapping_add(1) as u16) as u16;
                let addr = (lo | (hi << 8)).wrapping_add(self.y as u16);
                self.bus.write(addr, self.a);
            }

            // STX - Store X Register
            0x86 => {
                // STX Zero Page
                let addr = self.bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                self.bus.write(addr as u16, self.x);
            }
            0x96 => {
                // STX Zero Page,Y
                let addr = self.bus.read(self.pc).wrapping_add(self.y);
                self.pc = self.pc.wrapping_add(1);
                self.bus.write(addr as u16, self.x);
            }
            0x8E => {
                // STX Absolute
                let lo = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = lo | (hi << 8);
                self.bus.write(addr, self.x);
            }

            // STY - Store Y Register
            0x84 => {
                // STY Zero Page
                let addr = self.bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                self.bus.write(addr as u16, self.y);
            }
            0x94 => {
                // STY Zero Page,X
                let addr = self.bus.read(self.pc).wrapping_add(self.x);
                self.pc = self.pc.wrapping_add(1);
                self.bus.write(addr as u16, self.y);
            }
            0x8C => {
                // STY Absolute
                let lo = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = lo | (hi << 8);
                self.bus.write(addr, self.y);
            }

            // ADC - Add with Carry
            0x69 => {
                // ADC Immediate
                let value = self.bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                let carry = if self.get_flag(FLAGS6502::C) { 1 } else { 0 };
                let result = self.a as u16 + value as u16 + carry;
                self.set_flag(FLAGS6502::C, result > 0xFF);
                self.set_flag(FLAGS6502::Z, (result & 0xFF) == 0x00);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
                self.set_flag(
                    FLAGS6502::V,
                    ((self.a ^ value) & 0x80) == 0 && ((self.a ^ (result as u8)) & 0x80) != 0,
                );
                self.a = (result & 0xFF) as u8;
            }
            0x65 => {
                // ADC Zero Page
                let addr = self.bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                let value = self.bus.read(addr as u16);
                let carry = if self.get_flag(FLAGS6502::C) { 1 } else { 0 };
                let result = self.a as u16 + value as u16 + carry;
                self.set_flag(FLAGS6502::C, result > 0xFF);
                self.set_flag(FLAGS6502::Z, (result & 0xFF) == 0x00);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
                self.set_flag(
                    FLAGS6502::V,
                    ((self.a ^ value) & 0x80) == 0 && ((self.a ^ (result as u8)) & 0x80) != 0,
                );
                self.a = (result & 0xFF) as u8;
            }
            0x6D => {
                // ADC Absolute
                let lo = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = lo | (hi << 8);
                let value = self.bus.read(addr);
                let carry = if self.get_flag(FLAGS6502::C) { 1 } else { 0 };
                let result = self.a as u16 + value as u16 + carry;
                self.set_flag(FLAGS6502::C, result > 0xFF);
                self.set_flag(FLAGS6502::Z, (result & 0xFF) == 0x00);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
                self.set_flag(
                    FLAGS6502::V,
                    ((self.a ^ value) & 0x80) == 0 && ((self.a ^ (result as u8)) & 0x80) != 0,
                );
                self.a = (result & 0xFF) as u8;
            }

            // AND - Logical AND
            0x29 => {
                // AND Immediate
                let value = self.bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                self.a &= value;
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
            }
            0x25 => {
                // AND Zero Page
                let addr = self.bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                self.a &= self.bus.read(addr as u16);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
            }
            0x35 => {
                // AND Zero Page,X
                let addr = self.bus.read(self.pc).wrapping_add(self.x);
                self.pc = self.pc.wrapping_add(1);
                self.a &= self.bus.read(addr as u16);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
            }
            0x2D => {
                // AND Absolute
                let lo = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = lo | (hi << 8);
                self.a &= self.bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
            }
            0x3D => {
                // AND Absolute,X
                let lo = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = (lo | (hi << 8)).wrapping_add(self.x as u16);
                self.a &= self.bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
            }
            0x39 => {
                // AND Absolute,Y
                let lo = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = (lo | (hi << 8)).wrapping_add(self.y as u16);
                self.a &= self.bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
            }
            0x21 => {
                // AND (Indirect,X)
                let zp = self.bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                let lo = self.bus.read(zp.wrapping_add(self.x) as u16) as u16;
                let hi = self
                    .bus
                    .read(zp.wrapping_add(self.x).wrapping_add(1) as u16)
                    as u16;
                let addr = lo | (hi << 8);
                self.a &= self.bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
            }
            0x31 => {
                // AND (Indirect),Y
                let zp = self.bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                let lo = self.bus.read(zp as u16) as u16;
                let hi = self.bus.read(zp.wrapping_add(1) as u16) as u16;
                let addr = (lo | (hi << 8)).wrapping_add(self.y as u16);
                self.a &= self.bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
            }

            // ORA - Logical OR
            0x09 => {
                // ORA Immediate
                let value = self.bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                self.a |= value;
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
            }
            0x05 => {
                // ORA Zero Page
                let addr = self.bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                self.a |= self.bus.read(addr as u16);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
            }
            0x15 => {
                // ORA Zero Page,X
                let addr = self.bus.read(self.pc).wrapping_add(self.x);
                self.pc = self.pc.wrapping_add(1);
                self.a |= self.bus.read(addr as u16);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
            }
            0x0D => {
                // ORA Absolute
                let lo = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = lo | (hi << 8);
                self.a |= self.bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
            }

            // EOR - Logical XOR
            0x49 => {
                // EOR Immediate
                let value = self.bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                self.a ^= value;
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
            }
            0x45 => {
                // EOR Zero Page
                let addr = self.bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                self.a ^= self.bus.read(addr as u16);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
            }
            0x4D => {
                // EOR Absolute
                let lo = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = lo | (hi << 8);
                self.a ^= self.bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
            }

            // CMP - Compare Accumulator
            0xC9 => {
                // CMP Immediate
                let value = self.bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                let result = self.a.wrapping_sub(value);
                self.set_flag(FLAGS6502::C, self.a >= value);
                self.set_flag(FLAGS6502::Z, self.a == value);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
            }
            0xC5 => {
                // CMP Zero Page
                let addr = self.bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                let value = self.bus.read(addr as u16);
                let result = self.a.wrapping_sub(value);
                self.set_flag(FLAGS6502::C, self.a >= value);
                self.set_flag(FLAGS6502::Z, self.a == value);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
            }
            0xD5 => {
                // CMP Zero Page,X
                let addr = self.bus.read(self.pc).wrapping_add(self.x);
                self.pc = self.pc.wrapping_add(1);
                let value = self.bus.read(addr as u16);
                let result = self.a.wrapping_sub(value);
                self.set_flag(FLAGS6502::C, self.a >= value);
                self.set_flag(FLAGS6502::Z, self.a == value);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
            }
            0xCD => {
                // CMP Absolute
                let lo = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = lo | (hi << 8);
                let value = self.bus.read(addr);
                let result = self.a.wrapping_sub(value);
                self.set_flag(FLAGS6502::C, self.a >= value);
                self.set_flag(FLAGS6502::Z, self.a == value);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
            }
            0xDD => {
                // CMP Absolute,X
                let lo = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = (lo | (hi << 8)).wrapping_add(self.x as u16);
                let value = self.bus.read(addr);
                let result = self.a.wrapping_sub(value);
                self.set_flag(FLAGS6502::C, self.a >= value);
                self.set_flag(FLAGS6502::Z, self.a == value);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
            }
            0xD9 => {
                // CMP Absolute,Y
                let lo = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = (lo | (hi << 8)).wrapping_add(self.y as u16);
                let value = self.bus.read(addr);
                let result = self.a.wrapping_sub(value);
                self.set_flag(FLAGS6502::C, self.a >= value);
                self.set_flag(FLAGS6502::Z, self.a == value);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
            }
            0xC1 => {
                // CMP (Indirect,X)
                let zp = self.bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                let lo = self.bus.read(zp.wrapping_add(self.x) as u16) as u16;
                let hi = self
                    .bus
                    .read(zp.wrapping_add(self.x).wrapping_add(1) as u16)
                    as u16;
                let addr = lo | (hi << 8);
                let value = self.bus.read(addr);
                let result = self.a.wrapping_sub(value);
                self.set_flag(FLAGS6502::C, self.a >= value);
                self.set_flag(FLAGS6502::Z, self.a == value);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
            }
            0xD1 => {
                // CMP (Indirect),Y
                let zp = self.bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                let lo = self.bus.read(zp as u16) as u16;
                let hi = self.bus.read(zp.wrapping_add(1) as u16) as u16;
                let addr = (lo | (hi << 8)).wrapping_add(self.y as u16);
                let value = self.bus.read(addr);
                let result = self.a.wrapping_sub(value);
                self.set_flag(FLAGS6502::C, self.a >= value);
                self.set_flag(FLAGS6502::Z, self.a == value);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
            }

            // CPX - Compare X Register
            0xE0 => {
                // CPX Immediate
                let value = self.bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                let result = self.x.wrapping_sub(value);
                self.set_flag(FLAGS6502::C, self.x >= value);
                self.set_flag(FLAGS6502::Z, self.x == value);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
            }
            0xE4 => {
                // CPX Zero Page
                let addr = self.bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                let value = self.bus.read(addr as u16);
                let result = self.x.wrapping_sub(value);
                self.set_flag(FLAGS6502::C, self.x >= value);
                self.set_flag(FLAGS6502::Z, self.x == value);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
            }
            0xEC => {
                // CPX Absolute
                let lo = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = lo | (hi << 8);
                let value = self.bus.read(addr);
                let result = self.x.wrapping_sub(value);
                self.set_flag(FLAGS6502::C, self.x >= value);
                self.set_flag(FLAGS6502::Z, self.x == value);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
            }

            // BIT - Bit Test
            0x24 => {
                // BIT Zero Page
                let addr = self.bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                let value = self.bus.read(addr as u16);
                let result = self.a & value;
                self.set_flag(FLAGS6502::Z, result == 0x00);
                self.set_flag(FLAGS6502::V, (value & 0x40) != 0);
                self.set_flag(FLAGS6502::N, (value & 0x80) != 0);
            }
            0x2C => {
                // BIT Absolute
                let lo = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = lo | (hi << 8);
                let value = self.bus.read(addr);
                let result = self.a & value;
                self.set_flag(FLAGS6502::Z, result == 0x00);
                self.set_flag(FLAGS6502::V, (value & 0x40) != 0);
                self.set_flag(FLAGS6502::N, (value & 0x80) != 0);
            }

            // CPY - Compare Y Register
            0xC0 => {
                // CPY Immediate
                let value = self.bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                let result = self.y.wrapping_sub(value);
                self.set_flag(FLAGS6502::C, self.y >= value);
                self.set_flag(FLAGS6502::Z, self.y == value);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
            }
            0xC4 => {
                // CPY Zero Page
                let addr = self.bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                let value = self.bus.read(addr as u16);
                let result = self.y.wrapping_sub(value);
                self.set_flag(FLAGS6502::C, self.y >= value);
                self.set_flag(FLAGS6502::Z, self.y == value);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
            }
            0xCC => {
                // CPY Absolute
                let lo = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = lo | (hi << 8);
                let value = self.bus.read(addr);
                let result = self.y.wrapping_sub(value);
                self.set_flag(FLAGS6502::C, self.y >= value);
                self.set_flag(FLAGS6502::Z, self.y == value);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
            }

            // JMP - Jump
            0x4C => {
                // JMP Absolute
                let lo = self.bus.read(self.pc) as u16;
                let hi = self.bus.read(self.pc.wrapping_add(1)) as u16;
                self.pc = lo | (hi << 8);
            }
            0x6C => {
                // JMP Indirect (with 6502 bug)
                let lo = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = lo | (hi << 8);
                // 6502 bug: if addr is at page boundary (e.g., 0x12FF),
                // it wraps around to 0x1200 instead of 0x1300
                if (addr & 0xFF) == 0xFF {
                    let lo = self.bus.read(addr) as u16;
                    let hi = self.bus.read(addr & 0xFF00) as u16;
                    self.pc = lo | (hi << 8);
                } else {
                    let lo = self.bus.read(addr) as u16;
                    let hi = self.bus.read(addr.wrapping_add(1)) as u16;
                    self.pc = lo | (hi << 8);
                }
            }

            // JSR - Jump to Subroutine
            0x20 => {
                let lo = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = lo | (hi << 8);
                let ret_addr = self.pc.wrapping_sub(1);
                self.bus
                    .write(0x100 + self.sp as u16, (ret_addr >> 8) as u8);
                self.sp = self.sp.wrapping_sub(1);
                self.bus
                    .write(0x100 + self.sp as u16, (ret_addr & 0xFF) as u8);
                self.sp = self.sp.wrapping_sub(1);
                self.pc = addr;
            }

            // RTS - Return from Subroutine
            0x60 => {
                self.sp = self.sp.wrapping_add(1);
                let lo = self.bus.read(0x100 + self.sp as u16) as u16;
                self.sp = self.sp.wrapping_add(1);
                let hi = self.bus.read(0x100 + self.sp as u16) as u16;
                self.pc = (lo | (hi << 8)).wrapping_add(1);
            }

            // RTI - Return from Interrupt
            0x40 => {
                self.sp = self.sp.wrapping_add(1);
                self.p = self.bus.read(0x100 + self.sp as u16) & !0x10;
                self.sp = self.sp.wrapping_add(1);
                let lo = self.bus.read(0x100 + self.sp as u16) as u16;
                self.sp = self.sp.wrapping_add(1);
                let hi = self.bus.read(0x100 + self.sp as u16) as u16;
                self.pc = lo | (hi << 8);
            }

            // Branch Instructions
            0xF0 => {
                // BEQ - Branch if Equal
                let offset = self.bus.read(self.pc) as i8;
                self.pc = self.pc.wrapping_add(1);
                if self.get_flag(FLAGS6502::Z) {
                    self.pc = self.pc.wrapping_add(offset as u16);
                }
            }
            0xD0 => {
                // BNE - Branch if Not Equal
                let offset = self.bus.read(self.pc) as i8;
                self.pc = self.pc.wrapping_add(1);
                if !self.get_flag(FLAGS6502::Z) {
                    self.pc = self.pc.wrapping_add(offset as u16);
                }
            }
            0xB0 => {
                // BCS - Branch if Carry Set
                let offset = self.bus.read(self.pc) as i8;
                self.pc = self.pc.wrapping_add(1);
                if self.get_flag(FLAGS6502::C) {
                    self.pc = self.pc.wrapping_add(offset as u16);
                }
            }
            0x90 => {
                // BCC - Branch if Carry Clear
                let offset = self.bus.read(self.pc) as i8;
                self.pc = self.pc.wrapping_add(1);
                if !self.get_flag(FLAGS6502::C) {
                    self.pc = self.pc.wrapping_add(offset as u16);
                }
            }
            0x30 => {
                // BMI - Branch if Minus
                let offset = self.bus.read(self.pc) as i8;
                self.pc = self.pc.wrapping_add(1);
                if self.get_flag(FLAGS6502::N) {
                    self.pc = self.pc.wrapping_add(offset as u16);
                }
            }
            0x10 => {
                // BPL - Branch if Positive
                let offset = self.bus.read(self.pc) as i8;
                self.pc = self.pc.wrapping_add(1);
                if !self.get_flag(FLAGS6502::N) {
                    self.pc = self.pc.wrapping_add(offset as u16);
                }
            }
            0x70 => {
                // BVS - Branch if Overflow Set
                let offset = self.bus.read(self.pc) as i8;
                self.pc = self.pc.wrapping_add(1);
                if self.get_flag(FLAGS6502::V) {
                    self.pc = self.pc.wrapping_add(offset as u16);
                }
            }
            0x50 => {
                // BVC - Branch if Overflow Clear
                let offset = self.bus.read(self.pc) as i8;
                self.pc = self.pc.wrapping_add(1);
                if !self.get_flag(FLAGS6502::V) {
                    self.pc = self.pc.wrapping_add(offset as u16);
                }
            }

            // INC - Increment Memory
            0xE6 => {
                // INC Zero Page
                let addr = self.bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                let value = self.bus.read(addr as u16).wrapping_add(1);
                self.bus.write(addr as u16, value);
                self.set_flag(FLAGS6502::Z, value == 0x00);
                self.set_flag(FLAGS6502::N, (value & 0x80) != 0);
            }
            0xF6 => {
                // INC Zero Page,X
                let addr = self.bus.read(self.pc).wrapping_add(self.x);
                self.pc = self.pc.wrapping_add(1);
                let value = self.bus.read(addr as u16).wrapping_add(1);
                self.bus.write(addr as u16, value);
                self.set_flag(FLAGS6502::Z, value == 0x00);
                self.set_flag(FLAGS6502::N, (value & 0x80) != 0);
            }
            0xEE => {
                // INC Absolute
                let lo = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = lo | (hi << 8);
                let value = self.bus.read(addr).wrapping_add(1);
                self.bus.write(addr, value);
                self.set_flag(FLAGS6502::Z, value == 0x00);
                self.set_flag(FLAGS6502::N, (value & 0x80) != 0);
            }
            0xFE => {
                // INC Absolute,X
                let lo = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = (lo | (hi << 8)).wrapping_add(self.x as u16);
                let value = self.bus.read(addr).wrapping_add(1);
                self.bus.write(addr, value);
                self.set_flag(FLAGS6502::Z, value == 0x00);
                self.set_flag(FLAGS6502::N, (value & 0x80) != 0);
            }

            // INX/INY - Increment X/Y
            0xE8 => {
                // INX
                self.x = self.x.wrapping_add(1);
                self.set_flag(FLAGS6502::Z, self.x == 0x00);
                self.set_flag(FLAGS6502::N, (self.x & 0x80) != 0);
            }
            0xC8 => {
                // INY
                self.y = self.y.wrapping_add(1);
                self.set_flag(FLAGS6502::Z, self.y == 0x00);
                self.set_flag(FLAGS6502::N, (self.y & 0x80) != 0);
            }

            // DEC - Decrement Memory
            0xC6 => {
                // DEC Zero Page
                let addr = self.bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                let value = self.bus.read(addr as u16).wrapping_sub(1);
                self.bus.write(addr as u16, value);
                self.set_flag(FLAGS6502::Z, value == 0x00);
                self.set_flag(FLAGS6502::N, (value & 0x80) != 0);
            }
            0xD6 => {
                // DEC Zero Page,X
                let addr = self.bus.read(self.pc).wrapping_add(self.x);
                self.pc = self.pc.wrapping_add(1);
                let value = self.bus.read(addr as u16).wrapping_sub(1);
                self.bus.write(addr as u16, value);
                self.set_flag(FLAGS6502::Z, value == 0x00);
                self.set_flag(FLAGS6502::N, (value & 0x80) != 0);
            }
            0xCE => {
                // DEC Absolute
                let lo = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = lo | (hi << 8);
                let value = self.bus.read(addr).wrapping_sub(1);
                self.bus.write(addr, value);
                self.set_flag(FLAGS6502::Z, value == 0x00);
                self.set_flag(FLAGS6502::N, (value & 0x80) != 0);
            }
            0xDE => {
                // DEC Absolute,X
                let lo = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = (lo | (hi << 8)).wrapping_add(self.x as u16);
                let value = self.bus.read(addr).wrapping_sub(1);
                self.bus.write(addr, value);
                self.set_flag(FLAGS6502::Z, value == 0x00);
                self.set_flag(FLAGS6502::N, (value & 0x80) != 0);
            }

            // DEX/DEY - Decrement X/Y
            0xCA => {
                // DEX
                self.x = self.x.wrapping_sub(1);
                self.set_flag(FLAGS6502::Z, self.x == 0x00);
                self.set_flag(FLAGS6502::N, (self.x & 0x80) != 0);
            }
            0x88 => {
                // DEY
                self.y = self.y.wrapping_sub(1);
                self.set_flag(FLAGS6502::Z, self.y == 0x00);
                self.set_flag(FLAGS6502::N, (self.y & 0x80) != 0);
            }

            // Stack Operations
            0x48 => {
                // PHA - Push Accumulator
                self.bus.write(0x100 + self.sp as u16, self.a);
                self.sp = self.sp.wrapping_sub(1);
            }
            0x68 => {
                // PLA - Pull Accumulator
                self.sp = self.sp.wrapping_add(1);
                self.a = self.bus.read(0x100 + self.sp as u16);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
            }
            0x08 => {
                // PHP - Push Processor Status
                self.bus.write(0x100 + self.sp as u16, self.p | 0x10);
                self.sp = self.sp.wrapping_sub(1);
            }
            0x28 => {
                // PLP - Pull Processor Status
                self.sp = self.sp.wrapping_add(1);
                self.p = self.bus.read(0x100 + self.sp as u16) & !0x10;
            }

            // Transfer Instructions
            0xAA => {
                // TAX - Transfer A to X
                self.x = self.a;
                self.set_flag(FLAGS6502::Z, self.x == 0x00);
                self.set_flag(FLAGS6502::N, (self.x & 0x80) != 0);
            }
            0xA8 => {
                // TAY - Transfer A to Y
                self.y = self.a;
                self.set_flag(FLAGS6502::Z, self.y == 0x00);
                self.set_flag(FLAGS6502::N, (self.y & 0x80) != 0);
            }
            0x8A => {
                // TXA - Transfer X to A
                self.a = self.x;
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
            }
            0x98 => {
                // TYA - Transfer Y to A
                self.a = self.y;
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
            }
            0xBA => {
                // TSX - Transfer Stack Pointer to X
                self.x = self.sp;
                self.set_flag(FLAGS6502::Z, self.x == 0x00);
                self.set_flag(FLAGS6502::N, (self.x & 0x80) != 0);
            }
            0x9A => {
                // TXS - Transfer X to Stack Pointer
                self.sp = self.x;
            }

            // ASL - Arithmetic Shift Left
            0x06 => {
                // ASL Zero Page
                let addr = self.bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                let value = self.bus.read(addr as u16);
                self.set_flag(FLAGS6502::C, (value & 0x80) != 0);
                let result = value << 1;
                self.bus.write(addr as u16, result);
                self.set_flag(FLAGS6502::Z, result == 0x00);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
            }
            0x0E => {
                // ASL Absolute
                let lo = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = self.bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = lo | (hi << 8);
                let value = self.bus.read(addr);
                self.set_flag(FLAGS6502::C, (value & 0x80) != 0);
                let result = value << 1;
                self.bus.write(addr, result);
                self.set_flag(FLAGS6502::Z, result == 0x00);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
            }

            // NOP - No Operation
            0xEA => {
                // NOP Implied (official)
                // Does nothing, just takes up time
            }
            0x89 => {
                // NOP Immediate (unofficial)
                self.pc = self.pc.wrapping_add(1); // Skip the immediate byte
            }

            // Flag Instructions
            0x18 => {
                // CLC - Clear Carry Flag
                self.set_flag(FLAGS6502::C, false);
            }
            0x38 => {
                // SEC - Set Carry Flag
                self.set_flag(FLAGS6502::C, true);
            }
            0x58 => {
                // CLI - Clear Interrupt Disable
                self.set_flag(FLAGS6502::I, false);
            }
            0x78 => {
                // SEI - Set Interrupt Disable
                self.set_flag(FLAGS6502::I, true);
            }
            0xB8 => {
                // CLV - Clear Overflow Flag
                self.set_flag(FLAGS6502::V, false);
            }
            0xD8 => {
                // CLD - Clear Decimal Mode
                self.set_flag(FLAGS6502::D, false);
            }
            0xF8 => {
                // SED - Set Decimal Mode
                self.set_flag(FLAGS6502::D, true);
            }

            _ => {
                eprintln!("Unknown opcode: 0x{:02X}", opcode);
            }
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    pub fn set_flag(&mut self, flag: FLAGS6502, value: bool) {
        if value {
            self.p |= flag as u8;
        } else {
            self.p &= !(flag as u8);
        }
    }

    pub fn get_flag(&self, flag: FLAGS6502) -> bool {
        (self.p & (flag as u8)) != 0
    }

    // pub fn trace(cpu: &mut Cpu) -> String {
    //     let ref opscodes: HashMap<u8, &'static opscode::OpsCode> = *opscode::OPSCODES_MAP;
    //     let ref non_readable_addr = *NON_READABLE_ADDR;

    //     let code = cpu.mem_read(cpu.program_counter);
    //     let ops = opscodes.get(&code).unwrap();

    //     let begin = cpu.program_counter;
    //     let mut hex_dump = vec![];
    //     hex_dump.push(code);

    //     let (mem_addr, stored_value) = match ops.mode {
    //         AddressingMode::Immediate
    //         | AddressingMode::NoneAddressing
    //         | AddressingMode::Accumulator => (0, 0),
    //         _ => {
    //             let address = if ops.len == 2 {
    //                 cpu.mem_read(begin + 1) as u16
    //             } else {
    //                 cpu.mem_read_u16(begin + 1)
    //             };
    //             let (_, addr) = ops.mode.get_absolute_addr(cpu, address);
    //             if !non_readable_addr.contains(&addr) {
    //                 (addr, cpu.mem_read(addr))
    //             } else {
    //                 (addr, 0)
    //             }
    //         }
    //     };

    //     let tmp = match ops.len {
    //         1 => match ops.mode {
    //             AddressingMode::Accumulator => format!("A "),
    //             _ => String::from(""),
    //         },
    //         2 => {
    //             let address: u8 = cpu.read(begin + 1);
    //             // let value = cpu.mem_read(address));
    //             hex_dump.push(address);

    //             match ops.mode {
    //                 AddressingMode::Immediate => format!("#${:02x}", address),
    //                 AddressingMode::ZeroPage => format!("${:02x} = {:02x}", mem_addr, stored_value),
    //                 AddressingMode::ZeroPage_X => format!(
    //                     "${:02x},X @ {:02x} = {:02x}",
    //                     address, mem_addr, stored_value
    //                 ),
    //                 AddressingMode::ZeroPage_Y => format!(
    //                     "${:02x},Y @ {:02x} = {:02x}",
    //                     address, mem_addr, stored_value
    //                 ),
    //                 AddressingMode::Indirect_X => format!(
    //                     "(${:02x},X) @ {:02x} = {:04x} = {:02x}",
    //                     address,
    //                     (address.wrapping_add(cpu.x)),
    //                     mem_addr,
    //                     stored_value
    //                 ),
    //                 AddressingMode::Indirect_Y | AddressingMode::Indirect_Y_PageCross => format!(
    //                     "(${:02x}),Y = {:04x} @ {:04x} = {:02x}",
    //                     address,
    //                     (mem_addr.wrapping_sub(cpu.y as u16)),
    //                     mem_addr,
    //                     stored_value
    //                 ),
    //                 AddressingMode::NoneAddressing => {
    //                     // assuming local jumps: BNE, BVS, etc.... todo: check ?
    //                     let address: usize =
    //                         (begin as usize + 2).wrapping_add((address as i8) as usize);
    //                     format!("${:04x}", address)
    //                 }

    //                 _ => panic!(
    //                     "unexpected addressing mode {:?} has ops-len 2. code {:02x}",
    //                     ops.mode, ops.code
    //                 ),
    //             }
    //         }
    //         3 => {
    //             let address_lo = cpu.read(begin + 1);
    //             let address_hi = cpu.read(begin + 2);
    //             hex_dump.push(address_lo);
    //             hex_dump.push(address_hi);

    //             let address = cpu.read(begin + 1);

    //             match ops.mode {
    //                 AddressingMode::NoneAddressing => {
    //                     if ops.code == 0x6c {
    //                         //jmp indirect
    //                         let jmp_addr = if address & 0x00FF == 0x00FF {
    //                             let lo = cpu.read(address as u16);
    //                             let hi = cpu.read(address as u16 & 0xFF00 as u16);
    //                             (hi as u16) << 8 | (lo as u16)
    //                         } else {
    //                             cpu.read(address as u16) as u16
    //                         };

    //                         // let jmp_addr = cpu.mem_read_u16(address);
    //                         format!("(${:04x}) = {:04x}", address, jmp_addr)
    //                     } else {
    //                         format!("${:04x}", address)
    //                     }
    //                 }
    //                 AddressingMode::Absolute => format!("${:04x} = {:02x}", mem_addr, stored_value),
    //                 AddressingMode::Absolute_X | AddressingMode::Absolute_X_PageCross => format!(
    //                     "${:04x},X @ {:04x} = {:02x}",
    //                     address, mem_addr, stored_value
    //                 ),
    //                 AddressingMode::Absolute_Y | AddressingMode::Absolute_Y_PageCross => format!(
    //                     "${:04x},Y @ {:04x} = {:02x}",
    //                     address, mem_addr, stored_value
    //                 ),
    //                 _ => panic!(
    //                     "unexpected addressing mode {:?} has ops-len 3. code {:02x}",
    //                     ops.mode, ops.code
    //                 ),
    //             }
    //         }
    //         _ => String::from(""),
    //     };

    //     let hex_str = hex_dump
    //         .iter()
    //         .map(|z| format!("{:02x}", z))
    //         .collect::<Vec<String>>()
    //         .join(" ");
    //     let asm_str = format!("{:04x}  {:8} {: >4} {}", begin, hex_str, ops.mnemonic, tmp)
    //         .trim()
    //         .to_string();

    //     // let bus_trace = cpu.bus.trace();
    //     format!(
    //         // "{:47} A:{:02x} X:{:02x} Y:{:02x} SP:{:02x} FL:{:08b}",
    //         "{:47} A:{:02x} X:{:02x} Y:{:02x} P:{:02x} SP:{:02x} PPU:{:3},{:3} CYC:{}",
    //         // "{:30}(a:{:x}, x:{:x}, y:{:x}, sp:{:x}, fl:{:x})",
    //         asm_str,
    //         cpu.a,
    //         cpu.x,
    //         cpu.y,
    //         cpu.f,
    //         cpu.stack_pointer,
    //         // bus_trace.ppu_cycles,
    //         bus_trace.ppu_scanline,
    //         bus_trace.cpu_cycles
    //     )
    //     .to_ascii_uppercase()
    // }
}
