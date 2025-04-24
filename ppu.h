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

    const std::array<uint8_t, 256 * 240>& getFrame() const { return framebuffer; }

    PPU();

    uint8_t read(uint16_t addr);    // Read memory or registers
    void write(uint16_t addr, uint8_t data);  // Write to memory or registers
    void step();  // Perform one step of the PPU cycle (rendering, etc.)

    bool isFrameComplete() const { return frame_complete; }
    void resetFrameComplete() { frame_complete = false; }
    
private:

    std::array<uint8_t, 256 * 240> framebuffer{};
    // PPU Video Memory (VRAM)
    std::array<uint8_t, 0x4000> vram; // 16KB of VRAM
    std::array<uint8_t, 1024> nameTable{};
    std::array<uint8_t, 64> palette{};
    // Internal functions to handle specific memory ranges
    uint8_t readControl(uint16_t addr);
    uint8_t readMask(uint16_t addr);
    uint8_t readStatus(uint16_t addr);
    uint8_t readOAM(uint16_t addr);

    void writeControl(uint16_t addr, uint8_t data);
    void writeMask(uint16_t addr, uint8_t data);
    void writeOAM(uint16_t addr, uint8_t data);

    void setStatusFlag(bool flag);


    void fetchTileID();
    void fetchAttribute();
    void fetchTileLSB();
    void fetchTileMSB();

    void renderTile();

    int scanline = 0; // -1 (pre-render) to 261
    int cycle = 0;    // 0 to 340
    bool frame_complete = false;

    uint8_t tileID = 0;
    uint8_t tileAttrib = 0;
    uint8_t tileLSB = 0;
    uint8_t tileMSB = 0;

    // More internal functionality can be added for handling rendering
};

#endif
