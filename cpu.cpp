#include "cpu.h"
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
}

uint8_t CPU6502::read(uint16_t addr) {
    return bus->read(addr);
}

void CPU6502::write(uint16_t addr, uint8_t data) {
    bus->write(addr, data);
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

        default:
            std::cerr << "Unknown opcode: " << std::hex << (int)opcode << std::endl;
            break;
    }
}

void CPU6502::ConnectBus(Bus* n) {
    bus = n;
}
