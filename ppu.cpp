#include "ppu.h"
#include <iostream>

PPU::PPU() {
    // Initialize registers to default values
    control = 0x00;
    mask = 0x00;
    status = 0x00;
    oam_addr = 0x00;
    oam_data = 0x00;
    vram.fill(0);  // Fill VRAM with 0
}

uint8_t PPU::read(uint16_t addr) {
    if (addr >= 0x2000 && addr <= 0x3FFF) {
        // Reading from the PPU memory (VRAM)
        return vram[addr - 0x2000];
    }

    // Handle register reads (status, mask, control, etc.)
    if (addr == 0x2000) return readControl(addr);
    if (addr == 0x2001) return readMask(addr);
    if (addr == 0x2002) return readStatus(addr);
    if (addr == 0x2003) return readOAM(addr);

    std::cerr << "Unknown PPU read address: " << std::hex << addr << std::endl;
    return 0x00;
}

void PPU::write(uint16_t addr, uint8_t data) {
    if (addr >= 0x2000 && addr <= 0x3FFF) {
        // Writing to the PPU memory (VRAM)
        vram[addr - 0x2000] = data;
        return;
    }

    // Handle register writes (status, mask, control, etc.)
    if (addr == 0x2000) {
        writeControl(addr, data);
    } else if (addr == 0x2001) {
        writeMask(addr, data);
    } else if (addr == 0x2003) {
        writeOAM(addr, data);
    }

    // Other control registers can be added here
}

uint8_t PPU::readControl(uint16_t addr) {
    // Control register (e.g., for enabling rendering)
    return control;
}

uint8_t PPU::readMask(uint16_t addr) {
    // Mask register (e.g., for adjusting sprite rendering settings)
    return mask;
}

uint8_t PPU::readStatus(uint16_t addr) {
    // Status register (read-only by CPU, returns the current PPU status)
    return status;
}

uint8_t PPU::readOAM(uint16_t addr) {
    // Object Attribute Memory (OAM) for sprite data
    return oam_data;
}

void PPU::writeControl(uint16_t addr, uint8_t data) {
    // Set control registers (enabling rendering, etc.)
    control = data;
}

void PPU::writeMask(uint16_t addr, uint8_t data) {
    // Set mask registers (e.g., to enable or disable rendering)
    mask = data;
}

void PPU::writeOAM(uint16_t addr, uint8_t data) {
    // Write sprite data into OAM (Object Attribute Memory)
    oam_data = data;
}

void PPU::setStatusFlag(bool vblank) {
    if (vblank)
        status |= 0x80; // Set VBlank bit
    else
        status &= ~0x80;
}

void PPU::fetchTileID() {
    uint16_t baseAddr = 0x2000; // For now, use nametable 0
    uint16_t ntIndex = ((scanline / 8) * 32) + ((cycle - 1) / 8);
    tileID = read(baseAddr + ntIndex);
}

void PPU::fetchTileLSB() {
    uint16_t patternBase = 0x0000; // For now, fixed
    uint16_t tileAddr = patternBase + (tileID * 16) + (scanline % 8);
    tileLSB = read(tileAddr);
}

void PPU::fetchTileMSB() {
    uint16_t patternBase = 0x0000;
    uint16_t tileAddr = patternBase + (tileID * 16) + (scanline % 8) + 8;
    tileMSB = read(tileAddr);
}

void PPU::fetchAttribute() {
    // Calculate position in attribute table
    int tileX = (cycle - 1) / 8;
    int tileY = scanline / 8;
    
    // Each attribute byte covers a 4x4 tile area (32x32 pixels)
    int attrX = tileX / 4;
    int attrY = tileY / 4;
    
    // Attribute table starts at nametable + 0x3C0
    uint16_t attrAddr = 0x2000 + 0x3C0 + (attrY * 8) + attrX;
    uint8_t attrByte = read(attrAddr);
    
    // Each attribute byte contains 4 2-bit palette selections
    // Determine which 2x2 tile quadrant we're in
    int quadrantX = (tileX % 4) / 2;
    int quadrantY = (tileY % 4) / 2;
    int shift = (quadrantY * 4) + (quadrantX * 2);
    
    tileAttrib = (attrByte >> shift) & 0x03;
}

