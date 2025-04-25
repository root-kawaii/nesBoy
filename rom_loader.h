#ifndef ROM_LOADER_H
#define ROM_LOADER_H

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


class RomLoader{
public:
    bool loadRom(const std::string& filename);

    const std::vector<uint8_t>& getPrgRom() const { return prgRom; }
    const std::vector<uint8_t>& getChrRom() const { return chrRom; }
    uint8_t getMapper() const { return mapper; }
    bool hasVerticalMirroring() const { return verticalMirroring; }
    bool hasFourScreenMode() const { return fourScreenMode; }
    bool hasBatteryBackedRam() const { return hasBattery; }

    // iNES header structure
    struct NESHeader {
        char signature[4];        // Should be "NES" followed by MS-DOS EOF
        uint8_t prgRomSize;       // PRG ROM size in 16KB units
        uint8_t chrRomSize;       // CHR ROM size in 8KB units
        uint8_t flags6;           // Mapper, mirroring, battery, trainer
        uint8_t flags7;           // Mapper, VS/Playchoice, NES 2.0
        uint8_t flags8;           // PRG-RAM size (rarely used)
        uint8_t flags9;           // TV system (rarely used)
        uint8_t flags10;          // TV system, PRG-RAM (rarely used)
        uint8_t padding[5];       // Unused padding
    };

private:



    // ROM data
    std::vector<uint8_t> prgRom;  // Program ROM
    std::vector<uint8_t> chrRom;  // Character ROM
    
    // ROM metadata
    uint8_t mapper;
    bool hasTrainer;
    bool hasBattery;
    bool verticalMirroring;
    bool fourScreenMode;



};

#endif