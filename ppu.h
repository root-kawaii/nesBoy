#ifndef PPU_H
#define PPU_H

#include <array>
#include <cstdint>

class PPU {
public:
    // PPU Registers
    uint8_t control;  // Control register
    uint8_t mask;     // Mask register (for rendering options)
    uint8_t status;   // Status register (read-only by CPU)
    uint8_t oam_addr; // OAM (Object Attribute Memory) address for sprites
    uint8_t oam_data; // OAM data (sprite data)

    // PPU Video Memory (VRAM)
    std::array<uint8_t, 0x4000> vram; // 16KB of VRAM

    PPU();

    uint8_t read(uint16_t addr);    // Read memory or registers
    void write(uint16_t addr, uint8_t data);  // Write to memory or registers
    void step();  // Perform one step of the PPU cycle (rendering, etc.)
    
private:
    // Internal functions to handle specific memory ranges
    uint8_t readControl(uint16_t addr);
    uint8_t readMask(uint16_t addr);
    uint8_t readStatus(uint16_t addr);
    uint8_t readOAM(uint16_t addr);

    void writeControl(uint16_t addr, uint8_t data);
    void writeMask(uint16_t addr, uint8_t data);
    void writeOAM(uint16_t addr, uint8_t data);

    // More internal functionality can be added for handling rendering
};

#endif
