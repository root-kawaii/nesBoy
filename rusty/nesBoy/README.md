# nesBoy

A Nintendo Entertainment System (NES) emulator written in Rust from scratch. This project is a work-in-progress implementation of the NES/Famicom hardware, focusing on accuracy and understanding the low-level details of the system.

## Current Status

🎮 **Playable** - Super Mario Bros and other NROM (Mapper 0) games are fully playable with smooth scrolling!

## Recent Improvements

### Scrolling Fix (Latest)
Fixed choppy scrolling and screen transition flickering issues:
- Implemented **scroll value latching** - Scroll register writes are stored temporarily and latched at VBlank start (scanline 241)
- Implemented **nametable address latching** - Nametable selection is captured at VBlank to prevent mid-frame changes
- **Result**: Smooth horizontal scrolling throughout entire levels, no more Goomba speed issues or flickering between screens

### Previous Work
- Implemented full background rendering pipeline with proper tile and attribute fetching
- Fixed sprite rendering and sprite 0 hit detection
- Implemented proper PPU timing and scanline rendering
- Added joypad input support with SDL2 keyboard mapping
- Fixed numerous PPU register behaviors and edge cases

## Implemented Features ✅

### **CPU (6502)**
- ✅ All official opcodes (~151 instructions)
- ✅ All addressing modes (immediate, zero page, absolute, indexed, indirect, etc.)
- ✅ Status flag management (N, V, B, D, I, Z, C)
- ✅ Stack operations
- ✅ Interrupt handling (NMI, IRQ, BRK)
- ✅ Decimal mode flag support
- ✅ Cycle counting and timing

### **PPU (Picture Processing Unit)**
- ✅ All PPU registers properly implemented:
  - PPUCTRL (0x2000) - Control register with nametable selection, VRAM increment, sprite tables
  - PPUMASK (0x2001) - Mask register for rendering control
  - PPUSTATUS (0x2002) - Status register with VBlank, sprite 0 hit, sprite overflow
  - OAMADDR (0x2003) - OAM address register
  - OAMDATA (0x2004) - OAM data read/write
  - PPUSCROLL (0x2005) - Scroll position with proper latching
  - PPUADDR (0x2006) - VRAM address register
  - PPUDATA (0x2007) - VRAM data read/write with buffering
- ✅ **Background Rendering**
  - Pattern table (tile graphics) rendering
  - Nametable rendering with proper addressing
  - Attribute table support for color palettes
  - Horizontal and vertical scrolling with latching
  - Vertical and horizontal mirroring modes
- ✅ **Sprite Rendering**
  - 8x8 and 8x16 sprite modes
  - Sprite priority (behind/in front of background)
  - Sprite 0 hit detection
  - Up to 64 sprites in OAM
  - Proper sprite evaluation and rendering
- ✅ **Timing & Synchronization**
  - Scanline and cycle accurate timing (341 PPU cycles per scanline)
  - VBlank generation and NMI triggering
  - Frame timing (262 scanlines per frame)
  - Proper scroll and nametable latching at VBlank start
- ✅ **Memory Management**
  - VRAM (0x0000-0x3FFF) with proper mirroring
  - Palette RAM (0x3F00-0x3F1F) with mirroring
  - OAM (Object Attribute Memory) 256 bytes
  - CHR-ROM/RAM access

### **Memory Bus**
- ✅ CPU address space (0x0000-0xFFFF)
  - Internal RAM (0x0000-0x07FF) with mirroring to 0x1FFF
  - PPU registers (0x2000-0x2007) with mirroring to 0x3FFF
  - APU and I/O registers (0x4000-0x4017)
  - Cartridge space (0x4020-0xFFFF)
- ✅ PPU address space (0x0000-0x3FFF)
  - Pattern tables (0x0000-0x1FFF)
  - Nametables (0x2000-0x2FFF) with mirroring
  - Palettes (0x3F00-0x3FFF) with mirroring

### **Input**
- ✅ Joypad support (standard NES controller)
  - D-pad (Up, Down, Left, Right)
  - Buttons (A, B, Select, Start)
  - Proper strobe and read behavior
  - SDL2 keyboard mapping:
    - Arrow keys → D-pad
    - A → Button A
    - S → Button B
    - Space → Select
    - Enter → Start

### **ROM Loader**
- ✅ iNES format parser (.nes files)
- ✅ Header parsing (PRG-ROM size, CHR-ROM size, mapper number)
- ✅ PRG-ROM loading (program code)
- ✅ CHR-ROM loading (graphics data)
- ✅ Mirroring mode detection (horizontal/vertical)
- ✅ Trainer detection
- ✅ Battery-backed RAM detection

### **Mappers**
- ✅ **Mapper 0 (NROM)** - Fully implemented
  - 16KB or 32KB PRG-ROM
  - 8KB CHR-ROM or CHR-RAM
  - Fixed mirroring

### **Rendering & Display**
- ✅ SDL2 integration for window and graphics
- ✅ 256x240 resolution with 3x scaling (768x720 window)
- ✅ 60 FPS rendering with proper frame timing
- ✅ RGB24 pixel format rendering
- ✅ Full color palette support (NES standard palette)

### **Infrastructure**
- ✅ Main emulation loop with proper timing
- ✅ CPU-PPU synchronization (3 PPU cycles per CPU cycle)
- ✅ Frame completion callbacks
- ✅ Event handling (keyboard input, window close)