void PPU::step() {
    // Visible scanlines (0-239)
    if (scanline >= 0 && scanline <= 239) {
        if (cycle >= 1 && cycle <= 256) {
            // Background rendering - fetch tile data every 8 cycles
            if ((cycle - 1) % 8 == 0) {
                fetchTileID();
            } else if ((cycle - 1) % 8 == 2) {
                fetchAttribute();
            } else if ((cycle - 1) % 8 == 4) {
                fetchTileLSB();
            } else if ((cycle - 1) % 8 == 6) {
                fetchTileMSB();
                // After fetching all data, render the tile
                if (mask & 0x08) { // Check if background rendering is enabled
                    renderTile();
                }
            }
            renderSprites();
        }
        
        // Sprite evaluation for next scanline happens during cycles 257-320
        if (cycle >= 257 && cycle <= 320) {
            spriteEvaluation();
            // TODO: Implement sprite evaluation
        }
    }
    // Post-render scanline (240) - do nothing
    else if (scanline == 240) {
        // Idle scanline
    }
    // VBlank scanlines (241-260)
    else if (scanline == 241 && cycle == 1) {
        // Enter VBlank
        setStatusFlag(true);
        // TODO: Trigger NMI if enabled in control register
        if (control & 0x80) {
            // nmi_pending = true; // Signal NMI to CPU
        }
    }
    // Pre-render scanline (261)
    else if (scanline == 261) {
        if (cycle == 1) {
            // Clear VBlank flag
            setStatusFlag(false);
            // Clear sprite 0 hit and sprite overflow flags
            status &= ~0x60;
        }
        
        // Pre-render scanline does background fetches like visible scanlines
        if (cycle >= 1 && cycle <= 256) {
            if ((cycle - 1) % 8 == 0) {
                fetchTileID();
            } else if ((cycle - 1) % 8 == 2) {
                fetchAttribute();
            } else if ((cycle - 1) % 8 == 4) {
                fetchTileLSB();
            } else if ((cycle - 1) % 8 == 6) {
                fetchTileMSB();
            }
        }
    }

    // Advance timing
    cycle++;
    if (cycle >= 341) {
        cycle = 0;
        scanline++;
        if (scanline >= 262) {
            scanline = 0;
            frame_complete = true;
        }
    }
}



void PPU::spriteEvaluation() {
    // Clear secondary OAM at start of evaluation
    if (cycle == 257) {
        sprites_found = 0;
        sprite_overflow = false;
        std::fill(secondary_oam, secondary_oam + 32, 0xFF); // Fill with invalid sprite data
    }
    
    // Sprite evaluation happens over cycles 257-320 (64 cycles for 64 sprites)
    if (cycle >= 257 && cycle <= 320) {
        int sprite_index = cycle - 257; // 0-63
        
        if (sprite_index < 64 && sprites_found < 8) {
            // Read sprite Y position from OAM
            uint8_t sprite_y = oam[sprite_index * 4];
            
            // Determine sprite height (8x8 or 8x16 based on control register)
            int sprite_height = (control & 0x20) ? 16 : 8;
            
            // Check if sprite is visible on NEXT scanline (scanline + 1)
            int next_scanline = (scanline + 1) % 262;
            
            // Sprite is visible if next scanline is within sprite's Y range
            if (next_scanline >= sprite_y && next_scanline < (sprite_y + sprite_height)) {
                // Copy entire sprite data (4 bytes) to secondary OAM
                secondary_oam[sprites_found * 4 + 0] = oam[sprite_index * 4 + 0]; // Y position
                secondary_oam[sprites_found * 4 + 1] = oam[sprite_index * 4 + 1]; // Tile ID
                secondary_oam[sprites_found * 4 + 2] = oam[sprite_index * 4 + 2]; // Attributes
                secondary_oam[sprites_found * 4 + 3] = oam[sprite_index * 4 + 3]; // X position
                
                sprites_found++;
                
                // Check for sprite 0 (important for games that use sprite 0 hit)
                if (sprite_index == 0) {
                    sprite_0_in_range = true;
                }
            }
        }
        else if (sprites_found >= 8) {
            // Hardware bug: Continue checking remaining sprites for overflow
            // This is simplified - real hardware has a more complex bug here
            uint8_t sprite_y = oam[sprite_index * 4];
            int sprite_height = (control & 0x20) ? 16 : 8;
            int next_scanline = (scanline + 1) % 262;
            
            if (next_scanline >= sprite_y && next_scanline < (sprite_y + sprite_height)) {
                sprite_overflow = true;
                status |= 0x20; // Set sprite overflow flag in status register
            }
        }
    }
}

