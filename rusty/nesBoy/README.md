# nesBoy

A Nintendo Entertainment System (NES) emulator written in Rust from scratch. This project is a work-in-progress implementation of the NES/Famicom hardware, focusing on accuracy and understanding the low-level details of the system.

## Project Status

🚧 **Work in Progress** - Currently implementing core components

### Implemented ✅

- **ROM Loader**
  - iNES format parser
  - Support for PRG-ROM and CHR-ROM loading
  - Mapper detection
  - Mirroring mode detection (horizontal/vertical)
  - Trainer and battery-backed RAM detection

- **CPU (6502)**
  - ~112 opcodes implemented
  - All major instruction categories:
    - Load/Store (LDA, LDX, LDY, STA, STX, STY)
    - Arithmetic (ADC, SBC, INC, DEC)
    - Logical (AND, ORA, EOR)
    - Shifts/Rotates (ASL, LSR, ROL, ROR)
    - Comparisons (CMP, CPX, CPY, BIT)
    - Branches (BEQ, BNE, BCS, BCC, BMI, BPL, BVS, BVC)
    - Stack operations (PHA, PLA, PHP, PLP)
    - Transfers (TAX, TAY, TXA, TYA, TSX, TXS)
    - System (BRK, RTI, RTS, NOP, JMP, JSR)
  - All addressing modes
  - Status flag management

- **PPU (Picture Processing Unit)**
  - Basic PPU registers (control, mask, status, OAM, scroll, addr, data)
  - Address register with mirroring
  - Control register with bitflags
  - VRAM address space management (0x0000-0x3FFF)
  - Nametable mirroring (horizontal/vertical)
  - Scanline and cycle tracking
  - Frame completion detection
  - Palette table support
  - Basic tile rendering structure

- **Memory Bus**
  - CPU memory mapping (0x0000-0xFFFF)
  - PPU register mapping (0x2000-0x3FFF with mirroring)
  - PRG-ROM mapping (0x8000-0xFFFF)
  - PRG-RAM mapping (0x0000-0x1FFF with mirroring)
  - PPU register writes (control, addr, data)

- **Infrastructure**
  - SDL2 integration for graphics output
  - Main loop structure with timing

### In Progress 🔨

- **PPU Rendering**
  - Background rendering
  - Sprite rendering
  - Scrolling
  - PPU memory writes completion
  - Full register implementation

### TODO 📋

- **CPU**
  - Remaining unofficial opcodes
  - Cycle-accurate timing
  - IRQ/NMI interrupt handling refinement

- **PPU**
  - Complete background rendering pipeline
  - Sprite rendering and sprite 0 hit
  - Fine scrolling
  - PPU status register updates
  - OAM (Object Attribute Memory) implementation

- **APU (Audio Processing Unit)**
  - Pulse channels
  - Triangle channel
  - Noise channel
  - DMC channel
  - Mixer

- **Input**
  - Controller input handling
  - Joypad registers

- **Mappers**
  - Mapper 0 (NROM)
  - Additional mappers (1, 2, 3, 4, etc.)

- **Features**
  - Save states
  - Debugging tools
  - ROM compatibility testing
  - Performance optimization

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
├── main.rs              # Entry point, SDL2 setup, main loop
├── cpu.rs               # 6502 CPU implementation
├── ppu.rs               # PPU (Picture Processing Unit)
├── bus.rs               # Memory bus and address mapping
├── rom_loader.rs        # iNES ROM file parser
├── controller_register.rs  # PPU control register
└── add_register.rs      # PPU address register
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
