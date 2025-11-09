// use cpu;
use crate::ppu::Ppu;

pub struct Bus {
    // cpu: *mut Cpu, // The CPU object
    ppu: Ppu,
    prg_rom: [u8; 32768], // PRG-ROM data
    prg_ram: [u8; 2048],  // PRG-RAM (work RAM)
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            prg_rom: [0; 32768],
            prg_ram: [0; 2048],
            ppu: Ppu::new(),
            // cpu: std::ptr::null_mut(),   // CPU will be set later
        }
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        // Implementation of read method
        if addr >= 0x8000 && addr <= 0xFFFF {
            return self.prg_rom[(addr - 0x8000) as usize];
        }
        if addr >= 0x2000 && addr <= 0x3FFF {
            return self.ppu.read(0x2000 + (addr % 8)); // Mirroring every 8 bytes
        }
        if addr >= 0x0000 && addr <= 0x1FFF {
            return self.prg_ram[(addr % 0x0800) as usize]; // mirror every 2KB
        }
        0
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        // Implementation of write method
        if addr >= 0x0000 && addr <= 0x1FFF {
            self.prg_ram[(addr % 0x0800) as usize] = data;
        } else if addr >= 0x2000 && addr <= 0x3FFF {
            self.ppu.write(0x2000 + (addr % 8), data); // Mirroring every 8 bytes
        }
        // ROM is read-only in NES
    }
}