void PPU::fetchSpriteData() {
    // This happens during cycles 321-340 (and some during 257-320)
    // Fetch pattern data for the sprites found in secondary OAM
    
    if (cycle >= 321 && cycle <= 340) {
        int fetch_cycle = (cycle - 321) / 8; // Which sprite we're fetching (0-7)
        int fetch_step = (cycle - 321) % 8;  // Which step of the fetch
        
        if (fetch_cycle < sprites_found) {
            uint8_t sprite_y = secondary_oam[fetch_cycle * 4 + 0];
            uint8_t tile_id = secondary_oam[fetch_cycle * 4 + 1];
            uint8_t attributes = secondary_oam[fetch_cycle * 4 + 2];
            uint8_t sprite_x = secondary_oam[fetch_cycle * 4 + 3];
            
            // Calculate which row of the sprite we need for next scanline
            int sprite_row = ((scanline + 1) - sprite_y) % 16;
            
            // Handle vertical flipping
            if (attributes & 0x80) {
                int sprite_height = (control & 0x20) ? 16 : 8;
                sprite_row = (sprite_height - 1) - sprite_row;
            }
            
            switch (fetch_step) {
                case 0: case 1:
                    // Fetch tile ID (already have it)
                    break;
                case 2: case 3:
                    // Fetch attributes (already have them)
                    break;
                case 4: case 5: {
                    // Fetch low bit plane
                    uint16_t pattern_addr;
                    if (control & 0x20) {
                        // 8x16 sprites - tile ID determines pattern table
                        pattern_addr = ((tile_id & 0x01) ? 0x1000 : 0x0000) + 
                                      ((tile_id & 0xFE) * 16) + sprite_row;
                    } else {
                        // 8x8 sprites - control register determines pattern table
                        pattern_addr = ((control & 0x08) ? 0x1000 : 0x0000) + 
                                      (tile_id * 16) + sprite_row;
                    }
                    sprite_pattern_low[fetch_cycle] = read(pattern_addr);
                    break;
                }
                case 6: case 7: {
                    // Fetch high bit plane
                    uint16_t pattern_addr;
                    if (control & 0x20) {
                        pattern_addr = ((tile_id & 0x01) ? 0x1000 : 0x0000) + 
                                      ((tile_id & 0xFE) * 16) + sprite_row + 8;
                    } else {
                        pattern_addr = ((control & 0x08) ? 0x1000 : 0x0000) + 
                                      (tile_id * 16) + sprite_row + 8;
                    }
                    sprite_pattern_high[fetch_cycle] = read(pattern_addr);
                    break;
                }
            }
        }
    }
}

void PPU::renderSprites() {
    // Render sprites during visible pixel output (cycles 1-256)
    if (cycle >= 1 && cycle <= 256 && (mask & 0x10)) { // Check if sprite rendering enabled
        int pixel_x = cycle - 1;
        
        // Check all sprites in secondary OAM (back to front for priority)
        for (int i = sprites_found - 1; i >= 0; i--) {
            uint8_t sprite_x = secondary_oam[i * 4 + 3];
            uint8_t attributes = secondary_oam[i * 4 + 2];
            
            // Check if current pixel is within this sprite's X range
            if (pixel_x >= sprite_x && pixel_x < (sprite_x + 8)) {
                int sprite_pixel = pixel_x - sprite_x;
                
                // Handle horizontal flipping
                if (attributes & 0x40) {
                    sprite_pixel = 7 - sprite_pixel;
                }
                
                // Get color from pattern data
                uint8_t bit0 = (sprite_pattern_low[i] >> (7 - sprite_pixel)) & 1;
                uint8_t bit1 = (sprite_pattern_high[i] >> (7 - sprite_pixel)) & 1;
                uint8_t color_index = (bit1 << 1) | bit0;
                
                // Skip transparent pixels
                if (color_index != 0) {
                    // Check sprite priority (bit 5 of attributes)
                    bool behind_background = attributes & 0x20;
                    uint8_t bg_pixel = framebuffer[scanline * 256 + pixel_x];
                    
                    // Sprite 0 hit detection
                    if (i == 0 && bg_pixel != 0 && color_index != 0) {
                        status |= 0x40; // Set sprite 0 hit flag
                    }
                    
                    // Draw sprite pixel if it should be visible
                    if (!behind_background || bg_pixel == 0) {
                        // Add palette offset for sprites (palette 4-7)
                        uint8_t palette = (attributes & 0x03) + 4;
                        framebuffer[scanline * 256 + pixel_x] = (palette << 2) | color_index;
                    }
                    
                    break; // First opaque sprite wins
                }
            }
        }
    }
}
