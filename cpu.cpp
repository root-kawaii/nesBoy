#include "cpu.h"
#include "bus.h"
#include <iostream>

void CPU6502::setFlag(FLAGS6502 flag, bool value) {
    if (value)
        P |= flag;
    else
        P &= ~flag;
}

bool CPU6502::getFlag(FLAGS6502 flag) {
    return (P & flag) > 0;
}

void CPU6502::reset() {
    A = X = Y = 0;
    SP = 0xFD;
    P = 0x24;
    PC = 0x8000; // Set to where your program starts
}

void CPU6502::step() {
    uint8_t opcode = read(PC++);
    execute(opcode);
    if (bus)
        bus->ppu->step(); // Let PPU run in syncc
}

uint8_t CPU6502::read(uint16_t addr) {
    return bus->read(addr);
}

void CPU6502::write(uint16_t addr, uint8_t data) {
    bus->write(addr, data);
}

void CPU6502::ConnectBus(Bus* n) {
    this->bus = n;
}

void CPU6502::execute(uint8_t opcode) {
    switch (opcode) {
        case 0xA9: { // LDA Immediate
            uint8_t value = read(PC++);
            A = value;
            setFlag(Z, A == 0x00);
            setFlag(N, A & 0x80);
            break;
        }

        case 0xA5: { // LDA Zero Page
            uint8_t addr = read(PC++);
            A = read(addr);
            setFlag(Z, A == 0x00);
            setFlag(N, A & 0x80);
            break;
        }

                        // LDA Absolute
        case 0xAD: {
            uint16_t addr = read(PC++) | (read(PC++) << 8);
            A = read(addr);
            setFlag(Z, A == 0x00);
            setFlag(N, A & 0x80);
            break;
        }

        // LDA Absolute,X
        case 0xBD: {
            uint16_t addr = (read(PC++) | (read(PC++) << 8)) + X;
            A = read(addr);
            setFlag(Z, A == 0x00);
            setFlag(N, A & 0x80);
            break;
        }

        // LDA Absolute,Y
        case 0xB9: {
            uint16_t addr = (read(PC++) | (read(PC++) << 8)) + Y;
            A = read(addr);
            setFlag(Z, A == 0x00);
            setFlag(N, A & 0x80);
            break;
        }

        // LDA (Indirect,X)
        case 0xA1: {
            uint8_t zp = read(PC++);
            uint16_t addr = (read((zp + X) & 0xFF) | (read((zp + X + 1) & 0xFF) << 8));
            A = read(addr);
            setFlag(Z, A == 0x00);
            setFlag(N, A & 0x80);
            break;
        }

        // LDA (Indirect),Y
        case 0xB1: {
            uint8_t zp = read(PC++);
            uint16_t addr = (read(zp) | (read((zp + 1) & 0xFF) << 8)) + Y;
            A = read(addr);
            setFlag(Z, A == 0x00);
            setFlag(N, A & 0x80);
            break;
        }

        // LDA Zero Page,X
        case 0xB5: {
            uint8_t addr = (read(PC++) + X) & 0xFF;
            A = read(addr);
            setFlag(Z, A == 0x00);
            setFlag(N, A & 0x80);
            break;
        }

        // LDX Zero Page,Y
        case 0xB6: {
            uint8_t addr = (read(PC++) + Y) & 0xFF;
            X = read(addr);
            setFlag(Z, X == 0x00);
            setFlag(N, X & 0x80);
            break;
        }

        // LDX Absolute
        case 0xAE: {
            uint16_t addr = read(PC++) | (read(PC++) << 8);
            X = read(addr);
            setFlag(Z, X == 0x00);
            setFlag(N, X & 0x80);
            break;
        }

        // LDX Absolute,Y
        case 0xBE: {
            uint16_t addr = (read(PC++) | (read(PC++) << 8)) + Y;
            X = read(addr);
            setFlag(Z, X == 0x00);
            setFlag(N, X & 0x80);
            break;
        }

        // LDY Absolute
        case 0xAC: {
            uint16_t addr = read(PC++) | (read(PC++) << 8);
            Y = read(addr);
            setFlag(Z, Y == 0x00);
            setFlag(N, Y & 0x80);
            break;
        }

        // LDY Absolute,X
        case 0xBC: {
            uint16_t addr = (read(PC++) | (read(PC++) << 8)) + X;
            Y = read(addr);
            setFlag(Z, Y == 0x00);
            setFlag(N, Y & 0x80);
            break;
        }

        // LDY Zero Page,X
        case 0xB4: {
            uint8_t addr = (read(PC++) + X) & 0xFF;
            Y = read(addr);
            setFlag(Z, Y == 0x00);
            setFlag(N, Y & 0x80);
            break;
        }

        case 0xA2: { // LDX Immediate
            X = read(PC++);
            setFlag(Z, X == 0x00);
            setFlag(N, X & 0x80);
            break;
        }

        case 0xA6: { // LDX Zero Page
            uint8_t addr = read(PC++);
            X = read(addr);
            setFlag(Z, X == 0x00);
            setFlag(N, X & 0x80);
            break;
        }

        case 0xA0: { // LDY Immediate
            Y = read(PC++);
            setFlag(Z, Y == 0x00);
            setFlag(N, Y & 0x80);
            break;
        }

        case 0xA4: { // LDY Zero Page
            uint8_t addr = read(PC++);
            Y = read(addr);
            setFlag(Z, Y == 0x00);
            setFlag(N, Y & 0x80);
            break;
        }

        case 0x85: { // STA Zero Page
            uint8_t addr = read(PC++);
            write(addr, A);
            break;
        }

        case 0x86: { // STX Zero Page
            uint8_t addr = read(PC++);
            write(addr, X);
            break;
        }

        case 0x84: { // STY Zero Page
            uint8_t addr = read(PC++);
            write(addr, Y);
            break;
        }

                // STA Absolute
        case 0x8D: {
            uint16_t addr = read(PC++) | (read(PC++) << 8);
            write(addr, A);
            break;
        }

        // STA Zero Page,X
        case 0x95: {
            uint8_t addr = (read(PC++) + X) & 0xFF;
            write(addr, A);
            break;
        }

        // STA Absolute,X
        case 0x9D: {
            uint16_t addr = (read(PC++) | (read(PC++) << 8)) + X;
            write(addr, A);
            break;
        }

        // STA Absolute,Y
        case 0x99: {
            uint16_t addr = (read(PC++) | (read(PC++) << 8)) + Y;
            write(addr, A);
            break;
        }

        // STA (Indirect,X)
        case 0x81: {
            uint8_t zp = read(PC++);
            uint16_t addr = (read((zp + X) & 0xFF) | (read((zp + X + 1) & 0xFF) << 8));
            write(addr, A);
            break;
        }

        // STA (Indirect),Y
        case 0x91: {
            uint8_t zp = read(PC++);
            uint16_t addr = (read(zp) | (read((zp + 1) & 0xFF) << 8)) + Y;
            write(addr, A);
            break;
        }

        // STX Zero Page,Y
        case 0x96: {
            uint8_t addr = (read(PC++) + Y) & 0xFF;
            write(addr, X);
            break;
        }

        // STX Absolute
        case 0x8E: {
            uint16_t addr = read(PC++) | (read(PC++) << 8);
            write(addr, X);
            break;
        }

        // STY Zero Page,X
        case 0x94: {
            uint8_t addr = (read(PC++) + X) & 0xFF;
            write(addr, Y);
            break;
        }

        // STY Absolute
        case 0x8C: {
            uint16_t addr = read(PC++) | (read(PC++) << 8);
            write(addr, Y);
            break;
        }

            // ADC Zero Page
        case 0x65: {
        uint8_t addr = read(PC++);
        uint8_t value = read(addr);
        uint16_t result = A + value + (getFlag(C) ? 1 : 0);
        setFlag(C, result > 0xFF);
        setFlag(Z, (result & 0xFF) == 0x00);
        setFlag(N, result & 0x80);
        setFlag(V, ((A ^ value) & (A ^ (result & 0xFF)) & 0x80) != 0);
        A = result & 0xFF;
        break;
        }

        // ADC Absolute
        case 0x6D: {
        uint16_t addr = read(PC++) | (read(PC++) << 8);
        uint8_t value = read(addr);
        uint16_t result = A + value + (getFlag(C) ? 1 : 0);
        setFlag(C, result > 0xFF);
        setFlag(Z, (result & 0xFF) == 0x00);
        setFlag(N, result & 0x80);
        setFlag(V, ((A ^ value) & (A ^ (result & 0xFF)) & 0x80) != 0);
        A = result & 0xFF;
        break;
        }

        // JMP Absolute
        case 0x4C: {
        PC = read(PC) | (read(PC + 1) << 8);
        break;
        }

        // JMP Indirect
        case 0x6C: {
        uint16_t addr = read(PC++) | (read(PC++) << 8);
        // Note: 6502 bug - if address is at page boundary
        if ((addr & 0xFF) == 0xFF) {
            PC = read(addr) | (read(addr & 0xFF00) << 8);
        } else {
            PC = read(addr) | (read(addr + 1) << 8);
        }
        break;
        }

        // JSR Absolute
        case 0x20: {
        uint16_t addr = read(PC++) | (read(PC++) << 8);
        write(0x100 + SP--, (PC - 1) >> 8);
        write(0x100 + SP--, (PC - 1) & 0xFF);
        PC = addr;
        break;
        }

        // RTS
        case 0x60: {
        uint8_t lo = read(0x100 + ++SP);
        uint8_t hi = read(0x100 + ++SP);
        PC = ((hi << 8) | lo) + 1;
         break;
        }

                // CMP Immediate
        case 0xC9: {
            uint8_t value = read(PC++);
            uint8_t result = A - value;
            setFlag(C, A >= value);
            setFlag(Z, A == value);
            setFlag(N, result & 0x80);
            break;
        }

        // CMP Zero Page
        case 0xC5: {
            uint8_t addr = read(PC++);
            uint8_t value = read(addr);
            uint8_t result = A - value;
            setFlag(C, A >= value);
            setFlag(Z, A == value);
            setFlag(N, result & 0x80);
            break;
        }

        // CMP Zero Page,X
        case 0xD5: {
            uint8_t addr = (read(PC++) + X) & 0xFF;
            uint8_t value = read(addr);
            uint8_t result = A - value;
            setFlag(C, A >= value);
            setFlag(Z, A == value);
            setFlag(N, result & 0x80);
            break;
        }

        // CMP Absolute
        case 0xCD: {
            uint16_t addr = read(PC++) | (read(PC++) << 8);
            uint8_t value = read(addr);
            uint8_t result = A - value;
            setFlag(C, A >= value);
            setFlag(Z, A == value);
            setFlag(N, result & 0x80);
            break;
        }

        // CMP Absolute,X
        case 0xDD: {
            uint16_t addr = (read(PC++) | (read(PC++) << 8)) + X;
            uint8_t value = read(addr);
            uint8_t result = A - value;
            setFlag(C, A >= value);
            setFlag(Z, A == value);
            setFlag(N, result & 0x80);
            break;
        }

        // CMP Absolute,Y
        case 0xD9: {
            uint16_t addr = (read(PC++) | (read(PC++) << 8)) + Y;
            uint8_t value = read(addr);
            uint8_t result = A - value;
            setFlag(C, A >= value);
            setFlag(Z, A == value);
            setFlag(N, result & 0x80);
            break;
        }

        // CMP (Indirect,X)
        case 0xC1: {
            uint8_t zp = read(PC++);
            uint16_t addr = (read((zp + X) & 0xFF) | (read((zp + X + 1) & 0xFF) << 8));
            uint8_t value = read(addr);
            uint8_t result = A - value;
            setFlag(C, A >= value);
            setFlag(Z, A == value);
            setFlag(N, result & 0x80);
            break;
        }

        // CMP (Indirect),Y
        case 0xD1: {
            uint8_t zp = read(PC++);
            uint16_t addr = (read(zp) | (read((zp + 1) & 0xFF) << 8)) + Y;
            uint8_t value = read(addr);
            uint8_t result = A - value;
            setFlag(C, A >= value);
            setFlag(Z, A == value);
            setFlag(N, result & 0x80);
            break;
        }

        // CPX Immediate
        case 0xE0: {
            uint8_t value = read(PC++);
            uint8_t result = X - value;
            setFlag(C, X >= value);
            setFlag(Z, X == value);
            setFlag(N, result & 0x80);
            break;
        }

        // CPX Zero Page
        case 0xE4: {
            uint8_t addr = read(PC++);
            uint8_t value = read(addr);
            uint8_t result = X - value;
            setFlag(C, X >= value);
            setFlag(Z, X == value);
            setFlag(N, result & 0x80);
            break;
        }

        // CPX Absolute
        case 0xEC: {
            uint16_t addr = read(PC++) | (read(PC++) << 8);
            uint8_t value = read(addr);
            uint8_t result = X - value;
            setFlag(C, X >= value);
            setFlag(Z, X == value);
            setFlag(N, result & 0x80);
            break;
        }

        // CPY Immediate
        case 0xC0: {
            uint8_t value = read(PC++);
            uint8_t result = Y - value;
            setFlag(C, Y >= value);
            setFlag(Z, Y == value);
            setFlag(N, result & 0x80);
            break;
        }

        // CPY Zero Page
        case 0xC4: {
            uint8_t addr = read(PC++);
            uint8_t value = read(addr);
            uint8_t result = Y - value;
            setFlag(C, Y >= value);
            setFlag(Z, Y == value);
            setFlag(N, result & 0x80);
            break;
        }

        // CPY Absolute
        case 0xCC: {
            uint16_t addr = read(PC++) | (read(PC++) << 8);
            uint8_t value = read(addr);
            uint8_t result = Y - value;
            setFlag(C, Y >= value);
            setFlag(Z, Y == value);
            setFlag(N, result & 0x80);
            break;
        }

        case 0x69: { // ADC Immediate
            uint8_t value = read(PC++);
            uint16_t result = A + value + (getFlag(C) ? 1 : 0);
            setFlag(C, result > 0xFF);
            setFlag(Z, (result & 0xFF) == 0x00);
            setFlag(N, result & 0x80);
            setFlag(V, ((A ^ value) & (A ^ (result & 0xFF)) & 0x80) != 0);
            A = result & 0xFF;
            break;
        }

                // AND Immediate
        case 0x29: {
            uint8_t value = read(PC++);
            A &= value;
            setFlag(Z, A == 0x00);
            setFlag(N, A & 0x80);
            break;
        }

        // AND Zero Page
        case 0x25: {
            uint8_t addr = read(PC++);
            A &= read(addr);
            setFlag(Z, A == 0x00);
            setFlag(N, A & 0x80);
            break;
        }

        // AND Zero Page,X
        case 0x35: {
            uint8_t addr = (read(PC++) + X) & 0xFF;
            A &= read(addr);
            setFlag(Z, A == 0x00);
            setFlag(N, A & 0x80);
            break;
        }

        // AND Absolute
        case 0x2D: {
            uint16_t addr = read(PC++) | (read(PC++) << 8);
            A &= read(addr);
            setFlag(Z, A == 0x00);
            setFlag(N, A & 0x80);
            break;
        }

        // AND Absolute,X
        case 0x3D: {
            uint16_t addr = (read(PC++) | (read(PC++) << 8)) + X;
            A &= read(addr);
            setFlag(Z, A == 0x00);
            setFlag(N, A & 0x80);
            break;
        }

        // AND Absolute,Y
        case 0x39: {
            uint16_t addr = (read(PC++) | (read(PC++) << 8)) + Y;
            A &= read(addr);
            setFlag(Z, A == 0x00);
            setFlag(N, A & 0x80);
            break;
        }

        // AND (Indirect,X)
        case 0x21: {
            uint8_t zp = read(PC++);
            uint16_t addr = (read((zp + X) & 0xFF) | (read((zp + X + 1) & 0xFF) << 8));
            A &= read(addr);
            setFlag(Z, A == 0x00);
            setFlag(N, A & 0x80);
            break;
        }

        // AND (Indirect),Y
        case 0x31: {
            uint8_t zp = read(PC++);
            uint16_t addr = (read(zp) | (read((zp + 1) & 0xFF) << 8)) + Y;
            A &= read(addr);
            setFlag(Z, A == 0x00);
            setFlag(N, A & 0x80);
            break;
        }

        // ORA Immediate
        case 0x09: {
            uint8_t value = read(PC++);
            A |= value;
            setFlag(Z, A == 0x00);
            setFlag(N, A & 0x80);
            break;
        }

        // ORA Zero Page
        case 0x05: {
            uint8_t addr = read(PC++);
            A |= read(addr);
            setFlag(Z, A == 0x00);
            setFlag(N, A & 0x80);
            break;
        }

        // ORA Zero Page,X
        case 0x15: {
            uint8_t addr = (read(PC++) + X) & 0xFF;
            A |= read(addr);
            setFlag(Z, A == 0x00);
            setFlag(N, A & 0x80);
            break;
        }

        // ORA Absolute
        case 0x0D: {
            uint16_t addr = read(PC++) | (read(PC++) << 8);
            A |= read(addr);
            setFlag(Z, A == 0x00);
            setFlag(N, A & 0x80);
            break;
        }

        // EOR Immediate
        case 0x49: {
            uint8_t value = read(PC++);
            A ^= value;
            setFlag(Z, A == 0x00);
            setFlag(N, A & 0x80);
            break;
        }

        // EOR Zero Page
        case 0x45: {
            uint8_t addr = read(PC++);
            A ^= read(addr);
            setFlag(Z, A == 0x00);
            setFlag(N, A & 0x80);
            break;
        }

        // EOR Absolute
        case 0x4D: {
            uint16_t addr = read(PC++) | (read(PC++) << 8);
            A ^= read(addr);
            setFlag(Z, A == 0x00);
            setFlag(N, A & 0x80);
            break;
        }

                // BEQ - Branch if Equal
        case 0xF0: {
            int8_t offset = read(PC++);
            if (getFlag(Z)) {
                PC += offset;
            }
            break;
        }

        // BNE - Branch if Not Equal
        case 0xD0: {
            int8_t offset = read(PC++);
            if (!getFlag(Z)) {
                PC += offset;
            }
            break;
        }

        // BCS - Branch if Carry Set
        case 0xB0: {
            int8_t offset = read(PC++);
            if (getFlag(C)) {
                PC += offset;
            }
            break;
        }

        // BCC - Branch if Carry Clear
        case 0x90: {
            int8_t offset = read(PC++);
            if (!getFlag(C)) {
                PC += offset;
            }
            break;
        }

        // BMI - Branch if Minus
        case 0x30: {
            int8_t offset = read(PC++);
            if (getFlag(N)) {
                PC += offset;
            }
            break;
        }

        // BPL - Branch if Positive
        case 0x10: {
            int8_t offset = read(PC++);
            if (!getFlag(N)) {
                PC += offset;
            }
            break;
        }

        // BVS - Branch if Overflow Set
        case 0x70: {
            int8_t offset = read(PC++);
            if (getFlag(V)) {
                PC += offset;
            }
            break;
        }

        // BVC - Branch if Overflow Clear
        case 0x50: {
            int8_t offset = read(PC++);
            if (!getFlag(V)) {
                PC += offset;
            }
            break;
        }

                // INC Zero Page
        case 0xE6: {
            uint8_t addr = read(PC++);
            uint8_t value = read(addr) + 1;
            write(addr, value);
            setFlag(Z, value == 0x00);
            setFlag(N, value & 0x80);
            break;
        }

        // INC Zero Page,X
        case 0xF6: {
            uint8_t addr = (read(PC++) + X) & 0xFF;
            uint8_t value = read(addr) + 1;
            write(addr, value);
            setFlag(Z, value == 0x00);
            setFlag(N, value & 0x80);
            break;
        }

        // INC Absolute
        case 0xEE: {
            uint16_t addr = read(PC++) | (read(PC++) << 8);
            uint8_t value = read(addr) + 1;
            write(addr, value);
            setFlag(Z, value == 0x00);
            setFlag(N, value & 0x80);
            break;
        }

        // INC Absolute,X
        case 0xFE: {
            uint16_t addr = (read(PC++) | (read(PC++) << 8)) + X;
            uint8_t value = read(addr) + 1;
            write(addr, value);
            setFlag(Z, value == 0x00);
            setFlag(N, value & 0x80);
            break;
        }

        // INX - Increment X
        case 0xE8: {
            X++;
            setFlag(Z, X == 0x00);
            setFlag(N, X & 0x80);
            break;
        }

        // INY - Increment Y
        case 0xC8: {
            Y++;
            setFlag(Z, Y == 0x00);
            setFlag(N, Y & 0x80);
            break;
        }

        // DEC Zero Page
        case 0xC6: {
            uint8_t addr = read(PC++);
            uint8_t value = read(addr) - 1;
            write(addr, value);
            setFlag(Z, value == 0x00);
            setFlag(N, value & 0x80);
            break;
        }

        // DEC Zero Page,X
        case 0xD6: {
            uint8_t addr = (read(PC++) + X) & 0xFF;
            uint8_t value = read(addr) - 1;
            write(addr, value);
            setFlag(Z, value == 0x00);
            setFlag(N, value & 0x80);
            break;
        }

        // DEC Absolute
        case 0xCE: {
            uint16_t addr = read(PC++) | (read(PC++) << 8);
            uint8_t value = read(addr) - 1;
            write(addr, value);
            setFlag(Z, value == 0x00);
            setFlag(N, value & 0x80);
            break;
        }

        // DEC Absolute,X
        case 0xDE: {
            uint16_t addr = (read(PC++) | (read(PC++) << 8)) + X;
            uint8_t value = read(addr) - 1;
            write(addr, value);
            setFlag(Z, value == 0x00);
            setFlag(N, value & 0x80);
            break;
        }

        // DEX - Decrement X
        case 0xCA: {
            X--;
            setFlag(Z, X == 0x00);
            setFlag(N, X & 0x80);
            break;
        }

        // DEY - Decrement Y
        case 0x88: {
            Y--;
            setFlag(Z, Y == 0x00);
            setFlag(N, Y & 0x80);
            break;
        }

                // PHA - Push Accumulator
        case 0x48: {
            write(0x100 + SP--, A);
            break;
        }

        // PLA - Pull Accumulator
        case 0x68: {
            A = read(0x100 + ++SP);
            setFlag(Z, A == 0x00);
            setFlag(N, A & 0x80);
            break;
        }

        // PHP - Push Processor Status
        case 0x08: {
            write(0x100 + SP--, P | 0x10); // Set the break flag
            break;
        }

        // PLP - Pull Processor Status
        case 0x28: {
            P = read(0x100 + ++SP) & ~0x10; // Clear break flag
            break;
        }

                // TAX - Transfer A to X
        case 0xAA: {
            X = A;
            setFlag(Z, X == 0x00);
            setFlag(N, X & 0x80);
            break;
        }

        // TAY - Transfer A to Y
        case 0xA8: {
            Y = A;
            setFlag(Z, Y == 0x00);
            setFlag(N, Y & 0x80);
            break;
        }

        // TXA - Transfer X to A
        case 0x8A: {
            A = X;
            setFlag(Z, A == 0x00);
            setFlag(N, A & 0x80);
            break;
        }

        // TYA - Transfer Y to A
        case 0x98: {
            A = Y;
            setFlag(Z, A == 0x00);
            setFlag(N, A & 0x80);
            break;
        }

        // TSX - Transfer Stack Pointer to X
        case 0xBA: {
            X = SP;
            setFlag(Z, X == 0x00);
            setFlag(N, X & 0x80);
            break;
        }

        // TXS - Transfer X to Stack Pointer
        case 0x9A: {
            SP = X;
            break;
        }

                // CLC - Clear Carry Flag
        case 0x18: {
            setFlag(C, false);
            break;
        }

        // SEC - Set Carry Flag
        case 0x38: {
            setFlag(C, true);
            break;
        }

        // CLI - Clear Interrupt Disable
        case 0x58: {
            setFlag(I, false);
            break;
        }

        // SEI - Set Interrupt Disable
        case 0x78: {
            setFlag(I, true);
            break;
        }

        // CLV - Clear Overflow Flag
        case 0xB8: {
            setFlag(V, false);
            break;
        }

        // CLD - Clear Decimal Mode
        case 0xD8: {
            setFlag(D, false);
            break;
        }

        // SED - Set Decimal Mode
        case 0xF8: {
            setFlag(D, true);  // Note: NES 6502 doesn't actually use decimal mode
            break;
        }

        default:
            std::cerr << "Unknown opcode: " << std::hex << (int)opcode << std::endl;
            break;
    }
}
