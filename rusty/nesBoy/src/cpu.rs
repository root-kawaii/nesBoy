use crate::bus::Bus;
use crate::sleep;
use bitflags::bitflags;
use lazy_static::lazy_static;
use std::time::Duration;

bitflags! {
/// # Status Register (P) http://wiki.nesdev.com/w/index.php/Status_flags
///
///  7 6 5 4 3 2 1 0
///  N V _ B D I Z C
///  | |   | | | | +--- Carry Flag
///  | |   | | | +----- Zero Flag
///  | |   | | +------- Interrupt Disable
///  | |   | +--------- Decimal Mode (not used on NES)
///  | |   +----------- Break Command
///  | +--------------- Overflow Flag
///  +----------------- Negative Flag
///
    pub struct CpuFlags: u8 {
        const CARRY             = 0b00000001;
        const ZERO              = 0b00000010;
        const INTERRUPT_DISABLE = 0b00000100;
        const DECIMAL_MODE      = 0b00001000;
        const BREAK             = 0b00010000;
        const BREAK2            = 0b00100000;
        const OVERFLOW          = 0b01000000;
        const NEGATIV           = 0b10000000;
    }
}

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

const STACK: u16 = 0x0100;

pub struct Cpu<'a> {
    // CPU registers
    pub a: u8,                // Accumulator
    pub x: u8,                // X register
    pub y: u8,                // Y register
    pub stack_pointer: u8,    // Stack pointer
    pub program_counter: u16, // Program counter
    pub p: u8,                // Status register
    pub status: CpuFlags,

    // 64KB memory (internal RAM)
    pub memory: [u8; 0x10000],
    pub bus: Bus<'a>,
}

impl<'a> Cpu<'a> {
    pub fn new(bus: Bus<'a>) -> Self {
        Cpu {
            a: 0,
            x: 0,
            y: 0,
            stack_pointer: 0xFD,
            program_counter: 0xC000,
            p: 0x24,
            status: CpuFlags::from_bits_truncate(0x24),
            memory: [0; 0x10000],
            bus,
        }
    }

    pub fn reset(&mut self) {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.stack_pointer = 0xFD;
        self.p = 0x24;
        self.status = CpuFlags::from_bits_truncate(0x24);

        // Read the reset vector from 0xFFFC-0xFFFD
        self.program_counter = self.mem_read_u16(0xFFFC);
    }

    pub fn run(&mut self) {
        loop {
            // sleep(Duration::from_nanos(1 as u64));

            self.step();
        }
    }

    pub fn step(&mut self) {
        if let Some(_nmi) = self.bus.poll_nmi_status() {
            self.interrupt_nmi();
        }
        // Fetch opcode and execute
        let opcode = self.bus.read(self.program_counter);
        // println!("Executing opcode: {}, {}", opcode, self.program_counter);
        self.program_counter = self.program_counter.wrapping_add(1); // Move past opcode byte
        // println!("PC: {:04X} Opcode: {:02X}", self.program_counter - 1, opcode);
        let cycles = self.execute(opcode); // Get actual cycle count from instruction
        self.bus.tick(cycles); // Tick PPU with correct number of cycles
    }

    fn interrupt_nmi(&mut self) {
        self.stack_push_u16(self.program_counter);
        let mut flag = self.status.clone();
        flag.set(CpuFlags::BREAK, false);
        flag.set(CpuFlags::BREAK2, true);

        self.stack_push(flag.bits());
        self.status.insert(CpuFlags::INTERRUPT_DISABLE);

        self.bus.tick(2);
        self.program_counter = self.mem_read_u16(0xfffA);
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    fn stack_pop(&mut self) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        self.mem_read((STACK as u16) + self.stack_pointer as u16)
    }

