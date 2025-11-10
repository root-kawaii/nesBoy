use crate::add_register::AddrRegister;
use crate::controller_register::ControlRegister;


#[derive(Debug)]
pub enum Mirroring {
    VERTICAL,
    HORIZONTAL,
}

pub struct Ppu {
    // PPU Registers
    pub control: u8,
    pub mask: u8,
    pub status: u8,
    pub oam_addr: u8,
    pub oam_data: u8,
    pub scroll: u8,
    // pub addr: u8,
    pub data: u8,
    pub oam_dma: u8,

    internal_data_buf: u8,

    pub ctrl: ControlRegister,

    // Internal state
    chr_rom: Vec<u8>,
    framebuffer: [u8; 256 * 240],
    vram: [u8; 0x4000],
    name_table: [u8; 1024],
    palette_table: [u8; 64],

    // Rendering state
    scanline: i32,
    cycle: i32,
    frame_complete: bool,
    addr: AddrRegister,

    mirroring: Mirroring,

    tile_id: u8,
    tile_attrib: u8,
    tile_lsb: u8,
    tile_msb: u8,
}

impl Ppu {
    pub fn new(mirroring: bool, chr_rom: Vec<u8>) -> Self {
        Ppu {
            control: 0,
            mask: 0,
            status: 0,
            oam_addr: 0,
            oam_data: 0,
            scroll: 0,
            // addr: 0,
            data: 0,
            oam_dma: 0,
            framebuffer: [0; 256 * 240],
            vram: [0; 0x4000],
            name_table: [0; 1024],
            palette_table: [0; 64],
            scanline: 0,
            cycle: 0,
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
            addr: AddrRegister::new(),
            internal_data_buf: 0,
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

    // pub fn write(&mut self, addr: u16, data: u8) {
    //     // Write to PPU memory or registers
    //     match addr {
    //         0x2000 => self.write_control(addr, data),
    //         0x2001 => self.write_mask(addr, data),
    //         0x2003 => self.oam_addr = data,
    //         0x2004 => self.write_oam(addr, data),
    //         0x2005 => self.scroll = data,
    //         0x2006 => self.addr = data,
    //         0x2007 => self.data = data,
    //         _ => {}
    //     }
    // }

    pub fn step(&mut self) {
        // Perform one PPU cycle
        self.cycle += 1;
        if self.cycle > 340 {
            self.cycle = 0;
            self.scanline += 1;
            if self.scanline > 261 {
                self.scanline = 0;
                self.frame_complete = true;
            }
        }
    }

    pub fn render_tile(&mut self) {
        let tileX = (self.cycle - 1) / 8;
        let tileY = self.scanline / 8;
        let pixelRow = self.scanline % 8;

        for i in 0..8 {
            // Bits from pattern table
            let bit0: u8 = (self.tile_lsb >> (7 - i)) & 1;
            let bit1: u8 = (self.tile_msb >> (7 - i)) & 1;

            let colorIndex = (bit1 << 1) | bit0; // Combine to 2-bit color index

            let x = tileX * 8 + i;
            let y = self.scanline;

            if (x < 256 && y < 240) {
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

    fn write_to_ppu_addr(&mut self, value: u8) {
       self.addr.update(value);
    }

    pub fn is_frame_complete(&self) -> bool {
        self.frame_complete
    }

    pub fn reset_frame_complete(&mut self) {
        self.frame_complete = false;
    }

    fn read_status(&mut self, _addr: u16) -> u8 {
        self.status
    }

    fn write_control(&mut self, _addr: u16, data: u8) {
        self.control = data;
    }

    fn write_mask(&mut self, _addr: u16, data: u8) {
        self.mask = data;
    }

    fn write_oam(&mut self, _addr: u16, data: u8) {
        self.oam_data = data;
    }

    fn fetch_tile_id(&mut self) {
        // Fetch tile ID from name table
    }

    fn fetch_attribute(&mut self) {
        // Fetch attribute byte
    }

    fn fetch_tile_lsb(&mut self) {
        // Fetch tile LSB
    }

    fn fetch_tile_msb(&mut self) {
        // Fetch tile MSB
    }
}













