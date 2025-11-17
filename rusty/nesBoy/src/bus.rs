// use cpu;
use crate::joypad::Joypad;
use crate::ppu::Ppu;
use crate::rom_loader::RomLoader;
use std::thread::sleep;
use std::time::Duration;
use std::time::SystemTime;

const PPU_REGISTERS_MIRRORS_END: u16 = 0x3FFF;
const RAM: u16 = 0x0000;
const RAM_MIRRORS_END: u16 = 0x1FFF;

pub struct Bus<'call> {
    cpu_vram: [u8; 2048],
    prg_rom: Vec<u8>,
    pub ppu: Ppu,
    pub cycles: usize,
    gameloop_callback: Box<dyn FnMut(&Ppu, &mut Joypad) + 'call>,
    joypad1: Joypad,
    timer: usize,
}

impl<'call> Bus<'call> {
    pub fn new<F>(rom: RomLoader, gameloop_callback: F) -> Bus<'call>
    where
        F: FnMut(&Ppu, &mut Joypad) + 'call,
    {
        let ppu = Ppu::new(rom.vertical_mirroring, rom.chr_rom.clone());

        Bus {
            cpu_vram: [0; 2048],
            prg_rom: rom.prg_rom,
            ppu,
            cycles: 0,
            gameloop_callback: Box::from(gameloop_callback),
            joypad1: Joypad::new(),
            timer: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis() as usize,
        }
    }

    pub fn read_u16(&mut self, addr: u16) -> u16 {
        let lo = self.read(addr) as u16;
        let hi = self.read(addr.wrapping_add(1)) as u16;
        (hi << 8) | lo
    }

    pub fn read(&mut self, mut addr: u16) -> u8 {
        // Implementation of read method
        if addr >= 0x8000 && addr <= 0xFFFF {
            addr -= 0x8000;
            if self.prg_rom.len() == 0x4000 && addr >= 0x4000 {
                //mirror if needed
                addr = addr % 0x4000;
            }
            return self.prg_rom[addr as usize];
        }
        if addr >= 0x2000 && addr <= 0x3FFF {
            let mirror_addr = 0x2000 + (addr % 8); // Mirror every 8 bytes
            return match mirror_addr {
                0x2002 => self.ppu.read_status(),
                0x2004 => self.ppu.read_oam_data(),
                0x2007 => self.ppu.read_data(),
                _ => 0,
            };
        } else if addr >= 0x0000 && addr <= 0x1FFF {
            return self.cpu_vram[(addr % 0x0800) as usize]; // mirror every 2KB
        } else if addr == 0x4016 {
            // Controller 1 read
            self.joypad1.read()
        } else if addr == 0x4017 {
            // Controller 2 (not implemented)
            return 0;
        } else {
            0
        }
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        // Implementation of write method
        match addr {
            RAM..=RAM_MIRRORS_END => {
                let mirror_down_addr = addr & 0b11111111111;
                self.cpu_vram[mirror_down_addr as usize] = data;
            }
            0x2000 => {
                self.ppu.write_control(data);
            }
            0x2001 => {
                self.ppu.write_mask(data);
            }
            0x2003 => {
                self.ppu.write_oam_addr(data);
            }
            0x2004 => {
                self.ppu.write_oam_data(data);
            }
            0x2005 => {
                self.ppu.write_to_scroll(data);
            }
            0x2006 => {
                self.ppu.write_to_ppu_addr(data);
            }
            0x2007 => {
                self.ppu.write_to_data(data);
            }
            0x4014 => {
                // OAM DMA - Copy 256 bytes from CPU memory to OAM
                let page = (data as u16) << 8;
                for i in 0..256u16 {
                    let byte = if page + i >= 0x8000 && page + i <= 0xFFFF {
                        // PRG ROM
                        let mut addr = page + i - 0x8000;
                        if self.prg_rom.len() == 0x4000 && addr >= 0x4000 {
                            addr = addr % 0x4000;
                        }
                        self.prg_rom[addr as usize]
                    } else if page + i >= 0x0000 && page + i <= 0x1FFF {
                        // RAM
                        self.cpu_vram[((page + i) % 0x0800) as usize]
                    } else {
                        0
                    };
                    self.ppu.oam_data[i as usize] = byte;
                }
            }
            0x4016 => {
                // Controller 1 strobe
                self.joypad1.write(data);
            }
            0x4017 => {
                // APU and controller 2 (not fully implemented)
            }
            0x2008..=PPU_REGISTERS_MIRRORS_END => {
                let mirror_down_addr = addr & 0b00100000_00000111;
                self.write(mirror_down_addr, data);
            }
            0x8000..=0xFFFF => {
                // Mapper writes - ignore for now (TODO: implement mapper support)
                // println!("Ignoring mapper write at {:04X} = {:02X}", addr, data);
            }
            _ => {
                // println!("Ignoring mem write-access at {}", addr);
            }
        }
        // else if addr >= 0x2000 && addr <= 0x3FFF {
        //     // self.ppu.write(0x2000 + (addr % 8), data); // Mirroring every 8 bytes
        // }
        // ROM is read-only in NES
    }

    pub fn tick(&mut self, cycles: u8) {
        self.cycles += cycles as usize;
        let new_frame = self.ppu.tick(cycles * 3);

        if new_frame {
            (self.gameloop_callback)(&self.ppu, &mut self.joypad1);
            if self.timer < 8 {
                sleep(Duration::from_millis(8 - self.timer as u64));
            }
            self.timer = 0;
        }
    }

    pub fn poll_nmi_status(&mut self) -> Option<()> {
        if self.ppu.nmi_interrupt.is_some() {
            self.ppu.nmi_interrupt = None;
            Some(())
        } else {
            None
        }
    }
}
