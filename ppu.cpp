#include "ppu.h"
#include <iostream>

PPU::PPU() {
    // Initialize registers to default values
    control = 0x00;
    mask = 0x00;
    status = 0x00;
    oam_addr = 0x00;
    oam_data = 0x00;
    vram.fill(0);  // Fill VRAM with 0
}

uint8_t PPU::read(uint16_t addr) {
    if (addr >= 0x2000 && addr <= 0x3FFF) {
        // Reading from the PPU memory (VRAM)
        return vram[addr - 0x2000];
    }

    // Handle register reads (status, mask, control, etc.)
    if (addr == 0x2000) return readControl(addr);
    if (addr == 0x2001) return readMask(addr);
    if (addr == 0x2002) return readStatus(addr);
    if (addr == 0x2003) return readOAM(addr);

    std::cerr << "Unknown PPU read address: " << std::hex << addr << std::endl;
    return 0x00;
}

void PPU::write(uint16_t addr, uint8_t data) {
    if (addr >= 0x2000 && addr <= 0x3FFF) {
        // Writing to the PPU memory (VRAM)
        vram[addr - 0x2000] = data;
        return;
    }

    // Handle register writes (status, mask, control, etc.)
    if (addr == 0x2000) {
        writeControl(addr, data);
    } else if (addr == 0x2001) {
        writeMask(addr, data);
    } else if (addr == 0x2003) {
        writeOAM(addr, data);
    }

    // Other control registers can be added here
}

uint8_t PPU::readControl(uint16_t addr) {
    // Control register (e.g., for enabling rendering)
    return control;
}

uint8_t PPU::readMask(uint16_t addr) {
    // Mask register (e.g., for adjusting sprite rendering settings)
    return mask;
}

uint8_t PPU::readStatus(uint16_t addr) {
    // Status register (read-only by CPU, returns the current PPU status)
    return status;
}

uint8_t PPU::readOAM(uint16_t addr) {
    // Object Attribute Memory (OAM) for sprite data
    return oam_data;
}

void PPU::writeControl(uint16_t addr, uint8_t data) {
    // Set control registers (enabling rendering, etc.)
    control = data;
}

void PPU::writeMask(uint16_t addr, uint8_t data) {
    // Set mask registers (e.g., to enable or disable rendering)
    mask = data;
}

void PPU::writeOAM(uint16_t addr, uint8_t data) {
    // Write sprite data into OAM (Object Attribute Memory)
    oam_data = data;
}

void PPU::step() {
    // Perform one step of the PPU cycle (e.g., rendering a frame, updating the screen, etc.)
    // Placeholder for rendering and other functionality

    // In a real implementation, this would interact with the CPU and the screen
    std::cout << "PPU step!" << std::endl;
}
