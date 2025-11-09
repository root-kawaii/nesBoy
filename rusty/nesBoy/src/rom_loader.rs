use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone)]
struct NESHeader {
    signature: [u8; 4], // Should be "NES" followed by MS-DOS EOF
    prg_rom_size: u8,   // PRG ROM size in 16KB units
    chr_rom_size: u8,   // CHR ROM size in 8KB units
    flags6: u8,         // Mapper, mirroring, battery, trainer
    flags7: u8,         // Mapper, VS/Playchoice, NES 2.0
    flags8: u8,         // PRG-RAM size (rarely used)
    flags9: u8,         // TV system (rarely used)
    flags10: u8,        // TV system, PRG-RAM (rarely used)
    padding: [u8; 5],   // Unused padding
}

pub struct RomLoader {
    header: NESHeader,
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
    mapper: u8,
    vertical_mirroring: bool,
    four_screen_mode: bool,
    has_battery: bool,
    has_trainer: bool,
}

impl RomLoader {
    /// Create a new RomLoader by loading a ROM file
    pub fn new(filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = File::open(filename)?;
        let mut header_bytes = [0u8; 16];
        file.read_exact(&mut header_bytes)?;

        // Validate the signature is "NES\x1A"
        if &header_bytes[0..3] != b"NES" || header_bytes[3] != 0x1A {
            return Err("Invalid NES header signature".into());
        }

        let header = NESHeader {
            signature: [
                header_bytes[0],
                header_bytes[1],
                header_bytes[2],
                header_bytes[3],
            ],
            prg_rom_size: header_bytes[4],
            chr_rom_size: header_bytes[5],
            flags6: header_bytes[6],
            flags7: header_bytes[7],
            flags8: header_bytes[8],
            flags9: header_bytes[9],
            flags10: header_bytes[10],
            padding: [
                header_bytes[11],
                header_bytes[12],
                header_bytes[13],
                header_bytes[14],
                header_bytes[15],
            ],
        };

        // Extract flags
        let mapper = (header.flags7 & 0xF0) | ((header.flags6 & 0xF0) >> 4);
        let vertical_mirroring = (header.flags6 & 0x01) != 0;
        let four_screen_mode = (header.flags6 & 0x08) != 0;
        let has_battery = (header.flags6 & 0x02) != 0;
        let has_trainer = (header.flags6 & 0x04) != 0;

        // Skip trainer if present (512 bytes)
        if has_trainer {
            file.seek(std::io::SeekFrom::Current(512))?;
        }

        // Read PRG ROM (16KB units)
        let prg_rom_bytes = header.prg_rom_size as usize * 16384;
        let mut prg_rom = vec![0u8; prg_rom_bytes];
        file.read_exact(&mut prg_rom)?;

        // Read CHR ROM (8KB units) if present
        let mut chr_rom = Vec::new();
        if header.chr_rom_size > 0 {
            let chr_rom_bytes = header.chr_rom_size as usize * 8192;
            chr_rom.resize(chr_rom_bytes, 0);
            file.read_exact(&mut chr_rom)?;
        }

        Ok(RomLoader {
            header,
            prg_rom,
            chr_rom,
            mapper,
            vertical_mirroring,
            four_screen_mode,
            has_battery,
            has_trainer,
        })
    }

    /// Print ROM information
    pub fn print_info(&self) {
        println!("ROM loaded successfully:");
        println!(
            "  PRG ROM Size: {} KB",
            self.header.prg_rom_size as u32 * 16
        );
        println!("  CHR ROM Size: {} KB", self.header.chr_rom_size as u32 * 8);
        println!("  Mapper Number: {}", self.mapper);
        println!(
            "  Mirroring: {}",
            if self.vertical_mirroring {
                "Vertical"
            } else {
                "Horizontal"
            }
        );
        println!(
            "  Four-screen VRAM: {}",
            if self.four_screen_mode { "Yes" } else { "No" }
        );
        println!(
            "  Battery-backed RAM: {}",
            if self.has_battery { "Yes" } else { "No" }
        );
        println!(
            "  Trainer Present: {}",
            if self.has_trainer { "Yes" } else { "No" }
        );
    }

    // Getters
    pub fn header(&self) -> &NESHeader {
        &self.header
    }

    pub fn prg_rom(&self) -> &[u8] {
        &self.prg_rom
    }

    pub fn chr_rom(&self) -> &[u8] {
        &self.chr_rom
    }

    pub fn mapper(&self) -> u8 {
        self.mapper
    }

    pub fn vertical_mirroring(&self) -> bool {
        self.vertical_mirroring
    }

    pub fn four_screen_mode(&self) -> bool {
        self.four_screen_mode
    }

    pub fn has_battery(&self) -> bool {
        self.has_battery
    }

    pub fn has_trainer(&self) -> bool {
        self.has_trainer
    }
}

// Example usage
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rom = RomLoader::new("path/to/game.nes")?;
    rom.print_info();

    // Access ROM data
    println!("PRG ROM size: {} bytes", rom.prg_rom().len());
    println!("CHR ROM size: {} bytes", rom.chr_rom().len());

    Ok(())
}
