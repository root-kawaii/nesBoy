mod bus;
// mod cpu;
mod ppu;
mod rom_loader;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::TimerSubsystem;

use bus::Bus;
// use cpu::Cpu;
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

    let mut rom_loader = RomLoader::new("ff.nes").unwrap();
    println!("Loaded ROM");
    rom_loader.print_info();

    let mut bus = bus::Bus::new();
    // let mut cpu = cpu::Cpu::new();

    while running {
        let frameStart = TimerSubsystem::ticks(&timer);

        // --- Emulate one frame ---
        // Commented out for now until CPU/PPU are ready
        // while (!bus.ppu->isFrameComplete()) {
        //     bus.cpu->step();
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