    fn stack_push(&mut self, data: u8) {
        self.mem_write((STACK as u16) + self.stack_pointer as u16, data);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1)
    }

    fn stack_push_u16(&mut self, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xff) as u8;
        self.stack_push(hi);
        self.stack_push(lo);
        // self.mem_write_u16((STACK as u16) + self.stack_pointer as u16, data);
        // self.stack_pointer = self.stack_pointer.wrapping_sub(2);
    }

    fn stack_pop_u16(&mut self) -> u16 {
        let lo = self.stack_pop() as u16;
        let hi = self.stack_pop() as u16;

        hi << 8 | lo
        // self.stack_pointer = self.stack_pointer.wrapping_add(2);
        // self.mem_read_u16((STACK as u16) + self.stack_pointer as u16)
    }

    pub(super) fn mem_read(&mut self, pos: u16) -> u8 {
        self.bus.read(pos)
    }

    pub(super) fn mem_read_u16(&mut self, pos: u16) -> u16 {
        self.bus.read_u16(pos)
    }

    pub(super) fn mem_write(&mut self, pos: u16, data: u8) {
        self.bus.write(pos, data);
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

    pub fn execute(&mut self, opcode: u8) -> u8 {
        // print!("op cod{} \n", opcode);
        // Execute instruction based on opcode
        match opcode {
            // BRK - Break/Software Interrupt
            0x00 => {
                self.program_counter = self.program_counter.wrapping_add(1);
                // Push PC to stack
                self.bus.write(
                    0x100 + self.stack_pointer as u16,
                    (self.program_counter >> 8) as u8,
                );
                self.stack_pointer = self.stack_pointer.wrapping_sub(1);
                self.bus.write(
                    0x100 + self.stack_pointer as u16,
                    (self.program_counter & 0xFF) as u8,
                );
                self.stack_pointer = self.stack_pointer.wrapping_sub(1);
                // Push status register with B flag set
                self.bus
                    .write(0x100 + self.stack_pointer as u16, self.p | 0x30);
                self.stack_pointer = self.stack_pointer.wrapping_sub(1);
                // Set interrupt disable flag
                self.set_flag(FLAGS6502::I, true);
                // Load PC from IRQ/BRK vector at 0xFFFE/0xFFFF
                let lo = self.bus.read(0xFFFE) as u16;
                let hi = self.bus.read(0xFFFF) as u16;
                self.program_counter = lo | (hi << 8);
                7
            }

            // LDA - Load Accumulator
            0xA9 => {
                // LDA Immediate
                let value = self.bus.read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);
                self.a = value;
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
                2
            }
            0xA5 => {
                // LDA Zero Page
                let addr = self.bus.read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);
                self.a = self.bus.read(addr as u16);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
                3
            }
            0xB5 => {
                // LDA Zero Page,X
                let addr = self.bus.read(self.program_counter).wrapping_add(self.x);
                self.program_counter = self.program_counter.wrapping_add(1);
                self.a = self.bus.read(addr as u16);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
                4
            }
            0xAD => {
                // LDA Absolute
                let lo = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let hi = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let addr = lo | (hi << 8);
                self.a = self.bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
                4
            }
            0xBD => {
                // LDA Absolute,X
                let lo = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let hi = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let addr = (lo | (hi << 8)).wrapping_add(self.x as u16);
                self.a = self.bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
                4
            }
            0xB9 => {
                // LDA Absolute,Y
                let lo = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let hi = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let addr = (lo | (hi << 8)).wrapping_add(self.y as u16);
                self.a = self.bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
                4
            }
            0xA1 => {
                // LDA (Indirect,X)
                let zp = self.bus.read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);
                let lo = self.bus.read(zp.wrapping_add(self.x) as u16) as u16;
                let hi = self
                    .bus
                    .read(zp.wrapping_add(self.x).wrapping_add(1) as u16)
                    as u16;
                let addr = lo | (hi << 8);
                self.a = self.bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
                6
            }
            0xB1 => {
                // LDA (Indirect),Y
                let zp = self.bus.read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);
                let lo = self.bus.read(zp as u16) as u16;
                let hi = self.bus.read(zp.wrapping_add(1) as u16) as u16;
                let addr = (lo | (hi << 8)).wrapping_add(self.y as u16);
                self.a = self.bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
                5
            }

            // LDX - Load X Register
            0xA2 => {
                // LDX Immediate
                self.x = self.bus.read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);
                self.set_flag(FLAGS6502::Z, self.x == 0x00);
                self.set_flag(FLAGS6502::N, (self.x & 0x80) != 0);
                2
            }
            0xA6 => {
                // LDX Zero Page
                let addr = self.bus.read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);
                self.x = self.bus.read(addr as u16);
                self.set_flag(FLAGS6502::Z, self.x == 0x00);
                self.set_flag(FLAGS6502::N, (self.x & 0x80) != 0);
                3
            }
            0xB6 => {
                // LDX Zero Page,Y
                let addr = self.bus.read(self.program_counter).wrapping_add(self.y);
                self.program_counter = self.program_counter.wrapping_add(1);
                self.x = self.bus.read(addr as u16);
                self.set_flag(FLAGS6502::Z, self.x == 0x00);
                self.set_flag(FLAGS6502::N, (self.x & 0x80) != 0);
                4
            }
            0xAE => {
                // LDX Absolute
                let lo = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let hi = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let addr = lo | (hi << 8);
                self.x = self.bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.x == 0x00);
                self.set_flag(FLAGS6502::N, (self.x & 0x80) != 0);
                4
            }
            0xBE => {
                // LDX Absolute,Y
                let lo = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let hi = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let addr = (lo | (hi << 8)).wrapping_add(self.y as u16);
                self.x = self.bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.x == 0x00);
                self.set_flag(FLAGS6502::N, (self.x & 0x80) != 0);
                4
            }

            // LDY - Load Y Register
            0xA0 => {
                // LDY Immediate
                self.y = self.bus.read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);
                self.set_flag(FLAGS6502::Z, self.y == 0x00);
                self.set_flag(FLAGS6502::N, (self.y & 0x80) != 0);
                2
            }
            0xA4 => {
                // LDY Zero Page
                let addr = self.bus.read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);
                self.y = self.bus.read(addr as u16);
                self.set_flag(FLAGS6502::Z, self.y == 0x00);
                self.set_flag(FLAGS6502::N, (self.y & 0x80) != 0);
                3
            }
            0xB4 => {
                // LDY Zero Page,X
                let addr = self.bus.read(self.program_counter).wrapping_add(self.x);
                self.program_counter = self.program_counter.wrapping_add(1);
                self.y = self.bus.read(addr as u16);
                self.set_flag(FLAGS6502::Z, self.y == 0x00);
                self.set_flag(FLAGS6502::N, (self.y & 0x80) != 0);
                4
            }
            0xAC => {
                // LDY Absolute
                let lo = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let hi = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let addr = lo | (hi << 8);
                self.y = self.bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.y == 0x00);
                self.set_flag(FLAGS6502::N, (self.y & 0x80) != 0);
                4
            }
            0xBC => {
                // LDY Absolute,X
                let lo = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let hi = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let addr = (lo | (hi << 8)).wrapping_add(self.x as u16);
                self.y = self.bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.y == 0x00);
                self.set_flag(FLAGS6502::N, (self.y & 0x80) != 0);
                4
            }

            // STA - Store Accumulator
            0x85 => {
                // STA Zero Page
                let addr = self.bus.read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);
                self.bus.write(addr as u16, self.a);
                3
            }
            0x95 => {
                // STA Zero Page,X
                let addr = self.bus.read(self.program_counter).wrapping_add(self.x);
                self.program_counter = self.program_counter.wrapping_add(1);
                self.bus.write(addr as u16, self.a);
                4
            }
            0x8D => {
                // STA Absolute
                let lo = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let hi = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let addr = lo | (hi << 8);
                self.bus.write(addr, self.a);
                4
            }
            0x9D => {
                // STA Absolute,X
                let lo = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let hi = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let addr = (lo | (hi << 8)).wrapping_add(self.x as u16);
                self.bus.write(addr, self.a);
                5
            }
            0x99 => {
                // STA Absolute,Y
                let lo = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let hi = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let addr = (lo | (hi << 8)).wrapping_add(self.y as u16);
                self.bus.write(addr, self.a);
                5
            }
            0x81 => {
                // STA (Indirect,X)
                let zp = self.bus.read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);
                let lo = self.bus.read(zp.wrapping_add(self.x) as u16) as u16;
                let hi = self
                    .bus
                    .read(zp.wrapping_add(self.x).wrapping_add(1) as u16)
                    as u16;
                let addr = lo | (hi << 8);
                self.bus.write(addr, self.a);
                6
            }
            0x91 => {
                // STA (Indirect),Y
                let zp = self.bus.read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);
                let lo = self.bus.read(zp as u16) as u16;
                let hi = self.bus.read(zp.wrapping_add(1) as u16) as u16;
                let addr = (lo | (hi << 8)).wrapping_add(self.y as u16);
                self.bus.write(addr, self.a);
                6
            }

            // STX - Store X Register
            0x86 => {
                // STX Zero Page
                let addr = self.bus.read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);
                self.bus.write(addr as u16, self.x);
                3
            }
            0x96 => {
                // STX Zero Page,Y
                let addr = self.bus.read(self.program_counter).wrapping_add(self.y);
                self.program_counter = self.program_counter.wrapping_add(1);
                self.bus.write(addr as u16, self.x);
                4
            }
            0x8E => {
                // STX Absolute
                let lo = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let hi = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let addr = lo | (hi << 8);
                self.bus.write(addr, self.x);
                4
            }

            // STY - Store Y Register
            0x84 => {
                // STY Zero Page
                let addr = self.bus.read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);
                self.bus.write(addr as u16, self.y);
                3
            }
            0x94 => {
                // STY Zero Page,X
                let addr = self.bus.read(self.program_counter).wrapping_add(self.x);
                self.program_counter = self.program_counter.wrapping_add(1);
                self.bus.write(addr as u16, self.y);
                4
            }
            0x8C => {
                // STY Absolute
                let lo = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let hi = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let addr = lo | (hi << 8);
                self.bus.write(addr, self.y);
                4
            }

            // ADC - Add with Carry
            0x69 => {
                // ADC Immediate
                let value = self.bus.read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);
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
                2
            }
            0x65 => {
                // ADC Zero Page
                let addr = self.bus.read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);
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
                3
            }
            0x75 => {
                // ADC Zero Page,X
                let addr = self.bus.read(self.program_counter).wrapping_add(self.x);
                self.program_counter = self.program_counter.wrapping_add(1);
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
                4
            }
            0x6D => {
                // ADC Absolute
                let lo = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let hi = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
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
                4
            }
            0x7D => {
                // ADC Absolute,X
                let lo = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let hi = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let addr = (lo | (hi << 8)).wrapping_add(self.x as u16);
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
                4
            }
            0x79 => {
                // ADC Absolute,Y
                let lo = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let hi = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let addr = (lo | (hi << 8)).wrapping_add(self.y as u16);
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
                4
            }
            0x61 => {
                // ADC (Indirect,X)
                let zp = self.bus.read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);
                let lo = self.bus.read(zp.wrapping_add(self.x) as u16) as u16;
                let hi = self
                    .bus
                    .read(zp.wrapping_add(self.x).wrapping_add(1) as u16)
                    as u16;
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
                6
            }
            0x71 => {
                // ADC (Indirect),Y
                let zp = self.bus.read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);
                let lo = self.bus.read(zp as u16) as u16;
                let hi = self.bus.read(zp.wrapping_add(1) as u16) as u16;
                let addr = (lo | (hi << 8)).wrapping_add(self.y as u16);
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
                5
            }

            // SBC - Subtract with Carry
            0xE9 => {
                // SBC Immediate
                let value = self.bus.read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);
                let carry = if self.get_flag(FLAGS6502::C) { 0 } else { 1 };
                let result = (self.a as u16)
                    .wrapping_sub(value as u16)
                    .wrapping_sub(carry);
                self.set_flag(FLAGS6502::C, result < 0x100);
                self.set_flag(FLAGS6502::Z, (result & 0xFF) == 0);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
                self.set_flag(
                    FLAGS6502::V,
                    ((self.a ^ value) & 0x80) != 0 && ((self.a ^ (result as u8)) & 0x80) != 0,
                );
                self.a = (result & 0xFF) as u8;
                2
            }
            0xE5 => {
                // SBC Zero Page
                let addr = self.bus.read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);
                let value = self.bus.read(addr as u16);
                let carry = if self.get_flag(FLAGS6502::C) { 0 } else { 1 };
                let result = (self.a as u16)
                    .wrapping_sub(value as u16)
                    .wrapping_sub(carry);
                self.set_flag(FLAGS6502::C, result < 0x100);
                self.set_flag(FLAGS6502::Z, (result & 0xFF) == 0);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
                self.set_flag(
                    FLAGS6502::V,
                    ((self.a ^ value) & 0x80) != 0 && ((self.a ^ (result as u8)) & 0x80) != 0,
                );
                self.a = (result & 0xFF) as u8;
                3
            }
            0xF5 => {
                // SBC Zero Page,X
                let addr = self.bus.read(self.program_counter).wrapping_add(self.x);
                self.program_counter = self.program_counter.wrapping_add(1);
                let value = self.bus.read(addr as u16);
                let carry = if self.get_flag(FLAGS6502::C) { 0 } else { 1 };
                let result = (self.a as u16)
                    .wrapping_sub(value as u16)
                    .wrapping_sub(carry);
                self.set_flag(FLAGS6502::C, result < 0x100);
                self.set_flag(FLAGS6502::Z, (result & 0xFF) == 0);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
                self.set_flag(
                    FLAGS6502::V,
                    ((self.a ^ value) & 0x80) != 0 && ((self.a ^ (result as u8)) & 0x80) != 0,
                );
                self.a = (result & 0xFF) as u8;
                4
            }
            0xED => {
                // SBC Absolute
                let lo = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let hi = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let addr = lo | (hi << 8);
                let value = self.bus.read(addr);
                let carry = if self.get_flag(FLAGS6502::C) { 0 } else { 1 };
                let result = (self.a as u16)
                    .wrapping_sub(value as u16)
                    .wrapping_sub(carry);
                self.set_flag(FLAGS6502::C, result < 0x100);
                self.set_flag(FLAGS6502::Z, (result & 0xFF) == 0);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
                self.set_flag(
                    FLAGS6502::V,
                    ((self.a ^ value) & 0x80) != 0 && ((self.a ^ (result as u8)) & 0x80) != 0,
                );
                self.a = (result & 0xFF) as u8;
                4
            }
            0xFD => {
                // SBC Absolute,X
                let lo = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let hi = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let addr = (lo | (hi << 8)).wrapping_add(self.x as u16);
                let value = self.bus.read(addr);
                let carry = if self.get_flag(FLAGS6502::C) { 0 } else { 1 };
                let result = (self.a as u16)
                    .wrapping_sub(value as u16)
                    .wrapping_sub(carry);
                self.set_flag(FLAGS6502::C, result < 0x100);
                self.set_flag(FLAGS6502::Z, (result & 0xFF) == 0);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
                self.set_flag(
                    FLAGS6502::V,
                    ((self.a ^ value) & 0x80) != 0 && ((self.a ^ (result as u8)) & 0x80) != 0,
                );
                self.a = (result & 0xFF) as u8;
                4
            }
            0xF9 => {
                // SBC Absolute,Y
                let lo = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let hi = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let addr = (lo | (hi << 8)).wrapping_add(self.y as u16);
                let value = self.bus.read(addr);
                let carry = if self.get_flag(FLAGS6502::C) { 0 } else { 1 };
                let result = (self.a as u16)
                    .wrapping_sub(value as u16)
                    .wrapping_sub(carry);
                self.set_flag(FLAGS6502::C, result < 0x100);
                self.set_flag(FLAGS6502::Z, (result & 0xFF) == 0);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
                self.set_flag(
                    FLAGS6502::V,
                    ((self.a ^ value) & 0x80) != 0 && ((self.a ^ (result as u8)) & 0x80) != 0,
                );
                self.a = (result & 0xFF) as u8;
                4
            }
            0xE1 => {
                // SBC (Indirect,X)
                let zp = self.bus.read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);
                let lo = self.bus.read(zp.wrapping_add(self.x) as u16) as u16;
                let hi = self
                    .bus
                    .read(zp.wrapping_add(self.x).wrapping_add(1) as u16)
                    as u16;
                let addr = lo | (hi << 8);
                let value = self.bus.read(addr);
                let carry = if self.get_flag(FLAGS6502::C) { 0 } else { 1 };
                let result = (self.a as u16)
                    .wrapping_sub(value as u16)
                    .wrapping_sub(carry);
                self.set_flag(FLAGS6502::C, result < 0x100);
                self.set_flag(FLAGS6502::Z, (result & 0xFF) == 0);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
                self.set_flag(
                    FLAGS6502::V,
                    ((self.a ^ value) & 0x80) != 0 && ((self.a ^ (result as u8)) & 0x80) != 0,
                );
                self.a = (result & 0xFF) as u8;
                6
            }
            0xF1 => {
                // SBC (Indirect),Y
                let zp = self.bus.read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);
                let lo = self.bus.read(zp as u16) as u16;
                let hi = self.bus.read(zp.wrapping_add(1) as u16) as u16;
                let addr = (lo | (hi << 8)).wrapping_add(self.y as u16);
                let value = self.bus.read(addr);
                let carry = if self.get_flag(FLAGS6502::C) { 0 } else { 1 };
                let result = (self.a as u16)
                    .wrapping_sub(value as u16)
                    .wrapping_sub(carry);
                self.set_flag(FLAGS6502::C, result < 0x100);
                self.set_flag(FLAGS6502::Z, (result & 0xFF) == 0);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
                self.set_flag(
                    FLAGS6502::V,
                    ((self.a ^ value) & 0x80) != 0 && ((self.a ^ (result as u8)) & 0x80) != 0,
                );
                self.a = (result & 0xFF) as u8;
                5
            }

            // AND - Logical AND
            0x29 => {
                // AND Immediate
                let value = self.bus.read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);
                self.a &= value;
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
                2
            }
            0x25 => {
                // AND Zero Page
                let addr = self.bus.read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);
                self.a &= self.bus.read(addr as u16);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
                3
            }
            0x35 => {
                // AND Zero Page,X
                let addr = self.bus.read(self.program_counter).wrapping_add(self.x);
                self.program_counter = self.program_counter.wrapping_add(1);
                self.a &= self.bus.read(addr as u16);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
                4
            }
            0x2D => {
                // AND Absolute
                let lo = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let hi = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let addr = lo | (hi << 8);
                self.a &= self.bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
                4
            }
            0x3D => {
                // AND Absolute,X
                let lo = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let hi = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let addr = (lo | (hi << 8)).wrapping_add(self.x as u16);
                self.a &= self.bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
                4
            }
            0x39 => {
                // AND Absolute,Y
                let lo = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let hi = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let addr = (lo | (hi << 8)).wrapping_add(self.y as u16);
                self.a &= self.bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
                4
            }
            0x21 => {
                // AND (Indirect,X)
                let zp = self.bus.read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);
                let lo = self.bus.read(zp.wrapping_add(self.x) as u16) as u16;
                let hi = self
                    .bus
                    .read(zp.wrapping_add(self.x).wrapping_add(1) as u16)
                    as u16;
                let addr = lo | (hi << 8);
                self.a &= self.bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
                6
            }
            0x31 => {
                // AND (Indirect),Y
                let zp = self.bus.read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);
                let lo = self.bus.read(zp as u16) as u16;
                let hi = self.bus.read(zp.wrapping_add(1) as u16) as u16;
                let addr = (lo | (hi << 8)).wrapping_add(self.y as u16);
                self.a &= self.bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
                5
            }

            // ORA - Logical OR
            0x09 => {
                // ORA Immediate
                let value = self.bus.read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);
                self.a |= value;
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
                2
            }
            0x05 => {
                // ORA Zero Page
                let addr = self.bus.read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);
                self.a |= self.bus.read(addr as u16);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
                3
            }
            0x15 => {
                // ORA Zero Page,X
                let addr = self.bus.read(self.program_counter).wrapping_add(self.x);
                self.program_counter = self.program_counter.wrapping_add(1);
                self.a |= self.bus.read(addr as u16);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
                4
            }
            0x0D => {
                // ORA Absolute
                let lo = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let hi = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let addr = lo | (hi << 8);
                self.a |= self.bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
                4
            }
            0x1D => {
                // ORA Absolute,X
                let lo = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let hi = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let addr = (lo | (hi << 8)).wrapping_add(self.x as u16);
                self.a |= self.bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
                4
            }
            0x19 => {
                // ORA Absolute,Y
                let lo = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let hi = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let addr = (lo | (hi << 8)).wrapping_add(self.y as u16);
                self.a |= self.bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
                4
            }
            0x01 => {
                // ORA (Indirect,X)
                let zp = self.bus.read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);
                let lo = self.bus.read(zp.wrapping_add(self.x) as u16) as u16;
                let hi = self
                    .bus
                    .read(zp.wrapping_add(self.x).wrapping_add(1) as u16)
                    as u16;
                let addr = lo | (hi << 8);
                self.a |= self.bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
                6
            }
            0x11 => {
                // ORA (Indirect),Y
                let zp = self.bus.read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);
                let lo = self.bus.read(zp as u16) as u16;
                let hi = self.bus.read(zp.wrapping_add(1) as u16) as u16;
                let addr = (lo | (hi << 8)).wrapping_add(self.y as u16);
                self.a |= self.bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
                5
            }

            // EOR - Logical XOR
            0x49 => {
                // EOR Immediate
                let value = self.bus.read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);
                self.a ^= value;
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
                2
            }
            0x45 => {
                // EOR Zero Page
                let addr = self.bus.read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);
                self.a ^= self.bus.read(addr as u16);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
                3
            }
            0x55 => {
                // EOR Zero Page,X
                let addr = self.bus.read(self.program_counter).wrapping_add(self.x);
                self.program_counter = self.program_counter.wrapping_add(1);
                self.a ^= self.bus.read(addr as u16);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
                4
            }
            0x4D => {
                // EOR Absolute
                let lo = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let hi = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let addr = lo | (hi << 8);
                self.a ^= self.bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
                4
            }
            0x5D => {
                // EOR Absolute,X
                let lo = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let hi = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let addr = (lo | (hi << 8)).wrapping_add(self.x as u16);
                self.a ^= self.bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
                4
            }
            0x59 => {
                // EOR Absolute,Y
                let lo = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let hi = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let addr = (lo | (hi << 8)).wrapping_add(self.y as u16);
                self.a ^= self.bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
                4
            }
            0x41 => {
                // EOR (Indirect,X)
                let zp = self.bus.read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);
                let lo = self.bus.read(zp.wrapping_add(self.x) as u16) as u16;
                let hi = self
                    .bus
                    .read(zp.wrapping_add(self.x).wrapping_add(1) as u16)
                    as u16;
                let addr = lo | (hi << 8);
                self.a ^= self.bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
                6
            }
            0x51 => {
                // EOR (Indirect),Y
                let zp = self.bus.read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);
                let lo = self.bus.read(zp as u16) as u16;
                let hi = self.bus.read(zp.wrapping_add(1) as u16) as u16;
                let addr = (lo | (hi << 8)).wrapping_add(self.y as u16);
                self.a ^= self.bus.read(addr);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
                5
            }

            // CMP - Compare Accumulator
            0xC9 => {
                // CMP Immediate
                let value = self.bus.read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);
                let result = self.a.wrapping_sub(value);
                self.set_flag(FLAGS6502::C, self.a >= value);
                self.set_flag(FLAGS6502::Z, self.a == value);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
                2
            }
            0xC5 => {
                // CMP Zero Page
                let addr = self.bus.read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);
                let value = self.bus.read(addr as u16);
                let result = self.a.wrapping_sub(value);
                self.set_flag(FLAGS6502::C, self.a >= value);
                self.set_flag(FLAGS6502::Z, self.a == value);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
                3
            }
            0xD5 => {
                // CMP Zero Page,X
                let addr = self.bus.read(self.program_counter).wrapping_add(self.x);
                self.program_counter = self.program_counter.wrapping_add(1);
                let value = self.bus.read(addr as u16);
                let result = self.a.wrapping_sub(value);
                self.set_flag(FLAGS6502::C, self.a >= value);
                self.set_flag(FLAGS6502::Z, self.a == value);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
                4
            }
            0xCD => {
                // CMP Absolute
                let lo = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let hi = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let addr = lo | (hi << 8);
                let value = self.bus.read(addr);
                let result = self.a.wrapping_sub(value);
                self.set_flag(FLAGS6502::C, self.a >= value);
                self.set_flag(FLAGS6502::Z, self.a == value);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
                4
            }
            0xDD => {
                // CMP Absolute,X
                let lo = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let hi = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let addr = (lo | (hi << 8)).wrapping_add(self.x as u16);
                let value = self.bus.read(addr);
                let result = self.a.wrapping_sub(value);
                self.set_flag(FLAGS6502::C, self.a >= value);
                self.set_flag(FLAGS6502::Z, self.a == value);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
                4
            }
            0xD9 => {
                // CMP Absolute,Y
                let lo = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let hi = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let addr = (lo | (hi << 8)).wrapping_add(self.y as u16);
                let value = self.bus.read(addr);
                let result = self.a.wrapping_sub(value);
                self.set_flag(FLAGS6502::C, self.a >= value);
                self.set_flag(FLAGS6502::Z, self.a == value);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
                4
            }
            0xC1 => {
                // CMP (Indirect,X)
                let zp = self.bus.read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);
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
                6
            }
            0xD1 => {
                // CMP (Indirect),Y
                let zp = self.bus.read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);
                let lo = self.bus.read(zp as u16) as u16;
                let hi = self.bus.read(zp.wrapping_add(1) as u16) as u16;
                let addr = (lo | (hi << 8)).wrapping_add(self.y as u16);
                let value = self.bus.read(addr);
                let result = self.a.wrapping_sub(value);
                self.set_flag(FLAGS6502::C, self.a >= value);
                self.set_flag(FLAGS6502::Z, self.a == value);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
                5
            }

            // CPX - Compare X Register
            0xE0 => {
                // CPX Immediate
                let value = self.bus.read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);
                let result = self.x.wrapping_sub(value);
                self.set_flag(FLAGS6502::C, self.x >= value);
                self.set_flag(FLAGS6502::Z, self.x == value);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
                2
            }
            0xE4 => {
                // CPX Zero Page
                let addr = self.bus.read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);
                let value = self.bus.read(addr as u16);
                let result = self.x.wrapping_sub(value);
                self.set_flag(FLAGS6502::C, self.x >= value);
                self.set_flag(FLAGS6502::Z, self.x == value);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
                3
            }
            0xEC => {
                // CPX Absolute
                let lo = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let hi = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let addr = lo | (hi << 8);
                let value = self.bus.read(addr);
                let result = self.x.wrapping_sub(value);
                self.set_flag(FLAGS6502::C, self.x >= value);
                self.set_flag(FLAGS6502::Z, self.x == value);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
                4
            }

            // BIT - Bit Test
            0x24 => {
                // BIT Zero Page
                let addr = self.bus.read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);
                let value = self.bus.read(addr as u16);
                let result = self.a & value;
                self.set_flag(FLAGS6502::Z, result == 0x00);
                self.set_flag(FLAGS6502::V, (value & 0x40) != 0);
                self.set_flag(FLAGS6502::N, (value & 0x80) != 0);
                3
            }
            0x2C => {
                // BIT Absolute
                let lo = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let hi = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let addr = lo | (hi << 8);
                let value = self.bus.read(addr);
                let result = self.a & value;
                self.set_flag(FLAGS6502::Z, result == 0x00);
                self.set_flag(FLAGS6502::V, (value & 0x40) != 0);
                self.set_flag(FLAGS6502::N, (value & 0x80) != 0);
                4
            }

            // CPY - Compare Y Register
            0xC0 => {
                // CPY Immediate
                let value = self.bus.read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);
                let result = self.y.wrapping_sub(value);
                self.set_flag(FLAGS6502::C, self.y >= value);
                self.set_flag(FLAGS6502::Z, self.y == value);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
                2
            }
            0xC4 => {
                // CPY Zero Page
                let addr = self.bus.read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);
                let value = self.bus.read(addr as u16);
                let result = self.y.wrapping_sub(value);
                self.set_flag(FLAGS6502::C, self.y >= value);
                self.set_flag(FLAGS6502::Z, self.y == value);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
                3
            }
            0xCC => {
                // CPY Absolute
                let lo = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let hi = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let addr = lo | (hi << 8);
                let value = self.bus.read(addr);
                let result = self.y.wrapping_sub(value);
                self.set_flag(FLAGS6502::C, self.y >= value);
                self.set_flag(FLAGS6502::Z, self.y == value);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
                4
            }

            // JMP - Jump
            0x4C => {
                // JMP Absolute
                let lo = self.bus.read(self.program_counter) as u16;
                let hi = self.bus.read(self.program_counter.wrapping_add(1)) as u16;
                self.program_counter = lo | (hi << 8);
                3
            }
            0x6C => {
                // JMP Indirect (with 6502 bug)
                let lo = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let hi = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let addr = lo | (hi << 8);
                // 6502 bug: if addr is at page boundary (e.g., 0x12FF),
                // it wraps around to 0x1200 instead of 0x1300
                if (addr & 0xFF) == 0xFF {
                    let lo = self.bus.read(addr) as u16;
                    let hi = self.bus.read(addr & 0xFF00) as u16;
                    self.program_counter = lo | (hi << 8);
                } else {
                    let lo = self.bus.read(addr) as u16;
                    let hi = self.bus.read(addr.wrapping_add(1)) as u16;
                    self.program_counter = lo | (hi << 8);
                }
                5
            }

            // JSR - Jump to Subroutine
            0x20 => {
                let lo = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let hi = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let addr = lo | (hi << 8);
                let ret_addr = self.program_counter.wrapping_sub(1);
                self.bus
                    .write(0x100 + self.stack_pointer as u16, (ret_addr >> 8) as u8);
                self.stack_pointer = self.stack_pointer.wrapping_sub(1);
                self.bus
                    .write(0x100 + self.stack_pointer as u16, (ret_addr & 0xFF) as u8);
                self.stack_pointer = self.stack_pointer.wrapping_sub(1);
                self.program_counter = addr;
                6
            }

            // RTS - Return from Subroutine
            0x60 => {
                self.stack_pointer = self.stack_pointer.wrapping_add(1);
                let lo = self.bus.read(0x100 + self.stack_pointer as u16) as u16;
                self.stack_pointer = self.stack_pointer.wrapping_add(1);
                let hi = self.bus.read(0x100 + self.stack_pointer as u16) as u16;
                self.program_counter = (lo | (hi << 8)).wrapping_add(1);
                6
            }

            // RTI - Return from Interrupt
            0x40 => {
                self.stack_pointer = self.stack_pointer.wrapping_add(1);
                self.p = self.bus.read(0x100 + self.stack_pointer as u16) & !0x10;
                self.stack_pointer = self.stack_pointer.wrapping_add(1);
                let lo = self.bus.read(0x100 + self.stack_pointer as u16) as u16;
                self.stack_pointer = self.stack_pointer.wrapping_add(1);
                let hi = self.bus.read(0x100 + self.stack_pointer as u16) as u16;
                self.program_counter = lo | (hi << 8);
                6
            }

            // Branch Instructions
            0xF0 => {
                // BEQ - Branch if Equal
                let offset = self.bus.read(self.program_counter) as i8;
                self.program_counter = self.program_counter.wrapping_add(1);
                if self.get_flag(FLAGS6502::Z) {
                    self.program_counter = self.program_counter.wrapping_add(offset as u16);
                    3
                } else {
                    2
                }
            }
            0xD0 => {
                // BNE - Branch if Not Equal
                let offset = self.bus.read(self.program_counter) as i8;
                self.program_counter = self.program_counter.wrapping_add(1);
                if !self.get_flag(FLAGS6502::Z) {
                    self.program_counter = self.program_counter.wrapping_add(offset as u16);
                    3
                } else {
                    2
                }
            }
            0xB0 => {
                // BCS - Branch if Carry Set
                let offset = self.bus.read(self.program_counter) as i8;
                self.program_counter = self.program_counter.wrapping_add(1);
                if self.get_flag(FLAGS6502::C) {
                    self.program_counter = self.program_counter.wrapping_add(offset as u16);
                    3
                } else {
                    2
                }
            }
            0x90 => {
                // BCC - Branch if Carry Clear
                let offset = self.bus.read(self.program_counter) as i8;
                self.program_counter = self.program_counter.wrapping_add(1);
                if !self.get_flag(FLAGS6502::C) {
                    self.program_counter = self.program_counter.wrapping_add(offset as u16);
                    3
                } else {
                    2
                }
            }
            0x30 => {
                // BMI - Branch if Minus
                let offset = self.bus.read(self.program_counter) as i8;
                self.program_counter = self.program_counter.wrapping_add(1);
                if self.get_flag(FLAGS6502::N) {
                    self.program_counter = self.program_counter.wrapping_add(offset as u16);
                    3
                } else {
                    2
                }
            }
            0x10 => {
                // BPL - Branch if Positive
                let offset = self.bus.read(self.program_counter) as i8;
                self.program_counter = self.program_counter.wrapping_add(1);
                if !self.get_flag(FLAGS6502::N) {
                    self.program_counter = self.program_counter.wrapping_add(offset as u16);
                    3
                } else {
                    2
                }
            }
            0x70 => {
                // BVS - Branch if Overflow Set
                let offset = self.bus.read(self.program_counter) as i8;
                self.program_counter = self.program_counter.wrapping_add(1);
                if self.get_flag(FLAGS6502::V) {
                    self.program_counter = self.program_counter.wrapping_add(offset as u16);
                    3
                } else {
                    2
                }
            }
            0x50 => {
                // BVC - Branch if Overflow Clear
                let offset = self.bus.read(self.program_counter) as i8;
                self.program_counter = self.program_counter.wrapping_add(1);
                if !self.get_flag(FLAGS6502::V) {
                    self.program_counter = self.program_counter.wrapping_add(offset as u16);
                    3
                } else {
                    2
                }
            }

            // INC - Increment Memory
            0xE6 => {
                // INC Zero Page
                let addr = self.bus.read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);
                let value = self.bus.read(addr as u16).wrapping_add(1);
                self.bus.write(addr as u16, value);
                self.set_flag(FLAGS6502::Z, value == 0x00);
                self.set_flag(FLAGS6502::N, (value & 0x80) != 0);
                5
            }
            0xF6 => {
                // INC Zero Page,X
                let addr = self.bus.read(self.program_counter).wrapping_add(self.x);
                self.program_counter = self.program_counter.wrapping_add(1);
                let value = self.bus.read(addr as u16).wrapping_add(1);
                self.bus.write(addr as u16, value);
                self.set_flag(FLAGS6502::Z, value == 0x00);
                self.set_flag(FLAGS6502::N, (value & 0x80) != 0);
                6
            }
            0xEE => {
                // INC Absolute
                let lo = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let hi = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let addr = lo | (hi << 8);
                let value = self.bus.read(addr).wrapping_add(1);
                self.bus.write(addr, value);
                self.set_flag(FLAGS6502::Z, value == 0x00);
                self.set_flag(FLAGS6502::N, (value & 0x80) != 0);
                6
            }
            0xFE => {
                // INC Absolute,X
                let lo = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let hi = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let addr = (lo | (hi << 8)).wrapping_add(self.x as u16);
                let value = self.bus.read(addr).wrapping_add(1);
                self.bus.write(addr, value);
                self.set_flag(FLAGS6502::Z, value == 0x00);
                self.set_flag(FLAGS6502::N, (value & 0x80) != 0);
                7
            }

            // INX/INY - Increment X/Y
            0xE8 => {
                // INX
                self.x = self.x.wrapping_add(1);
                self.set_flag(FLAGS6502::Z, self.x == 0x00);
                self.set_flag(FLAGS6502::N, (self.x & 0x80) != 0);
                2
            }
            0xC8 => {
                // INY
                self.y = self.y.wrapping_add(1);
                self.set_flag(FLAGS6502::Z, self.y == 0x00);
                self.set_flag(FLAGS6502::N, (self.y & 0x80) != 0);
                2
            }

            // DEC - Decrement Memory
            0xC6 => {
                // DEC Zero Page
                let addr = self.bus.read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);
                let value = self.bus.read(addr as u16).wrapping_sub(1);
                self.bus.write(addr as u16, value);
                self.set_flag(FLAGS6502::Z, value == 0x00);
                self.set_flag(FLAGS6502::N, (value & 0x80) != 0);
                5
            }
            0xD6 => {
                // DEC Zero Page,X
                let addr = self.bus.read(self.program_counter).wrapping_add(self.x);
                self.program_counter = self.program_counter.wrapping_add(1);
                let value = self.bus.read(addr as u16).wrapping_sub(1);
                self.bus.write(addr as u16, value);
                self.set_flag(FLAGS6502::Z, value == 0x00);
                self.set_flag(FLAGS6502::N, (value & 0x80) != 0);
                6
            }
            0xCE => {
                // DEC Absolute
                let lo = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let hi = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let addr = lo | (hi << 8);
                let value = self.bus.read(addr).wrapping_sub(1);
                self.bus.write(addr, value);
                self.set_flag(FLAGS6502::Z, value == 0x00);
                self.set_flag(FLAGS6502::N, (value & 0x80) != 0);
                6
            }
            0xDE => {
                // DEC Absolute,X
                let lo = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let hi = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let addr = (lo | (hi << 8)).wrapping_add(self.x as u16);
                let value = self.bus.read(addr).wrapping_sub(1);
                self.bus.write(addr, value);
                self.set_flag(FLAGS6502::Z, value == 0x00);
                self.set_flag(FLAGS6502::N, (value & 0x80) != 0);
                7
            }

            // DEX/DEY - Decrement X/Y
            0xCA => {
                // DEX
                self.x = self.x.wrapping_sub(1);
                self.set_flag(FLAGS6502::Z, self.x == 0x00);
                self.set_flag(FLAGS6502::N, (self.x & 0x80) != 0);
                2
            }
            0x88 => {
                // DEY
                self.y = self.y.wrapping_sub(1);
                self.set_flag(FLAGS6502::Z, self.y == 0x00);
                self.set_flag(FLAGS6502::N, (self.y & 0x80) != 0);
                2
            }

            // Stack Operations
            0x48 => {
                // PHA - Push Accumulator
                self.bus.write(0x100 + self.stack_pointer as u16, self.a);
                self.stack_pointer = self.stack_pointer.wrapping_sub(1);
                3
            }
            0x68 => {
                // PLA - Pull Accumulator
                self.stack_pointer = self.stack_pointer.wrapping_add(1);
                self.a = self.bus.read(0x100 + self.stack_pointer as u16);
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
                4
            }
            0x08 => {
                // PHP - Push Processor Status
                self.bus
                    .write(0x100 + self.stack_pointer as u16, self.p | 0x10);
                self.stack_pointer = self.stack_pointer.wrapping_sub(1);
                3
            }
            0x28 => {
                // PLP - Pull Processor Status
                self.stack_pointer = self.stack_pointer.wrapping_add(1);
                self.p = self.bus.read(0x100 + self.stack_pointer as u16) & !0x10;
                4
            }

            // Transfer Instructions
            0xAA => {
                // TAX - Transfer A to X
                self.x = self.a;
                self.set_flag(FLAGS6502::Z, self.x == 0x00);
                self.set_flag(FLAGS6502::N, (self.x & 0x80) != 0);
                2
            }
            0xA8 => {
                // TAY - Transfer A to Y
                self.y = self.a;
                self.set_flag(FLAGS6502::Z, self.y == 0x00);
                self.set_flag(FLAGS6502::N, (self.y & 0x80) != 0);
                2
            }
            0x8A => {
                // TXA - Transfer X to A
                self.a = self.x;
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
                2
            }
            0x98 => {
                // TYA - Transfer Y to A
                self.a = self.y;
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
                2
            }
            0xBA => {
                // TSX - Transfer Stack Pointer to X
                self.x = self.stack_pointer;
                self.set_flag(FLAGS6502::Z, self.x == 0x00);
                self.set_flag(FLAGS6502::N, (self.x & 0x80) != 0);
                2
            }
            0x9A => {
                // TXS - Transfer X to Stack Pointer
                self.stack_pointer = self.x;
                2
            }

            // ASL - Arithmetic Shift Left
            0x0A => {
                // ASL Accumulator
                self.set_flag(FLAGS6502::C, (self.a & 0x80) != 0);
                self.a = self.a << 1;
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
                2
            }
            0x06 => {
                // ASL Zero Page
                let addr = self.bus.read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);
                let value = self.bus.read(addr as u16);
                self.set_flag(FLAGS6502::C, (value & 0x80) != 0);
                let result = value << 1;
                self.bus.write(addr as u16, result);
                self.set_flag(FLAGS6502::Z, result == 0x00);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
                5
            }
            0x16 => {
                // ASL Zero Page,X
                let addr = self.bus.read(self.program_counter).wrapping_add(self.x);
                self.program_counter = self.program_counter.wrapping_add(1);
                let value = self.bus.read(addr as u16);
                self.set_flag(FLAGS6502::C, (value & 0x80) != 0);
                let result = value << 1;
                self.bus.write(addr as u16, result);
                self.set_flag(FLAGS6502::Z, result == 0x00);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
                6
            }
            0x0E => {
                // ASL Absolute
                let lo = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let hi = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let addr = lo | (hi << 8);
                let value = self.bus.read(addr);
                self.set_flag(FLAGS6502::C, (value & 0x80) != 0);
                let result = value << 1;
                self.bus.write(addr, result);
                self.set_flag(FLAGS6502::Z, result == 0x00);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
                6
            }
            0x1E => {
                // ASL Absolute,X
                let lo = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let hi = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let addr = (lo | (hi << 8)).wrapping_add(self.x as u16);
                let value = self.bus.read(addr);
                self.set_flag(FLAGS6502::C, (value & 0x80) != 0);
                let result = value << 1;
                self.bus.write(addr, result);
                self.set_flag(FLAGS6502::Z, result == 0x00);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
                7
            }

            // LSR - Logical Shift Right
            0x4A => {
                // LSR Accumulator
                self.set_flag(FLAGS6502::C, (self.a & 0x01) != 0);
                self.a = self.a >> 1;
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, false);
                2
            }
            0x46 => {
                // LSR Zero Page
                let addr = self.bus.read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);
                let value = self.bus.read(addr as u16);
                self.set_flag(FLAGS6502::C, (value & 0x01) != 0);
                let result = value >> 1;
                self.bus.write(addr as u16, result);
                self.set_flag(FLAGS6502::Z, result == 0x00);
                self.set_flag(FLAGS6502::N, false);
                5
            }
            0x56 => {
                // LSR Zero Page,X
                let addr = self.bus.read(self.program_counter).wrapping_add(self.x);
                self.program_counter = self.program_counter.wrapping_add(1);
                let value = self.bus.read(addr as u16);
                self.set_flag(FLAGS6502::C, (value & 0x01) != 0);
                let result = value >> 1;
                self.bus.write(addr as u16, result);
                self.set_flag(FLAGS6502::Z, result == 0x00);
                self.set_flag(FLAGS6502::N, false);
                6
            }
            0x4E => {
                // LSR Absolute
                let lo = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let hi = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let addr = lo | (hi << 8);
                let value = self.bus.read(addr);
                self.set_flag(FLAGS6502::C, (value & 0x01) != 0);
                let result = value >> 1;
                self.bus.write(addr, result);
                self.set_flag(FLAGS6502::Z, result == 0x00);
                self.set_flag(FLAGS6502::N, false);
                6
            }
            0x5E => {
                // LSR Absolute,X
                let lo = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let hi = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let addr = (lo | (hi << 8)).wrapping_add(self.x as u16);
                let value = self.bus.read(addr);
                self.set_flag(FLAGS6502::C, (value & 0x01) != 0);
                let result = value >> 1;
                self.bus.write(addr, result);
                self.set_flag(FLAGS6502::Z, result == 0x00);
                self.set_flag(FLAGS6502::N, false);
                7
            }

            // ROL - Rotate Left
            0x2A => {
                // ROL Accumulator
                let old_carry = if self.get_flag(FLAGS6502::C) { 1 } else { 0 };
                self.set_flag(FLAGS6502::C, (self.a & 0x80) != 0);
                self.a = (self.a << 1) | old_carry;
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
                2
            }
            0x26 => {
                // ROL Zero Page
                let addr = self.bus.read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);
                let value = self.bus.read(addr as u16);
                let old_carry = if self.get_flag(FLAGS6502::C) { 1 } else { 0 };
                self.set_flag(FLAGS6502::C, (value & 0x80) != 0);
                let result = (value << 1) | old_carry;
                self.bus.write(addr as u16, result);
                self.set_flag(FLAGS6502::Z, result == 0x00);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
                5
            }
            0x36 => {
                // ROL Zero Page,X
                let addr = self.bus.read(self.program_counter).wrapping_add(self.x);
                self.program_counter = self.program_counter.wrapping_add(1);
                let value = self.bus.read(addr as u16);
                let old_carry = if self.get_flag(FLAGS6502::C) { 1 } else { 0 };
                self.set_flag(FLAGS6502::C, (value & 0x80) != 0);
                let result = (value << 1) | old_carry;
                self.bus.write(addr as u16, result);
                self.set_flag(FLAGS6502::Z, result == 0x00);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
                6
            }
            0x2E => {
                // ROL Absolute
                let lo = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let hi = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let addr = lo | (hi << 8);
                let value = self.bus.read(addr);
                let old_carry = if self.get_flag(FLAGS6502::C) { 1 } else { 0 };
                self.set_flag(FLAGS6502::C, (value & 0x80) != 0);
                let result = (value << 1) | old_carry;
                self.bus.write(addr, result);
                self.set_flag(FLAGS6502::Z, result == 0x00);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
                6
            }
            0x3E => {
                // ROL Absolute,X
                let lo = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let hi = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let addr = (lo | (hi << 8)).wrapping_add(self.x as u16);
                let value = self.bus.read(addr);
                let old_carry = if self.get_flag(FLAGS6502::C) { 1 } else { 0 };
                self.set_flag(FLAGS6502::C, (value & 0x80) != 0);
                let result = (value << 1) | old_carry;
                self.bus.write(addr, result);
                self.set_flag(FLAGS6502::Z, result == 0x00);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
                7
            }

            // ROR - Rotate Right
            0x6A => {
                // ROR Accumulator
                let old_carry = if self.get_flag(FLAGS6502::C) { 0x80 } else { 0 };
                self.set_flag(FLAGS6502::C, (self.a & 0x01) != 0);
                self.a = (self.a >> 1) | old_carry;
                self.set_flag(FLAGS6502::Z, self.a == 0x00);
                self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
                2
            }
            0x66 => {
                // ROR Zero Page
                let addr = self.bus.read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);
                let value = self.bus.read(addr as u16);
                let old_carry = if self.get_flag(FLAGS6502::C) { 0x80 } else { 0 };
                self.set_flag(FLAGS6502::C, (value & 0x01) != 0);
                let result = (value >> 1) | old_carry;
                self.bus.write(addr as u16, result);
                self.set_flag(FLAGS6502::Z, result == 0x00);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
                5
            }
            0x76 => {
                // ROR Zero Page,X
                let addr = self.bus.read(self.program_counter).wrapping_add(self.x);
                self.program_counter = self.program_counter.wrapping_add(1);
                let value = self.bus.read(addr as u16);
                let old_carry = if self.get_flag(FLAGS6502::C) { 0x80 } else { 0 };
                self.set_flag(FLAGS6502::C, (value & 0x01) != 0);
                let result = (value >> 1) | old_carry;
                self.bus.write(addr as u16, result);
                self.set_flag(FLAGS6502::Z, result == 0x00);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
                6
            }
            0x6E => {
                // ROR Absolute
                let lo = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let hi = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let addr = lo | (hi << 8);
                let value = self.bus.read(addr);
                let old_carry = if self.get_flag(FLAGS6502::C) { 0x80 } else { 0 };
                self.set_flag(FLAGS6502::C, (value & 0x01) != 0);
                let result = (value >> 1) | old_carry;
                self.bus.write(addr, result);
                self.set_flag(FLAGS6502::Z, result == 0x00);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
                6
            }
            0x7E => {
                // ROR Absolute,X
                let lo = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let hi = self.bus.read(self.program_counter) as u16;
                self.program_counter = self.program_counter.wrapping_add(1);
                let addr = (lo | (hi << 8)).wrapping_add(self.x as u16);
                let value = self.bus.read(addr);
                let old_carry = if self.get_flag(FLAGS6502::C) { 0x80 } else { 0 };
                self.set_flag(FLAGS6502::C, (value & 0x01) != 0);
                let result = (value >> 1) | old_carry;
                self.bus.write(addr, result);
                self.set_flag(FLAGS6502::Z, result == 0x00);
                self.set_flag(FLAGS6502::N, (result & 0x80) != 0);
                7
            }

            // NOP - No Operation
            0xEA => {
                // NOP Implied (official)
                // Does nothing, just takes up time
                2
            }
            0x89 => {
                // NOP Immediate (unofficial)
                self.program_counter = self.program_counter.wrapping_add(1); // Skip the immediate byte
                2
            }

            // Flag Instructions
            0x18 => {
                // CLC - Clear Carry Flag
                self.set_flag(FLAGS6502::C, false);
                2
            }
            0x38 => {
                // SEC - Set Carry Flag
                self.set_flag(FLAGS6502::C, true);
                2
            }
            0x58 => {
                // CLI - Clear Interrupt Disable
                self.set_flag(FLAGS6502::I, false);
                2
            }
            0x78 => {
                // SEI - Set Interrupt Disable
                self.set_flag(FLAGS6502::I, true);
                2
            }
            0xB8 => {
                // CLV - Clear Overflow Flag
                self.set_flag(FLAGS6502::V, false);
                2
            }
            0xD8 => {
                // CLD - Clear Decimal Mode
                self.set_flag(FLAGS6502::D, false);
                2
            }
            0xF8 => {
                // SED - Set Decimal Mode
                self.set_flag(FLAGS6502::D, true);
                2
            }

            _ => {
                eprintln!("Unknown opcode: 0x{:02X}", opcode);
                2
            }
        }
    }
}
