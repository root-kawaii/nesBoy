// use cpu;
use crate::ppu::Ppu;
use crate::rom_loader::RomLoader;

pub struct Bus {
    // cpu: *mut Cpu, // The CPU object
    pub ppu: Ppu,
    prg_rom: [u8; 32768], // PRG-ROM data
    prg_ram: [u8; 2048],  // PRG-RAM (work RAM)
    pub rom: RomLoader,
}

impl Bus {
    pub fn new() -> Self {
        let rom = RomLoader::new("nestest.nes").unwrap();
        let mut prg_rom = [0u8; 32768];

        // Copy ROM data into prg_rom array
        let rom_data = rom.prg_rom();
        let copy_len = rom_data.len().min(32768);
        prg_rom[..copy_len].copy_from_slice(&rom_data[..copy_len]);

        let ppu = Ppu::new(rom.vertical_mirroring, rom.chr_rom.clone());

        Bus {
            prg_rom,
            prg_ram: [0; 2048],
            ppu,
            rom,
        }
    }

    pub fn read(&mut self, mut addr: u16) -> u8 {
        // Implementation of read method
        if addr >= 0x8000 && addr <= 0xFFFF {
            addr -= 0x8000;
            if self.rom.prg_rom().len() == 0x4000 && addr >= 0x4000 {
                //mirror if needed
                addr = addr % 0x4000;
            }
            return self.rom.prg_rom[addr as usize];
        }
        if addr >= 0x2000 && addr <= 0x3FFF {
            // return self.ppu.read(0x2000 + (addr % 8)); // Mirroring every 8 bytes
            return 0 as u8;
        }
        if addr >= 0x0000 && addr <= 0x1FFF {
            return self.prg_ram[(addr % 0x0800) as usize]; // mirror every 2KB
        }
        0
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        // Implementation of write method
        match addr {
            0x0000..= 0x1FFF => {
            self.prg_ram[(addr % 0x0800) as usize] = data;
            }
            0x2000 => {
                self.ppu.write_control(data);
            }
            0x2006 => {
                self.ppu.write_to_ppu_addr(data);
            }
            0x2007 => {
                self.ppu.write_data(data);
            } 
            _ => {
                println!("Ignoring mem write-access at {}", addr);
            }
        // else if addr >= 0x2000 && addr <= 0x3FFF {
        //     // self.ppu.write(0x2000 + (addr % 8), data); // Mirroring every 8 bytes
        // }
        // ROM is read-only in NES
    }
}
