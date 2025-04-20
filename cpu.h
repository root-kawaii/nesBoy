#ifndef CPU6502_H
#define CPU6502_H

#include <cstdint>
#include <array>
#include "bus.h"  // Include bus.h since the CPU interacts with the Bus

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

class Bus;

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

    void ConnectBus(Bus* n);

    uint8_t read(uint16_t addr);
    void write(uint16_t addr, uint8_t data);

    // Helpers
    void setFlag(FLAGS6502 flag, bool value);
    bool getFlag(FLAGS6502 flag);

private:
    Bus* bus = nullptr;
};

#endif // CPU6502_H
