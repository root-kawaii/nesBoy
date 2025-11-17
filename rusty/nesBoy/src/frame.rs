use crate::mirroring::Mirroring;
use crate::ppu::Ppu;

#[rustfmt::skip]

struct Rect {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

impl Rect {
    fn new(x1: usize, y1: usize, x2: usize, y2: usize) -> Self {
        Rect {
            x1: x1,
            y1: y1,
            x2: x2,
            y2: y2,
        }
    }
}

pub static SYSTEM_PALLETE: [(u8, u8, u8); 64] = [
    (0x80, 0x80, 0x80),
    (0x00, 0x3D, 0xA6),
    (0x00, 0x12, 0xB0),
    (0x44, 0x00, 0x96),
    (0xA1, 0x00, 0x5E),
    (0xC7, 0x00, 0x28),
    (0xBA, 0x06, 0x00),
    (0x8C, 0x17, 0x00),
    (0x5C, 0x2F, 0x00),
    (0x10, 0x45, 0x00),
    (0x05, 0x4A, 0x00),
    (0x00, 0x47, 0x2E),
    (0x00, 0x41, 0x66),
    (0x00, 0x00, 0x00),
    (0x05, 0x05, 0x05),
    (0x05, 0x05, 0x05),
    (0xC7, 0xC7, 0xC7),
    (0x00, 0x77, 0xFF),
    (0x21, 0x55, 0xFF),
    (0x82, 0x37, 0xFA),
    (0xEB, 0x2F, 0xB5),
    (0xFF, 0x29, 0x50),
    (0xFF, 0x22, 0x00),
    (0xD6, 0x32, 0x00),
    (0xC4, 0x62, 0x00),
    (0x35, 0x80, 0x00),
    (0x05, 0x8F, 0x00),
    (0x00, 0x8A, 0x55),
    (0x00, 0x99, 0xCC),
    (0x21, 0x21, 0x21),
    (0x09, 0x09, 0x09),
    (0x09, 0x09, 0x09),
    (0xFF, 0xFF, 0xFF),
    (0x0F, 0xD7, 0xFF),
    (0x69, 0xA2, 0xFF),
    (0xD4, 0x80, 0xFF),
    (0xFF, 0x45, 0xF3),
    (0xFF, 0x61, 0x8B),
    (0xFF, 0x88, 0x33),
    (0xFF, 0x9C, 0x12),
    (0xFA, 0xBC, 0x20),
    (0x9F, 0xE3, 0x0E),
    (0x2B, 0xF0, 0x35),
    (0x0C, 0xF0, 0xA4),
    (0x05, 0xFB, 0xFF),
    (0x5E, 0x5E, 0x5E),
    (0x0D, 0x0D, 0x0D),
    (0x0D, 0x0D, 0x0D),
    (0xFF, 0xFF, 0xFF),
    (0xA6, 0xFC, 0xFF),
    (0xB3, 0xEC, 0xFF),
    (0xDA, 0xAB, 0xEB),
    (0xFF, 0xA8, 0xF9),
    (0xFF, 0xAB, 0xB3),
    (0xFF, 0xD2, 0xB0),
    (0xFF, 0xEF, 0xA6),
    (0xFF, 0xF7, 0x9C),
    (0xD7, 0xE8, 0x95),
    (0xA6, 0xED, 0xAF),
    (0xA2, 0xF2, 0xDA),
    (0x99, 0xFF, 0xFC),
    (0xDD, 0xDD, 0xDD),
    (0x11, 0x11, 0x11),
    (0x11, 0x11, 0x11),
];

fn render_name_table(
    ppu: &Ppu,
    frame: &mut Frame,
    name_table: &[u8],
    view_port: Rect,
    shift_x: isize,
    shift_y: isize,
) {
    let bank = ppu.ctrl.bknd_pattern_addr();

    let attribute_table = &name_table[0x3c0..0x400];

    for i in 0..0x3c0 {
        let tile_column = i % 32;
        let tile_row = i / 32;
        let tile_idx = name_table[i] as u16;

        // Check if CHR ROM has enough data for this tile
        let tile_start = (bank + tile_idx * 16) as usize;
        let tile_end = tile_start + 15;
        if ppu.chr_rom.is_empty() || tile_end >= ppu.chr_rom.len() {
            continue; // Skip this tile if CHR ROM doesn't have the data
        }

        let tile = &ppu.chr_rom[tile_start..=tile_end];
        let palette = bg_pallette(ppu, attribute_table, tile_column, tile_row);

        for y in 0..=7 {
            let mut upper = tile[y];
            let mut lower = tile[y + 8];

            for x in (0..=7).rev() {
                let value = (1 & lower) << 1 | (1 & upper);
                upper = upper >> 1;
                lower = lower >> 1;
                let rgb = match value {
                    0 => SYSTEM_PALLETE[ppu.palette_table[0] as usize],
                    1 => SYSTEM_PALLETE[palette[1] as usize],
                    2 => SYSTEM_PALLETE[palette[2] as usize],
                    3 => SYSTEM_PALLETE[palette[3] as usize],
                    _ => panic!("can't be"),
                };
                let pixel_x = tile_column * 8 + x;
                let pixel_y = tile_row * 8 + y;

                if pixel_x >= view_port.x1
                    && pixel_x < view_port.x2
                    && pixel_y >= view_port.y1
                    && pixel_y < view_port.y2
                {
                    let screen_x = shift_x + pixel_x as isize;
                    let screen_y = shift_y + pixel_y as isize;

                    // Bounds check before setting pixel
                    if screen_x >= 0 && screen_x < 256 && screen_y >= 0 && screen_y < 240 {
                        frame.set_pixel(screen_x as usize, screen_y as usize, rgb);
                    }
                }
            }
        }
    }
}

fn sprite_palette(ppu: &Ppu, pallete_idx: u8) -> [u8; 4] {
    let start = 0x11 + (pallete_idx * 4) as usize;
    [
        0,
        ppu.palette_table[start],
        ppu.palette_table[start + 1],
        ppu.palette_table[start + 2],
    ]
}

fn bg_pallette(ppu: &Ppu, attribute_table: &[u8], tile_column: usize, tile_row: usize) -> [u8; 4] {
    let attr_table_idx = tile_row / 4 * 8 + tile_column / 4;
    let attr_byte = attribute_table[attr_table_idx];

    let pallet_idx = match (tile_column % 4 / 2, tile_row % 4 / 2) {
        (0, 0) => attr_byte & 0b11,
        (1, 0) => (attr_byte >> 2) & 0b11,
        (0, 1) => (attr_byte >> 4) & 0b11,
        (1, 1) => (attr_byte >> 6) & 0b11,
        (_, _) => panic!("should not happen"),
    };

    let pallete_start: usize = 1 + (pallet_idx as usize) * 4;
    [
        ppu.palette_table[0],
        ppu.palette_table[pallete_start],
        ppu.palette_table[pallete_start + 1],
        ppu.palette_table[pallete_start + 2],
    ]
}

pub struct Frame {
    pub data: Vec<u8>,
}

impl Frame {
    const WIDTH: usize = 256;
    const HEIGHT: usize = 240;

