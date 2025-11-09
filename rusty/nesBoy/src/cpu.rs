use crate::bus::Bus;

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

pub struct Cpu {
    // CPU registers
    pub a: u8,      // Accumulator
    pub x: u8,      // X register
    pub y: u8,      // Y register
    pub sp: u8,     // Stack pointer
    pub pc: u16,    // Program counter
    pub p: u8,      // Status register

    // 64KB memory (internal RAM)
    pub memory: [u8; 0x10000],
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            a: 0,
            x: 0,
            y: 0,
            sp: 0xFD,
            pc: 0x8000,
            p: 0x24,
            memory: [0; 0x10000],
        }
    }

    pub fn reset(&mut self) {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.sp = 0xFD;
        self.pc = 0x8000;
        self.p = 0x24;
    }

    pub fn step(&mut self, bus: &mut Bus) {
        // Fetch opcode and execute
        let opcode = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        self.execute(opcode, bus);
    }

    pub fn execute(&mut self, opcode: u8, bus: &mut Bus) {
        // Execute instruction based on opcode
        match opcode {
            // LDA - Load Accumulator
            0xA9 => {
                // LDA Immediate
                let value = bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                self.a = value;
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
            }
            0xA5 => {
                // LDA Zero Page
                let addr = bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                self.a = bus.read(addr as u16);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
            }
            0xB5 => {
                // LDA Zero Page,X
                let addr = bus.read(self.pc).wrapping_add(self.x);
                self.pc = self.pc.wrapping_add(1);
                self.a = bus.read(addr as u16);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
            }
            0xAD => {
                // LDA Absolute
                let lo = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = lo | (hi << 8);
                self.a = bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
            }
            0xBD => {
                // LDA Absolute,X
                let lo = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = (lo | (hi << 8)).wrapping_add(self.x as u16);
                self.a = bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
            }
            0xB9 => {
                // LDA Absolute,Y
                let lo = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = (lo | (hi << 8)).wrapping_add(self.y as u16);
                self.a = bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
            }
            0xA1 => {
                // LDA (Indirect,X)
                let zp = bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                let lo = bus.read(zp.wrapping_add(self.x) as u16) as u16;
                let hi = bus.read(zp.wrapping_add(self.x).wrapping_add(1) as u16) as u16;
                let addr = lo | (hi << 8);
                self.a = bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
            }
            0xB1 => {
                // LDA (Indirect),Y
                let zp = bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                let lo = bus.read(zp as u16) as u16;
                let hi = bus.read(zp.wrapping_add(1) as u16) as u16;
                let addr = (lo | (hi << 8)).wrapping_add(self.y as u16);
                self.a = bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
            }

            // LDX - Load X Register
            0xA2 => {
                // LDX Immediate
                self.x = bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                self.set_flag(FLAGS6502::Z, self.x == 0x00);
                self.set_flag(FLAGS6502::N, (self.x & 0x80) != 0);
            }
            0xA6 => {
                // LDX Zero Page
                let addr = bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                self.x = bus.read(addr as u16);
                self.set_flag(FLAGS6502::Z, self.x == 0x00);
                self.set_flag(FLAGS6502::N, (self.x & 0x80) != 0);
            }
            0xB6 => {
                // LDX Zero Page,Y
                let addr = bus.read(self.pc).wrapping_add(self.y);
                self.pc = self.pc.wrapping_add(1);
                self.x = bus.read(addr as u16);
                self.set_flag(FLAGS6502::Z, self.x == 0x00);
                self.set_flag(FLAGS6502::N, (self.x & 0x80) != 0);
            }
            0xAE => {
                // LDX Absolute
                let lo = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = lo | (hi << 8);
                self.x = bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.x == 0x00);
                self.set_flag(FLAGS6502::N, (self.x & 0x80) != 0);
            }
            0xBE => {
                // LDX Absolute,Y
                let lo = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = (lo | (hi << 8)).wrapping_add(self.y as u16);
                self.x = bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.x == 0x00);
                self.set_flag(FLAGS6502::N, (self.x & 0x80) != 0);
            }

            // LDY - Load Y Register
            0xA0 => {
                // LDY Immediate
                self.y = bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                self.set_flag(FLAGS6502::Z, self.y == 0x00);
                self.set_flag(FLAGS6502::N, (self.y & 0x80) != 0);
            }
            0xA4 => {
                // LDY Zero Page
                let addr = bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                self.y = bus.read(addr as u16);
                self.set_flag(FLAGS6502::Z, self.y == 0x00);
                self.set_flag(FLAGS6502::N, (self.y & 0x80) != 0);
            }
            0xB4 => {
                // LDY Zero Page,X
                let addr = bus.read(self.pc).wrapping_add(self.x);
                self.pc = self.pc.wrapping_add(1);
                self.y = bus.read(addr as u16);
                self.set_flag(FLAGS6502::Z, self.y == 0x00);
                self.set_flag(FLAGS6502::N, (self.y & 0x80) != 0);
            }
            0xAC => {
                // LDY Absolute
                let lo = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = lo | (hi << 8);
                self.y = bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.y == 0x00);
                self.set_flag(FLAGS6502::N, (self.y & 0x80) != 0);
            }
            0xBC => {
                // LDY Absolute,X
                let lo = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = (lo | (hi << 8)).wrapping_add(self.x as u16);
                self.y = bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.y == 0x00);
                self.set_flag(FLAGS6502::N, (self.y & 0x80) != 0);
            }

            // STA - Store Accumulator
            0x85 => {
                // STA Zero Page
                let addr = bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                bus.write(addr as u16, self.a);
            }
            0x95 => {
                // STA Zero Page,X
                let addr = bus.read(self.pc).wrapping_add(self.x);
                self.pc = self.pc.wrapping_add(1);
                bus.write(addr as u16, self.a);
            }
            0x8D => {
                // STA Absolute
                let lo = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = lo | (hi << 8);
                bus.write(addr, self.a);
            }
            0x9D => {
                // STA Absolute,X
                let lo = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = (lo | (hi << 8)).wrapping_add(self.x as u16);
                bus.write(addr, self.a);
            }
            0x99 => {
                // STA Absolute,Y
                let lo = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = (lo | (hi << 8)).wrapping_add(self.y as u16);
                bus.write(addr, self.a);
            }
            0x81 => {
                // STA (Indirect,X)
                let zp = bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                let lo = bus.read(zp.wrapping_add(self.x) as u16) as u16;
                let hi = bus.read(zp.wrapping_add(self.x).wrapping_add(1) as u16) as u16;
                let addr = lo | (hi << 8);
                bus.write(addr, self.a);
            }
            0x91 => {
                // STA (Indirect),Y
                let zp = bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                let lo = bus.read(zp as u16) as u16;
                let hi = bus.read(zp.wrapping_add(1) as u16) as u16;
                let addr = (lo | (hi << 8)).wrapping_add(self.y as u16);
                bus.write(addr, self.a);
            }

            // STX - Store X Register
            0x86 => {
                // STX Zero Page
                let addr = bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                bus.write(addr as u16, self.x);
            }
            0x96 => {
                // STX Zero Page,Y
                let addr = bus.read(self.pc).wrapping_add(self.y);
                self.pc = self.pc.wrapping_add(1);
                bus.write(addr as u16, self.x);
            }
            0x8E => {
                // STX Absolute
                let lo = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = lo | (hi << 8);
                bus.write(addr, self.x);
            }

            // STY - Store Y Register
            0x84 => {
                // STY Zero Page
                let addr = bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                bus.write(addr as u16, self.y);
            }
            0x94 => {
                // STY Zero Page,X
                let addr = bus.read(self.pc).wrapping_add(self.x);
                self.pc = self.pc.wrapping_add(1);
                bus.write(addr as u16, self.y);
            }
            0x8C => {
                // STY Absolute
                let lo = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = lo | (hi << 8);
                bus.write(addr, self.y);
            }

            // ADC - Add with Carry
            0x69 => {
                // ADC Immediate
                let value = bus.read(self.pc);
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
                let addr = bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                let value = bus.read(addr as u16);
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
                let lo = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = lo | (hi << 8);
                let value = bus.read(addr);
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
                let value = bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                self.a &= value;
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
            }
            0x25 => {
                // AND Zero Page
                let addr = bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                self.a &= bus.read(addr as u16);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
            }
            0x35 => {
                // AND Zero Page,X
                let addr = bus.read(self.pc).wrapping_add(self.x);
                self.pc = self.pc.wrapping_add(1);
                self.a &= bus.read(addr as u16);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
            }
            0x2D => {
                // AND Absolute
                let lo = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = lo | (hi << 8);
                self.a &= bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
            }
            0x3D => {
                // AND Absolute,X
                let lo = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = (lo | (hi << 8)).wrapping_add(self.x as u16);
                self.a &= bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
            }
            0x39 => {
                // AND Absolute,Y
                let lo = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = (lo | (hi << 8)).wrapping_add(self.y as u16);
                self.a &= bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
            }
            0x21 => {
                // AND (Indirect,X)
                let zp = bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                let lo = bus.read(zp.wrapping_add(self.x) as u16) as u16;
                let hi = bus.read(zp.wrapping_add(self.x).wrapping_add(1) as u16) as u16;
                let addr = lo | (hi << 8);
                self.a &= bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
            }
            0x31 => {
                // AND (Indirect),Y
                let zp = bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                let lo = bus.read(zp as u16) as u16;
                let hi = bus.read(zp.wrapping_add(1) as u16) as u16;
                let addr = (lo | (hi << 8)).wrapping_add(self.y as u16);
                self.a &= bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
            }

            // ORA - Logical OR
            0x09 => {
                // ORA Immediate
                let value = bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                self.a |= value;
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
            }
            0x05 => {
                // ORA Zero Page
                let addr = bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                self.a |= bus.read(addr as u16);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
            }
            0x15 => {
                // ORA Zero Page,X
                let addr = bus.read(self.pc).wrapping_add(self.x);
                self.pc = self.pc.wrapping_add(1);
                self.a |= bus.read(addr as u16);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
            }
            0x0D => {
                // ORA Absolute
                let lo = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = lo | (hi << 8);
                self.a |= bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
            }

            // EOR - Logical XOR
            0x49 => {
                // EOR Immediate
                let value = bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                self.a ^= value;
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
            }
            0x45 => {
                // EOR Zero Page
                let addr = bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                self.a ^= bus.read(addr as u16);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
            }
            0x4D => {
                // EOR Absolute
                let lo = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = lo | (hi << 8);
                self.a ^= bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
            }

            // CMP - Compare Accumulator
            0xC9 => {
                // CMP Immediate
                let value = bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                let result = self.a.wrapping_sub(value);
                self.set_flag(FLAGS6502::C, self.a >= value);
                self.set_flag(FLAGS6502::Z, self.a == value);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
            }
            0xC5 => {
                // CMP Zero Page
                let addr = bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                let value = bus.read(addr as u16);
                let result = self.a.wrapping_sub(value);
                self.set_flag(FLAGS6502::C, self.a >= value);
                self.set_flag(FLAGS6502::Z, self.a == value);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
            }
            0xD5 => {
                // CMP Zero Page,X
                let addr = bus.read(self.pc).wrapping_add(self.x);
                self.pc = self.pc.wrapping_add(1);
                let value = bus.read(addr as u16);
                let result = self.a.wrapping_sub(value);
                self.set_flag(FLAGS6502::C, self.a >= value);
                self.set_flag(FLAGS6502::Z, self.a == value);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
            }
            0xCD => {
                // CMP Absolute
                let lo = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = lo | (hi << 8);
                let value = bus.read(addr);
                let result = self.a.wrapping_sub(value);
                self.set_flag(FLAGS6502::C, self.a >= value);
                self.set_flag(FLAGS6502::Z, self.a == value);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
            }
            0xDD => {
                // CMP Absolute,X
                let lo = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = (lo | (hi << 8)).wrapping_add(self.x as u16);
                let value = bus.read(addr);
                let result = self.a.wrapping_sub(value);
                self.set_flag(FLAGS6502::C, self.a >= value);
                self.set_flag(FLAGS6502::Z, self.a == value);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
            }
            0xD9 => {
                // CMP Absolute,Y
                let lo = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = (lo | (hi << 8)).wrapping_add(self.y as u16);
                let value = bus.read(addr);
                let result = self.a.wrapping_sub(value);
                self.set_flag(FLAGS6502::C, self.a >= value);
                self.set_flag(FLAGS6502::Z, self.a == value);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
            }
            0xC1 => {
                // CMP (Indirect,X)
                let zp = bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                let lo = bus.read(zp.wrapping_add(self.x) as u16) as u16;
                let hi = bus.read(zp.wrapping_add(self.x).wrapping_add(1) as u16) as u16;
                let addr = lo | (hi << 8);
                let value = bus.read(addr);
                let result = self.a.wrapping_sub(value);
                self.set_flag(FLAGS6502::C, self.a >= value);
                self.set_flag(FLAGS6502::Z, self.a == value);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
            }
            0xD1 => {
                // CMP (Indirect),Y
                let zp = bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                let lo = bus.read(zp as u16) as u16;
                let hi = bus.read(zp.wrapping_add(1) as u16) as u16;
                let addr = (lo | (hi << 8)).wrapping_add(self.y as u16);
                let value = bus.read(addr);
                let result = self.a.wrapping_sub(value);
                self.set_flag(FLAGS6502::C, self.a >= value);
                self.set_flag(FLAGS6502::Z, self.a == value);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
            }

            // CPX - Compare X Register
            0xE0 => {
                // CPX Immediate
                let value = bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                let result = self.x.wrapping_sub(value);
                self.set_flag(FLAGS6502::C, self.x >= value);
                self.set_flag(FLAGS6502::Z, self.x == value);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
            }
            0xE4 => {
                // CPX Zero Page
                let addr = bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                let value = bus.read(addr as u16);
                let result = self.x.wrapping_sub(value);
                self.set_flag(FLAGS6502::C, self.x >= value);
                self.set_flag(FLAGS6502::Z, self.x == value);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
            }
            0xEC => {
                // CPX Absolute
                let lo = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = lo | (hi << 8);
                let value = bus.read(addr);
                let result = self.x.wrapping_sub(value);
                self.set_flag(FLAGS6502::C, self.x >= value);
                self.set_flag(FLAGS6502::Z, self.x == value);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
            }

            // CPY - Compare Y Register
            0xC0 => {
                // CPY Immediate
                let value = bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                let result = self.y.wrapping_sub(value);
                self.set_flag(FLAGS6502::C, self.y >= value);
                self.set_flag(FLAGS6502::Z, self.y == value);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
            }
            0xC4 => {
                // CPY Zero Page
                let addr = bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                let value = bus.read(addr as u16);
                let result = self.y.wrapping_sub(value);
                self.set_flag(FLAGS6502::C, self.y >= value);
                self.set_flag(FLAGS6502::Z, self.y == value);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
            }
            0xCC => {
                // CPY Absolute
                let lo = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = lo | (hi << 8);
                let value = bus.read(addr);
                let result = self.y.wrapping_sub(value);
                self.set_flag(FLAGS6502::C, self.y >= value);
                self.set_flag(FLAGS6502::Z, self.y == value);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
            }

            // JMP - Jump
            0x4C => {
                // JMP Absolute
                let lo = bus.read(self.pc) as u16;
                let hi = bus.read(self.pc.wrapping_add(1)) as u16;
                self.pc = lo | (hi << 8);
            }
            0x6C => {
                // JMP Indirect (with 6502 bug)
                let lo = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = lo | (hi << 8);
                // 6502 bug: if addr is at page boundary (e.g., 0x12FF),
                // it wraps around to 0x1200 instead of 0x1300
                if (addr & 0xFF) == 0xFF {
                    let lo = bus.read(addr) as u16;
                    let hi = bus.read(addr & 0xFF00) as u16;
                    self.pc = lo | (hi << 8);
                } else {
                    let lo = bus.read(addr) as u16;
                    let hi = bus.read(addr.wrapping_add(1)) as u16;
                    self.pc = lo | (hi << 8);
                }
            }

            // JSR - Jump to Subroutine
            0x20 => {
                let lo = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = lo | (hi << 8);
                let ret_addr = self.pc.wrapping_sub(1);
                bus.write(0x100 + self.sp as u16, (ret_addr >> 8) as u8);
                self.sp = self.sp.wrapping_sub(1);
                bus.write(0x100 + self.sp as u16, (ret_addr & 0xFF) as u8);
                self.sp = self.sp.wrapping_sub(1);
                self.pc = addr;
            }

            // RTS - Return from Subroutine
            0x60 => {
                self.sp = self.sp.wrapping_add(1);
                let lo = bus.read(0x100 + self.sp as u16) as u16;
                self.sp = self.sp.wrapping_add(1);
                let hi = bus.read(0x100 + self.sp as u16) as u16;
                self.pc = (lo | (hi << 8)).wrapping_add(1);
            }

            // Branch Instructions
            0xF0 => {
                // BEQ - Branch if Equal
                let offset = bus.read(self.pc) as i8;
                self.pc = self.pc.wrapping_add(1);
                if self.get_flag(FLAGS6502::Z) {
                    self.pc = self.pc.wrapping_add(offset as u16);
                }
            }
            0xD0 => {
                // BNE - Branch if Not Equal
                let offset = bus.read(self.pc) as i8;
                self.pc = self.pc.wrapping_add(1);
                if !self.get_flag(FLAGS6502::Z) {
                    self.pc = self.pc.wrapping_add(offset as u16);
                }
            }
            0xB0 => {
                // BCS - Branch if Carry Set
                let offset = bus.read(self.pc) as i8;
                self.pc = self.pc.wrapping_add(1);
                if self.get_flag(FLAGS6502::C) {
                    self.pc = self.pc.wrapping_add(offset as u16);
                }
            }
            0x90 => {
                // BCC - Branch if Carry Clear
                let offset = bus.read(self.pc) as i8;
                self.pc = self.pc.wrapping_add(1);
                if !self.get_flag(FLAGS6502::C) {
                    self.pc = self.pc.wrapping_add(offset as u16);
                }
            }
            0x30 => {
                // BMI - Branch if Minus
                let offset = bus.read(self.pc) as i8;
                self.pc = self.pc.wrapping_add(1);
                if self.get_flag(FLAGS6502::N) {
                    self.pc = self.pc.wrapping_add(offset as u16);
                }
            }
            0x10 => {
                // BPL - Branch if Positive
                let offset = bus.read(self.pc) as i8;
                self.pc = self.pc.wrapping_add(1);
                if !self.get_flag(FLAGS6502::N) {
                    self.pc = self.pc.wrapping_add(offset as u16);
                }
            }
            0x70 => {
                // BVS - Branch if Overflow Set
                let offset = bus.read(self.pc) as i8;
                self.pc = self.pc.wrapping_add(1);
                if self.get_flag(FLAGS6502::V) {
                    self.pc = self.pc.wrapping_add(offset as u16);
                }
            }
            0x50 => {
                // BVC - Branch if Overflow Clear
                let offset = bus.read(self.pc) as i8;
                self.pc = self.pc.wrapping_add(1);
                if !self.get_flag(FLAGS6502::V) {
                    self.pc = self.pc.wrapping_add(offset as u16);
                }
            }

            // INC - Increment Memory
            0xE6 => {
                // INC Zero Page
                let addr = bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                let value = bus.read(addr as u16).wrapping_add(1);
                bus.write(addr as u16, value);
                self.set_flag(FLAGS6502::Z, value == 0x00);
                self.set_flag(FLAGS6502::N, (value & 0x80) != 0);
            }
            0xF6 => {
                // INC Zero Page,X
                let addr = bus.read(self.pc).wrapping_add(self.x);
                self.pc = self.pc.wrapping_add(1);
                let value = bus.read(addr as u16).wrapping_add(1);
                bus.write(addr as u16, value);
                self.set_flag(FLAGS6502::Z, value == 0x00);
                self.set_flag(FLAGS6502::N, (value & 0x80) != 0);
            }
            0xEE => {
                // INC Absolute
                let lo = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = lo | (hi << 8);
                let value = bus.read(addr).wrapping_add(1);
                bus.write(addr, value);
                self.set_flag(FLAGS6502::Z, value == 0x00);
                self.set_flag(FLAGS6502::N, (value & 0x80) != 0);
            }
            0xFE => {
                // INC Absolute,X
                let lo = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = (lo | (hi << 8)).wrapping_add(self.x as u16);
                let value = bus.read(addr).wrapping_add(1);
                bus.write(addr, value);
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
                let addr = bus.read(self.pc);
                self.pc = self.pc.wrapping_add(1);
                let value = bus.read(addr as u16).wrapping_sub(1);
                bus.write(addr as u16, value);
                self.set_flag(FLAGS6502::Z, value == 0x00);
                self.set_flag(FLAGS6502::N, (value & 0x80) != 0);
            }
            0xD6 => {
                // DEC Zero Page,X
                let addr = bus.read(self.pc).wrapping_add(self.x);
                self.pc = self.pc.wrapping_add(1);
                let value = bus.read(addr as u16).wrapping_sub(1);
                bus.write(addr as u16, value);
                self.set_flag(FLAGS6502::Z, value == 0x00);
                self.set_flag(FLAGS6502::N, (value & 0x80) != 0);
            }
            0xCE => {
                // DEC Absolute
                let lo = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = lo | (hi << 8);
                let value = bus.read(addr).wrapping_sub(1);
                bus.write(addr, value);
                self.set_flag(FLAGS6502::Z, value == 0x00);
                self.set_flag(FLAGS6502::N, (value & 0x80) != 0);
            }
            0xDE => {
                // DEC Absolute,X
                let lo = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let hi = bus.read(self.pc) as u16;
                self.pc = self.pc.wrapping_add(1);
                let addr = (lo | (hi << 8)).wrapping_add(self.x as u16);
                let value = bus.read(addr).wrapping_sub(1);
                bus.write(addr, value);
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
                bus.write(0x100 + self.sp as u16, self.a);
                self.sp = self.sp.wrapping_sub(1);
            }
            0x68 => {
                // PLA - Pull Accumulator
                self.sp = self.sp.wrapping_add(1);
                self.a = bus.read(0x100 + self.sp as u16);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
            }
            0x08 => {
                // PHP - Push Processor Status
                bus.write(0x100 + self.sp as u16, self.p | 0x10);
                self.sp = self.sp.wrapping_sub(1);
            }
            0x28 => {
                // PLP - Pull Processor Status
                self.sp = self.sp.wrapping_add(1);
                self.p = bus.read(0x100 + self.sp as u16) & !0x10;
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
}
