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

void PPU::setStatusFlag(bool vblank) {
    if (vblank)
        status |= 0x80; // Set VBlank bit
    else
        status &= ~0x80;
}

void PPU::fetchTileID() {
    uint16_t baseAddr = 0x2000; // For now, use nametable 0
    uint16_t ntIndex = ((scanline / 8) * 32) + ((cycle - 1) / 8);
    tileID = read(baseAddr + ntIndex);
}

void PPU::fetchAttribute() {
    tileAttrib = 0; // Placeholder (we'll calculate from attribute table later)
}

void PPU::fetchTileLSB() {
    uint16_t patternBase = 0x0000; // For now, fixed
    uint16_t tileAddr = patternBase + (tileID * 16) + (scanline % 8);
    tileLSB = read(tileAddr);
}

void PPU::fetchTileMSB() {
    uint16_t patternBase = 0x0000;
    uint16_t tileAddr = patternBase + (tileID * 16) + (scanline % 8) + 8;
    tileMSB = read(tileAddr);
}

void PPU::renderTile() {
    int tileX = (cycle - 1) / 8;
    int tileY = scanline / 8;
    int pixelRow = scanline % 8;

    for (int i = 0; i < 8; i++) {
        // Bits from pattern table
        uint8_t bit0 = (tileLSB >> (7 - i)) & 1;
        uint8_t bit1 = (tileMSB >> (7 - i)) & 1;

        uint8_t colorIndex = (bit1 << 1) | bit0; // Combine to 2-bit color index

        int x = tileX * 8 + i;
        int y = scanline;

        if (x < 256 && y < 240) {
            framebuffer[y * 256 + x] = colorIndex; // Save to framebuffer
        }
    }
}

void PPU::step() {
    // Do something based on current scanline and cycle
    if (scanline >= 0 && scanline <= 239) {
        // Visible scanlines
        if (cycle == 1) {
            // Start of scanline, fetch background, etc.
        }
        if (cycle == 256) {
            // End of tile fetch
        }
        if (cycle == 340) {
            // Move to next scanline
        }
    }
    else if (scanline == 240) {
        // Post-render line, do nothing
    }
    else if (scanline == 241 && cycle == 1) {
        // Enter VBlank
        setStatusFlag(true); // (You'll need to define this)
    }
    else if (scanline == 261 && cycle == 1) {
        // Exit VBlank (start pre-render)
        setStatusFlag(false);
    }

    // Advance the timing
    cycle++;
    if (cycle >= 341) {
        cycle = 0;
        scanline++;
        if (scanline >= 262) {
            scanline = 0;
            frame_complete = true;
        }
    }
}
