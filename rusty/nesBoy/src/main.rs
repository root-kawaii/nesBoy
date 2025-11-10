mod add_register;
mod bus;
mod controller_register;
mod cpu;
mod ppu;
mod rom_loader;

use sdl2::TimerSubsystem;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;

use bus::Bus;
use cpu::Cpu;
use ppu::Ppu;
use rom_loader::RomLoader;

static NES_WIDTH: u64 = 256;
static NES_HEIGHT: u64 = 240;
static SCALE: u64 = 3;
static FPS: u64 = 60;
static FRAME_DELAY: u32 = 1000 / FPS as u32;

static NES_PALETTE: [u32; 4] = [
    0xFF7C7C7C, // gray
    0xFF0000FF, // red
    0xFF00FF00, // green
    0xFFFF0000, // blue
];

fn main() {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let window = video.window("NES", 256 * 3, 240 * 3).build().unwrap();
    let mut canvas = window
        .into_canvas()
        // .index(find_sdl_gl_driver().unwrap())
        .build()
        .unwrap();
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, 256, 240)
        .unwrap();
    let timer = sdl.timer().unwrap();

    let mut running = true;
    let mut colorTimer: u32 = 0;
    let mut color_index = 0;

    // let mut rom_loader = RomLoader::new("ff.nes").unwrap();
    // println!("Loaded ROM");
    // rom_loader.print_info();

    let mut bus = bus::Bus::new();
    let mut cpu = cpu::Cpu::new(bus);

    while running {
        let frameStart = TimerSubsystem::ticks(&timer);

        // --- Emulate one frame ---
        // Commented out for now until CPU/PPU are ready
        // while (!bus.ppu->isFrameComplete()) {
        cpu.step();
        // }
        // bus.ppu->resetFrameComplete();

        // --- Convert PPU framebuffer indices to actual pixels ---

        texture
            .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                let color = NES_PALETTE[color_index % 4];
                let r = ((color >> 16) & 0xFF) as u8;
                let g = ((color >> 8) & 0xFF) as u8;
                let b = (color & 0xFF) as u8;

                // Fill each pixel (4 bytes per pixel)
                for pixel in buffer.chunks_exact_mut(4) {
                    pixel[0] = r; // Red
                    pixel[1] = g; // Green
                    pixel[2] = b; // Blue
                    pixel[3] = 255; // Alpha
                }
            })
            .unwrap();

        canvas.clear();
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();

        for event in sdl.event_pump().unwrap().poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => running = false,
                _ => {}
            }
        }

        let frameTime = TimerSubsystem::ticks(&timer) - frameStart;
        if FRAME_DELAY > frameTime {
            timer.delay(FRAME_DELAY - frameTime);
        }

        // // Update color every 2 seconds
        colorTimer += frameTime;
        if (colorTimer >= 200) {
            color_index += 1;
            colorTimer = 0;
            println!("Changed color to index: {}", color_index % 4);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::bus::Bus;

    #[test]
    fn test_format_trace() {
        let mut bus = Bus::new();
        bus.write(100, 0xa2);
        bus.write(101, 0x01);
        bus.write(102, 0xca);
        bus.write(103, 0x88);
        bus.write(104, 0x00);

        let mut cpu = Cpu::new(bus);
        cpu.pc = 0x64;
        cpu.a = 1;
        cpu.x = 2;
        cpu.y = 3;
        let mut result: Vec<String> = vec![];
        cpu.run_with_callback(|cpu| {
            result.push(trace(cpu));
        });
        assert_eq!(
            "0064  A2 01     LDX #$01                        A:01 X:02 Y:03 P:24 SP:FD",
            result[0]
        );
        assert_eq!(
            "0066  CA        DEX                             A:01 X:01 Y:03 P:24 SP:FD",
            result[1]
        );
        assert_eq!(
            "0067  88        DEY                             A:01 X:00 Y:03 P:26 SP:FD",
            result[2]
        );
    }

    #[test]
    fn test_format_mem_access() {
        let mut bus = Bus::new();
        // ORA ($33), Y
        bus.write(100, 0x11);
        bus.write(101, 0x33);

        //data
        bus.write(0x33, 00);
        bus.write(0x34, 04);

        //target cell
        bus.write(0x400, 0xAA);

        let mut cpu = Cpu::new(bus);
        cpu.pc = 0x64;
        cpu.y = 0;
        let mut result: Vec<String> = vec![];
        cpu.run_with_callback(|cpu| {
            result.push(trace(cpu));
        });
        assert_eq!(
            "0064  11 33     ORA ($33),Y = 0400 @ 0400 = AA  A:00 X:00 Y:00 P:24 SP:FD",
            result[0]
        );
    }
}
