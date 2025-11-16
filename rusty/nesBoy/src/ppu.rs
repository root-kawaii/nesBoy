use crate::add_register::AddrRegister;
use crate::controller_register::ControlRegister;
use crate::status::StatusRegister;


#[derive(Debug)]
pub enum Mirroring {
    VERTICAL,
    HORIZONTAL,
}

pub struct Ppu {
    // PPU Registers
    pub control: u8,
    pub mask: u8,
    pub oam_addr: u8,
    pub scroll: u8,
    // pub addr: u8,
    pub data: u8,
    pub oam_dma: u8,

    internal_data_buf: u8,

    pub ctrl: ControlRegister,
    pub status: StatusRegister,

    // Internal state
    pub chr_rom: Vec<u8>,
    framebuffer: [u8; 256 * 240],
    pub vram: [u8; 0x4000],
    name_table: [u8; 1024],
    pub palette_table: [u8; 64],
    pub oam_data: [u8; 256],  // Object Attribute Memory (64 sprites * 4 bytes each)

    // Rendering state
    scanline: i32,
    cycles: usize,
    frame_complete: bool,
    addr: AddrRegister,

    mirroring: Mirroring,

    tile_id: u8,
    tile_attrib: u8,
    tile_lsb: u8,
    tile_msb: u8,

    pub nmi_interrupt: Option<u8>,
}

impl Ppu {
    pub fn new(mirroring: bool, chr_rom: Vec<u8>) -> Self {
        Ppu {
            control: 0,
            mask: 0,
            oam_addr: 0,
            scroll: 0,
            // addr: 0,
            data: 0,
            oam_dma: 0,
            framebuffer: [0; 256 * 240],
            vram: [0; 0x4000],
            name_table: [0; 1024],
            palette_table: [0; 64],
            oam_data: [0; 256],
            scanline: 0,
            cycles: 0,
            frame_complete: false,
            tile_id: 0,
            tile_attrib: 0,
            tile_lsb: 0,
            tile_msb: 0,

            mirroring: if mirroring {
                Mirroring::VERTICAL
            } else {
                Mirroring::HORIZONTAL
            },
            chr_rom,
            ctrl: ControlRegister::new(),
            status: StatusRegister::new(),
            addr: AddrRegister::new(),
            internal_data_buf: 0,
            nmi_interrupt: None,
        }
    }

    pub fn get_frame(&self) -> &[u8; 256 * 240] {
        &self.framebuffer
    }

    fn write_to_ctrl(&mut self, value: u8) {
       self.ctrl.update(value);
    }

    fn increment_vram_addr(&mut self) {
       self.addr.increment(self.ctrl.vram_addr_increment());
    }

    pub fn read_data(&mut self) -> u8 {
        let addr = self.addr.get();
        self.increment_vram_addr();

       match addr {
            0..=0x1fff => {
                let result = self.internal_data_buf;
                self.internal_data_buf = self.chr_rom[addr as usize];
                result
            }
            0x2000..=0x2fff => {
                let result = self.internal_data_buf;
                self.internal_data_buf = self.vram[self.mirror_vram_addr(addr) as usize];
                result
            }
           0x3000..=0x3eff => panic!("addr space 0x3000..0x3eff is not expected to be used, requested = {} ", addr),
           0x3f00..=0x3fff =>
           {
               self.palette_table[(addr - 0x3f00) as usize]
           }
           _ => panic!("unexpected access to mirrored space {}", addr),
       }
    }

    // pub fn read(&mut self, addr: u16) -> u8 {
    //     // Read from PPU memory or registers
    //     match addr {
    //         0x2000 => self.control,
    //         0x2001 => self.mask,
    //         0x2002 => self.read_status(addr),
    //         0x2004 => self.oam_data,
    //         0x2007 => self.data,
    //         _ => 0,
    //     }
    // }


    pub fn write_to_data(&mut self, value: u8) {
        let addr = self.addr.get();
        match addr {
            0..=0x1fff => {}, // CHR ROM is read-only, silently ignore
            0x2000..=0x2fff => {
                self.vram[self.mirror_vram_addr(addr) as usize] = value;
            }
            //Addresses $3F10/$3F14/$3F18/$3F1C are mirrors of $3F00/$3F04/$3F08/$3F0C
            0x3f10 | 0x3f14 | 0x3f18 | 0x3f1c => {
                let add_mirror = addr - 0x10;
                self.palette_table[(add_mirror - 0x3f00) as usize] = value;
            }
            0x3f00..=0x3fff =>
            {
                self.palette_table[(addr - 0x3f00) as usize] = value;
            }
            _ => panic!("unexpected access to mirrored space {}", addr),
        }
        self.increment_vram_addr();
    }