## Missing Features 📋

### **APU (Audio Processing Unit)** 🔇
The emulator currently has **no audio support**. To add sound:
- ❌ Pulse channel 1 (square wave with sweep)
- ❌ Pulse channel 2 (square wave)
- ❌ Triangle channel (triangle wave)
- ❌ Noise channel (pseudo-random noise)
- ❌ DMC channel (delta modulation channel)
- ❌ Frame counter and timing
- ❌ Audio mixer
- ❌ SDL2 audio output integration
- ❌ APU registers (0x4000-0x4017)

### **Mappers** 🗺️
Only Mapper 0 (NROM) is supported. Many popular games require other mappers:

- ❌ **Mapper 1 (MMC1)** - Required for:
  - The Legend of Zelda
  - Zelda II: The Adventure of Link
  - Final Fantasy
  - Metroid
  - Kid Icarus
  - Mega Man 2
  - Castlevania II: Simon's Quest

- ❌ **Mapper 2 (UxROM)** - Required for:
  - Mega Man
  - Castlevania
  - Contra
  - Duck Tales

- ❌ **Mapper 3 (CNROM)** - Required for:
  - Arkanoid
  - Cybernoid
  - Solomon's Key

- ❌ **Mapper 4 (MMC3)** - Required for:
  - Super Mario Bros 2 & 3
  - Kirby's Adventure
  - Mega Man 3-6
  - Ninja Gaiden 1-3

### **Other Missing Features**
- ❌ Save states (save/load emulator state)
- ❌ Battery-backed save RAM persistence
- ❌ Debugger (CPU/PPU state inspection, breakpoints)
- ❌ Game Genie cheat code support
- ❌ Zapper (light gun) support
- ❌ PAL/NTSC region selection
- ❌ Configurable key bindings
- ❌ GUI for ROM selection
- ❌ Performance statistics/FPS counter

## Compatible Games 🎮

### Currently Working (Mapper 0)
- ✅ Super Mario Bros
- ✅ Donkey Kong
- ✅ Ice Climber
- ✅ Excitebike
- ✅ Balloon Fight
- ✅ Pac-Man (NES version)

### Needs Mapper Support
- ⏳ The Legend of Zelda (Mapper 1 - MMC1)
- ⏳ Final Fantasy (Mapper 1 - MMC1)
- ⏳ Metroid (Mapper 1 - MMC1)
- ⏳ Mega Man 2 (Mapper 1 - MMC1)
- ⏳ Castlevania (Mapper 2 - UxROM)
- ⏳ Contra (Mapper 2 - UxROM)
- ⏳ Super Mario Bros 3 (Mapper 4 - MMC3)
- ⏳ Mega Man 3-6 (Mapper 4 - MMC3)

## Architecture

```
┌─────────────────────────────────────────┐
│               Main Loop                 │
│  (SDL2 window, timing, input)          │
└──────────────┬──────────────────────────┘
               │
       ┌───────▼────────┐
       │      CPU       │
       │   (6502)       │
       └───────┬────────┘
               │
       ┌───────▼────────┐
       │      Bus       │
       │  (Memory Map)  │
       └───┬────────┬───┘
           │        │
   ┌───────▼──┐  ┌──▼──────────┐
   │   PPU    │  │  ROM Loader │
   │  (2C02)  │  │   (iNES)    │
   └──────────┘  └─────────────┘
```

## Building

### Prerequisites

- Rust (latest stable version)
- SDL2 development libraries

#### Installing SDL2

**macOS:**
```bash
brew install sdl2
```

**Ubuntu/Debian:**
```bash
sudo apt-get install libsdl2-dev
```

**Windows:**
Download SDL2 development libraries from [libsdl.org](https://www.libsdl.org/download-2.0.php)

### Build and Run

```bash
# Build the project
cargo build --release

# Run with a ROM file
cargo run --release
```

## Project Structure

```
src/
├── main.rs                 # Entry point, SDL2 setup, main loop
├── cpu.rs                  # 6502 CPU implementation with all opcodes
├── ppu.rs                  # PPU (Picture Processing Unit) with rendering
├── bus.rs                  # Memory bus and address mapping
├── rom_loader.rs           # iNES ROM file parser
├── frame.rs                # Frame buffer and rendering logic
├── joypad.rs               # Joypad/controller input handling
├── controller_register.rs  # PPU control register (PPUCTRL)
├── add_register.rs         # PPU address register (PPUADDR)
├── status.rs               # PPU status register (PPUSTATUS)
├── mask.rs                 # PPU mask register (PPUMASK)
├── scroll.rs               # PPU scroll register with latching (PPUSCROLL)
└── mirroring.rs            # Nametable mirroring modes
```

## Resources

This emulator is being built with reference to:
- [NESdev Wiki](https://wiki.nesdev.com/)
- [6502 Reference](http://www.6502.org/tutorials/6502opcodes.html)
- [NES Emulator Book](https://bugzmanov.github.io/nes_ebook/)

## Testing

Currently uses `nestest.nes` for CPU validation.

## License

This is an educational project. Feel free to learn from it and use it as reference.

## Contributing

This is a personal learning project, but suggestions and feedback are welcome!