    pub fn new() -> Self {
        Frame {
            data: vec![0; (Frame::WIDTH * Frame::HEIGHT * 3) as usize],
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, rgb: (u8, u8, u8)) {
        let base = y * 3 * Frame::WIDTH + x * 3;
        if base + 2 < self.data.len() {
            self.data[base] = rgb.0;
            self.data[base + 1] = rgb.1;
            self.data[base + 2] = rgb.2;
        }
    }

    pub fn clear(&mut self, rgb: (u8, u8, u8)) {
        for i in 0..(Frame::WIDTH * Frame::HEIGHT) {
            let base = i * 3;
            self.data[base] = rgb.0;
            self.data[base + 1] = rgb.1;
            self.data[base + 2] = rgb.2;
        }
    }

    pub fn show_tile(chr_rom: &Vec<u8>, bank: usize, tile_n: usize) -> Frame {
        assert!(bank <= 1);

        let mut frame = Frame::new();
        let bank = (bank * 0x1000) as usize;

        let tile = &chr_rom[(bank + tile_n * 16)..=(bank + tile_n * 16 + 15)];

        for y in 0..=7 {
            let mut upper = tile[y];
            let mut lower = tile[y + 8];

            for x in (0..=7).rev() {
                let value = (1 & upper) << 1 | (1 & lower);
                upper = upper >> 1;
                lower = lower >> 1;
                let rgb = match value {
                    0 => SYSTEM_PALLETE[0x01],
                    1 => SYSTEM_PALLETE[0x23],
                    2 => SYSTEM_PALLETE[0x27],
                    3 => SYSTEM_PALLETE[0x30],
                    _ => panic!("can't be"),
                };
                frame.set_pixel(x, y, rgb)
            }
        }

        frame
    }

    pub fn show_tile_bank(chr_rom: &Vec<u8>, bank: usize) -> Frame {
        assert!(bank <= 1);

        let mut frame = Frame::new();
        let bank_offset = bank * 0x1000;

        // Calculate how many tiles we can fit: 256x240 screen
        // Each tile is 8x8, so we can fit 32x30 tiles
        // CHR ROM bank has 256 tiles (0x1000 bytes / 16 bytes per tile)
        // We'll display them in a 16x16 grid (256 tiles)

        for tile_n in 0..256 {
            let tile_x = (tile_n % 16) * 8; // 16 tiles per row
            let tile_y = (tile_n / 16) * 8; // 16 rows

            if tile_y >= 240 {
                break; // Don't draw beyond screen height
            }

            let tile_offset = bank_offset + tile_n * 16;
            if tile_offset + 15 >= chr_rom.len() {
                break; // Don't read beyond CHR ROM
            }

            let tile = &chr_rom[tile_offset..=tile_offset + 15];

            for y in 0..=7 {
                let mut upper = tile[y];
                let mut lower = tile[y + 8];

                for x in (0..=7).rev() {
                    let value = (1 & upper) << 1 | (1 & lower);
                    upper = upper >> 1;
                    lower = lower >> 1;
                    let rgb = match value {
                        0 => SYSTEM_PALLETE[0x01],
                        1 => SYSTEM_PALLETE[0x23],
                        2 => SYSTEM_PALLETE[0x27],
                        3 => SYSTEM_PALLETE[0x30],
                        _ => panic!("can't be"),
                    };
                    frame.set_pixel(tile_x + x, tile_y + y, rgb)
                }
            }
        }

        frame
    }

    pub fn render(ppu: &Ppu, frame: &mut Frame) {
        // Clear frame with background color (palette index 0)
        let bg_color = SYSTEM_PALLETE[ppu.palette_table[0] as usize];
        frame.clear(bg_color);

        let scroll_x = (ppu.scroll.scroll_x) as usize;
        let scroll_y = (ppu.scroll.scroll_y) as usize;

        // Use the LATCHED nametable address (captured at VBlank start)
        let nametable_addr = ppu.latched_nametable_addr;

        // Debug output
        if scroll_x > 0 || nametable_addr != 0x2000 {
            println!("Scroll X: {}, Y: {}, Nametable: 0x{:04X}", scroll_x, scroll_y, nametable_addr);
        }

        // Select main and secondary nametables based on control register
        // The nametable address determines which bank is "current"
        let (main_nametable, second_nametable) = match ppu.mirroring {
            Mirroring::VERTICAL => {
                match nametable_addr {
                    0x2000 | 0x2800 => (&ppu.vram[0..0x400], &ppu.vram[0x400..0x800]),
                    0x2400 | 0x2C00 => (&ppu.vram[0x400..0x800], &ppu.vram[0..0x400]),
                    _ => (&ppu.vram[0..0x400], &ppu.vram[0x400..0x800]),
                }
            }
            Mirroring::HORIZONTAL => {
                match nametable_addr {
                    0x2000 | 0x2400 => (&ppu.vram[0..0x400], &ppu.vram[0x400..0x800]),
                    0x2800 | 0x2C00 => (&ppu.vram[0x400..0x800], &ppu.vram[0..0x400]),
                    _ => (&ppu.vram[0..0x400], &ppu.vram[0x400..0x800]),
                }
            }
        };

        // Render both nametables based on mirroring type
        match ppu.mirroring {
            Mirroring::VERTICAL => {
                // Render current nametable starting from scroll position
                render_name_table(
                    ppu,
                    frame,
                    main_nametable,
                    Rect::new(scroll_x, scroll_y, 256, 240),
                    -(scroll_x as isize),
                    -(scroll_y as isize),
                );

                // Render adjacent nametable to the right to fill the gap
                render_name_table(
                    ppu,
                    frame,
                    second_nametable,
                    Rect::new(0, scroll_y, 256, 240),
                    (256 - scroll_x) as isize,
                    -(scroll_y as isize),
                );
            }
            Mirroring::HORIZONTAL => {
                // Horizontal mirroring: two banks stacked for vertical scrolling
                let nametable_top = &ppu.vram[0..0x400];
                let nametable_bottom = &ppu.vram[0x400..0x800];

                let (nt_top_shift_y, nt_bottom_shift_y) = match nametable_addr {
                    0x2000 | 0x2400 => {
                        // Rendering from top nametable
                        (-(scroll_y as isize), (240 - scroll_y) as isize)
                    }
                    0x2800 | 0x2C00 => {
                        // Rendering from bottom nametable
                        (-(240 + scroll_y as isize), -(scroll_y as isize))
                    }
                    _ => (-(scroll_y as isize), (240 - scroll_y) as isize),
                };

                render_name_table(
                    ppu,
                    frame,
                    nametable_top,
                    Rect::new(scroll_x, 0, 256, 240),
                    -(scroll_x as isize),
                    nt_top_shift_y,
                );

                render_name_table(
                    ppu,
                    frame,
                    nametable_bottom,
                    Rect::new(scroll_x, 0, 256, 240),
                    -(scroll_x as isize),
                    nt_bottom_shift_y,
                );
            }
        }

        for i in (0..ppu.oam_data.len()).step_by(4).rev() {
            let tile_idx = ppu.oam_data[i + 1] as u16;
            let tile_x = ppu.oam_data[i + 3] as usize;
            let tile_y = ppu.oam_data[i] as usize;

            let flip_vertical = if ppu.oam_data[i + 2] >> 7 & 1 == 1 {
                true
            } else {
                false
            };
            let flip_horizontal = if ppu.oam_data[i + 2] >> 6 & 1 == 1 {
                true
            } else {
                false
            };
            let pallette_idx = ppu.oam_data[i + 2] & 0b11;
            let sprite_palette = sprite_palette(ppu, pallette_idx);
            let bank: u16 = ppu.ctrl.sprt_pattern_addr();

            // Check if CHR ROM has enough data for this sprite tile
            let sprite_tile_start = (bank + tile_idx * 16) as usize;
            let sprite_tile_end = sprite_tile_start + 15;
            if ppu.chr_rom.is_empty() || sprite_tile_end >= ppu.chr_rom.len() {
                continue; // Skip this sprite if CHR ROM doesn't have the data
            }

            let tile = &ppu.chr_rom[sprite_tile_start..=sprite_tile_end];

            for y in 0..=7 {
                let mut upper = tile[y];
                let mut lower = tile[y + 8];
                'ololo: for x in (0..=7).rev() {
                    let value = (1 & lower) << 1 | (1 & upper);
                    upper = upper >> 1;
                    lower = lower >> 1;
                    let rgb = match value {
                        0 => continue 'ololo, // skip coloring the pixel
                        1 => SYSTEM_PALLETE[sprite_palette[1] as usize],
                        2 => SYSTEM_PALLETE[sprite_palette[2] as usize],
                        3 => SYSTEM_PALLETE[sprite_palette[3] as usize],
                        _ => panic!("can't be"),
                    };
                    match (flip_horizontal, flip_vertical) {
                        (false, false) => {
                            frame.set_pixel(tile_x + x, tile_y + y, rgb);
                            // frame.set_pixel(tile_x + x, tile_y + y +250, rgb);
                        }
                        (true, false) => {
                            frame.set_pixel(tile_x + 7 - x, tile_y + y, rgb);
                            // frame.set_pixel(tile_x + 7 - x , tile_y + y + 250, rgb);
                        }
                        (false, true) => {
                            frame.set_pixel(tile_x + x, tile_y + 7 - y, rgb);
                            // frame.set_pixel(tile_x + x, tile_y + 7 - y + 250, rgb);
                        }
                        (true, true) => {
                            frame.set_pixel(tile_x + 7 - x, tile_y + 7 - y, rgb);
                            // frame.set_pixel(tile_x + 7 - x, tile_y + 7 - y+250, rgb);
                        }
                    }
                }
            }
        }
    }
}
