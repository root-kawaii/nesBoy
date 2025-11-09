pub struct Ppu {
    // PPU Registers
    pub control: u8,
    pub mask: u8,
    pub status: u8,
    pub oam_addr: u8,
    pub oam_data: u8,
    pub scroll: u8,
    pub addr: u8,
    pub data: u8,
    pub oam_dma: u8,

    // Internal state
    framebuffer: [u8; 256 * 240],
    vram: [u8; 0x4000],
    name_table: [u8; 1024],
    palette: [u8; 64],

    // Rendering state
    scanline: i32,
    cycle: i32,
    frame_complete: bool,

    tile_id: u8,
    tile_attrib: u8,
    tile_lsb: u8,
    tile_msb: u8,
}

impl Ppu {
    pub fn new() -> Self {
        Ppu {
            control: 0,
            mask: 0,
            status: 0,
            oam_addr: 0,
            oam_data: 0,
            scroll: 0,
            addr: 0,
            data: 0,
            oam_dma: 0,
            framebuffer: [0; 256 * 240],
            vram: [0; 0x4000],
            name_table: [0; 1024],
            palette: [0; 64],
            scanline: 0,
            cycle: 0,
            frame_complete: false,
            tile_id: 0,
            tile_attrib: 0,
            tile_lsb: 0,
            tile_msb: 0,
        }
    }

    pub fn get_frame(&self) -> &[u8; 256 * 240] {
        &self.framebuffer
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        // Read from PPU memory or registers
        match addr {
            0x2000 => self.control,
            0x2001 => self.mask,
            0x2002 => self.read_status(addr),
            0x2004 => self.oam_data,
            0x2007 => self.data,
            _ => 0,
        }
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        // Write to PPU memory or registers
        match addr {
            0x2000 => self.write_control(addr, data),
            0x2001 => self.write_mask(addr, data),
            0x2003 => self.oam_addr = data,
            0x2004 => self.write_oam(addr, data),
            0x2005 => self.scroll = data,
            0x2006 => self.addr = data,
            0x2007 => self.data = data,
            _ => {}
        }
    }

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
