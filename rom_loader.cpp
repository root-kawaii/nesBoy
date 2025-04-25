
#include <cstdint>
#include <array>
#include "ppu.h"
#include <iostream>
#include <fstream>
#include <vector>
#include <string>
#include <iostream>
#include <stdexcept>
#include <cstdint>
#include "rom_loader.h"


bool RomLoader::loadRom(const std::string& filename) {
    std::ifstream file(filename, std::ios::binary);
    
    if (!file) {
        std::cerr << "Error: Could not open file " << filename << std::endl;
        return false;
    }

    // Read the header
    NESHeader header;
    file.read(reinterpret_cast<char*>(&header), sizeof(NESHeader));
    
    // Verify the file signature "NES^Z"
    if (header.signature[0] != 'N' || header.signature[1] != 'E' || 
        header.signature[2] != 'S' || header.signature[3] != 0x1A) {
        std::cerr << "Error: Invalid NES ROM file format" << std::endl;
        return false;
    }
    
    // Extract mapper number (bits 4-7 from flags6 and bits 4-7 from flags7)
    mapper = (header.flags7 & 0xF0) | ((header.flags6 & 0xF0) >> 4);
    
    // Extract mirroring type
    verticalMirroring = (header.flags6 & 0x01) != 0;
    fourScreenMode = (header.flags6 & 0x08) != 0;
    
    // Check for battery-backed RAM
    hasBattery = (header.flags6 & 0x02) != 0;
    
    // Check for trainer
    hasTrainer = (header.flags6 & 0x04) != 0;
    
    // Skip trainer if present (512 bytes)
    if (hasTrainer) {
        file.seekg(512, std::ios::cur);
    }
    
    // Read PRG ROM (16KB units)
    size_t prgRomBytes = header.prgRomSize * 16384;
    prgRom.resize(prgRomBytes);
    file.read(reinterpret_cast<char*>(prgRom.data()), prgRomBytes);
    
    // Read CHR ROM (8KB units) if present
    if (header.chrRomSize > 0) {
        size_t chrRomBytes = header.chrRomSize * 8192;
        chrRom.resize(chrRomBytes);
        file.read(reinterpret_cast<char*>(chrRom.data()), chrRomBytes);
    } else {
        // No CHR ROM, the game uses CHR RAM
        chrRom.clear();
    }
    
    std::cout << "ROM loaded successfully:" << std::endl;
    std::cout << "  Mapper: " << static_cast<int>(mapper) << std::endl;
    std::cout << "  PRG ROM: " << (prgRomBytes / 1024) << "KB" << std::endl;
    std::cout << "  CHR ROM: " << (chrRom.size() / 1024) << "KB" << std::endl;
    std::cout << "  Mirroring: " << (verticalMirroring ? "Vertical" : "Horizontal") << std::endl;
    if (fourScreenMode) std::cout << "  Four-screen VRAM" << std::endl;
    if (hasBattery) std::cout << "  Battery-backed RAM" << std::endl;
    if (hasTrainer) std::cout << "  Trainer present" << std::endl;
    
    return true;
}