    // pub fn step(&mut self) {
    //     // Perform one PPU cycle
    //     self.cycle += 1;
    //     if self.cycle > 340 {
    //         self.cycle = 0;
    //         self.scanline += 1;
    //         if self.scanline > 261 {
    //             self.scanline = 0;
    //             self.frame_complete = true;
    //         }
    //     }
    // }

    pub fn tick(&mut self, cycles: u8) -> bool {
        self.cycles += cycles as usize;
        if self.cycles >= 341 {
            self.cycles = self.cycles - 341;
            self.scanline += 1;

            if self.scanline == 241 {
                self.status.set_vblank_status(true);
                self.status.set_sprite_zero_hit(false);
                if self.ctrl.generate_vblank_nmi() {
                    self.nmi_interrupt = Some(1);
                }
            }

            if self.scanline >= 262 {
                self.scanline = 0;
                self.nmi_interrupt = None;
                self.status.set_sprite_zero_hit(false);
                self.status.reset_vblank_status();
                return true;
            }
        }
        return false;
    }

    pub fn render_tile(&mut self) {
        let tileX = (self.cycles - 1) / 8;
        let tileY = self.scanline / 8;
        let pixelRow = self.scanline % 8;

        for i in 0..8 {
            // Bits from pattern table
            let bit0: u8 = (self.tile_lsb >> (7 - i)) & 1;
            let bit1: u8 = (self.tile_msb >> (7 - i)) & 1;

            let colorIndex = (bit1 << 1) | bit0; // Combine to 2-bit color index

            let x = tileX * 8 + i;
            let y = self.scanline;

            if x < 256 && y < 240 {
                self.framebuffer[y as usize * 256 + x as usize] = colorIndex; // Save to framebuffer
            }
        }
    }

    // Horizontal:
    //   [ A ] [ a ]
    //   [ B ] [ b ]
    
    // Vertical:
    //   [ A ] [ B ]
    //   [ a ] [ b ]
    pub fn mirror_vram_addr(&self, addr: u16) -> u16 {
        let mirrored_vram = addr & 0b10111111111111; // mirror down 0x3000-0x3eff to 0x2000 - 0x2eff
        let vram_index = mirrored_vram - 0x2000; // to vram vector
        let name_table = vram_index / 0x400;
        match (&self.mirroring, name_table) {
            (Mirroring::VERTICAL, 2) | (Mirroring::VERTICAL, 3) => vram_index - 0x800,
            (Mirroring::HORIZONTAL, 2) => vram_index - 0x400,
            (Mirroring::HORIZONTAL, 1) => vram_index - 0x400,
            (Mirroring::HORIZONTAL, 3) => vram_index - 0x800,
            _ => vram_index,
        }
    }

    pub fn write_to_ppu_addr(&mut self, value: u8) {
       self.addr.update(value);
    }

    pub fn is_frame_complete(&self) -> bool {
        self.frame_complete
    }

    pub fn reset_frame_complete(&mut self) {
        self.frame_complete = false;
    }

    pub fn read_status(&mut self) -> u8 {
        let status = self.status.snapshot();
        self.status.reset_vblank_status();
        self.addr.reset_latch();
        status
    }

    pub fn write_control(&mut self, data: u8) {
        let before_nmi_status = self.ctrl.generate_vblank_nmi();
        self.ctrl.update(data);
        if !before_nmi_status && self.ctrl.generate_vblank_nmi() && self.status.is_in_vblank() {
            self.nmi_interrupt = Some(1);
        }
    }

    pub fn write_mask(&mut self, data: u8) {
        self.mask = data;
    }

    pub fn write_oam_addr(&mut self, data: u8) {
        self.oam_addr = data;
    }

    pub fn write_oam_data(&mut self, data: u8) {
        self.oam_data[self.oam_addr as usize] = data;
        self.oam_addr = self.oam_addr.wrapping_add(1);
    }

    pub fn read_oam_data(&self) -> u8 {
        self.oam_data[self.oam_addr as usize]
    }

    pub fn write_scroll(&mut self, data: u8) {
        self.scroll = data;
    }
}













