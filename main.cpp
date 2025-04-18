#include <stdint.h>

#include <iostream>
#include <array>

enum FLAGS6502 {
    C = (1 << 0), // Carry
    Z = (1 << 1), // Zero
    I = (1 << 2), // Interrupt Disable
    D = (1 << 3), // Decimal Mode (not used on NES)
    B = (1 << 4), // Break
    U = (1 << 5), // Unused (always 1)
    V = (1 << 6), // Overflow
    N = (1 << 7)  // Negative
};


class CPU6502 {
public:
    uint8_t A = 0x00;     // Accumulator
    uint8_t X = 0x00;     // X register
    uint8_t Y = 0x00;     // Y register
    uint8_t SP = 0xFD;    // Stack Pointer
    uint16_t PC = 0x8000; // Program Counter
    uint8_t P = 0x24;     // Status Register

    std::array<uint8_t, 0x10000> memory{}; // 64KB RAM

    void reset();
    void step();
    void execute(uint8_t opcode);

    // Helpers
    void setFlag(FLAGS6502 flag, bool value);
    bool getFlag(FLAGS6502 flag);
};

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
    uint8_t opcode = memory[PC++];
    execute(opcode);
}

void CPU6502::execute(uint8_t opcode) {
    switch (opcode) {
        case 0xA9: { // LDA Immediate
            uint8_t value = memory[PC++];
            A = value;
            setFlag(Z, A == 0x00);
            setFlag(N, A & 0x80);
            break;
        }

        default:
            std::cerr << "Unknown opcode: " << std::hex << (int)opcode << std::endl;
            break;
    }
}


int main() {
    CPU6502 cpu;
    cpu.reset();

    // Load program: LDA #$42
    cpu.memory[0x8000] = 0xA9;
    cpu.memory[0x8001] = 0x42;

    cpu.step();

    std::cout << "A = $" << std::hex << (int)cpu.A << std::endl; // Expect $42
    std::cout << "Zero Flag = " << cpu.getFlag(Z) << std::endl;
    std::cout << "Negative Flag = " << cpu.getFlag(N) << std::endl;

    return 0;
}
