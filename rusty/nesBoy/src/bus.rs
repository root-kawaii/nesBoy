// use cpu;
use crate::ppu::Ppu;
use crate::rom_loader::RomLoader;

pub struct Bus {
    // cpu: *mut Cpu, // The CPU object
    ppu: Ppu,
    prg_rom: [u8; 32768], // PRG-ROM data
    prg_ram: [u8; 2048],  // PRG-RAM (work RAM)
    rom: RomLoader,
}

impl Bus {
    pub fn new() -> Self {
        let rom = RomLoader::new("ff.nes").unwrap();
        let mut prg_rom = [0u8; 32768];

        // Copy ROM data into prg_rom array
        let rom_data = rom.prg_rom();
        let copy_len = rom_data.len().min(32768);
        prg_rom[..copy_len].copy_from_slice(&rom_data[..copy_len]);

        Bus {
            prg_rom,
            prg_ram: [0; 2048],
            ppu: Ppu::new(),
            rom,
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
