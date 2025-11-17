pub struct ScrollRegister {
    pub scroll_x: u8,
    pub scroll_y: u8,
    pub latch: bool,
    // Temporary storage for scroll writes
    temp_x: u8,
    temp_y: u8,
}

impl ScrollRegister {
    pub fn new() -> Self {
        ScrollRegister {
            scroll_x: 0,
            scroll_y: 0,
            latch: false,
            temp_x: 0,
            temp_y: 0,
        }
    }

    pub fn write(&mut self, data: u8) {
        if !self.latch {
            self.temp_x = data;
        } else {
            self.temp_y = data;
        }
        self.latch = !self.latch;
    }

    pub fn reset_latch(&mut self) {
        self.latch = false;
    }

    // Latch the temporary scroll values for rendering (called at frame start)
    pub fn latch_for_render(&mut self) {
        self.scroll_x = self.temp_x;
        self.scroll_y = self.temp_y;
    }
}
