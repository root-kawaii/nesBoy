#include "bus.h"

Bus::Bus() {
    cpu->ConnectBus(this); // Let the CPU talk to the bus
}

uint8_t Bus::read(uint16_t addr) {
    if (addr >= 0x8000 && addr <= 0xFFFF)
        return rom[addr - 0x8000];
    if (addr >= 0x0000 && addr <= 0x1FFF)
        return ram[addr % 0x0800]; // mirror every 2KB

    // TODO: Add PPU, ROM, controller reads
    return 0;
}

void Bus::write(uint16_t addr, uint8_t data) {
    if (addr >= 0x0000 && addr <= 0x1FFF)
        ram[addr % 0x0800] = data;

    // TODO: Add PPU, ROM, controller writes
}