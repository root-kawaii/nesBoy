#include <cstdint>
#include <iostream>
#include <array>
#include "cpu.h"



int main() {
    Bus nes;
    nes.rom[0] = 0xA9; // LDA #$42
    nes.rom[1] = 0x42;
    
    nes.cpu->reset();
    
    for (int i = 0; i < 10; i++) {
        nes.cpu->step();
    }

    return 0;
}
