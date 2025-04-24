#ifndef BUS_H
#define BUS_H

#include <cstdint>
#include <array>
#include "ppu.h"

class CPU6502;

class Bus {
public:
    CPU6502* cpu;  // The CPU object
    PPU* ppu;

    std::array<uint8_t, 2048> ram{};  // 2KB internal RAM
    std::array<uint8_t, 32768> rom{}; // 32KB PRG-ROM

    Bus();  // Constructor to initialize the bus and connect CPU

    uint8_t read(uint16_t addr);  // Read data from the bus at a specific address
    void write(uint16_t addr, uint8_t data);  // Write data to the bus at a specific address
};

#endif // BUS_H
