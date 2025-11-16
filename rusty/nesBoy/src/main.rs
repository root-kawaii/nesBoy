mod add_register;
mod bus;
mod controller_register;
mod cpu;
mod ppu;
mod rom_loader;
mod status;
mod frame;
mod joypad;


use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;

use crate::bus::Bus;
use crate::frame::Frame;
use crate::ppu::Ppu;
use crate::rom_loader::RomLoader;
use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

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
        .build()
        .unwrap();
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, 256, 240)
        .unwrap();
    let mut event_pump = sdl.event_pump().unwrap();

    let mut frame = Frame::new();

    let mut key_map = HashMap::new();
        key_map.insert(Keycode::Down, joypad::JoypadButton::DOWN);
        key_map.insert(Keycode::Up, joypad::JoypadButton::UP);
        key_map.insert(Keycode::Right, joypad::JoypadButton::RIGHT);
        key_map.insert(Keycode::Left, joypad::JoypadButton::LEFT);
        key_map.insert(Keycode::Space, joypad::JoypadButton::SELECT);
        key_map.insert(Keycode::Return, joypad::JoypadButton::START);
        key_map.insert(Keycode::A, joypad::JoypadButton::BUTTON_A);
        key_map.insert(Keycode::S, joypad::JoypadButton::BUTTON_B);

    // load the game
    let rom = RomLoader::new("pac.nes").unwrap();

    // the game cycle
    let bus = Bus::new(rom, move |ppu: &Ppu, joypad: &mut joypad::Joypad|  {
        Frame::render(ppu, &mut frame);
        texture.update(None, &frame.data, 256 * 3).unwrap();

        canvas.copy(&texture, None, None).unwrap();
        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => std::process::exit(0),
                Event::KeyDown { keycode, .. } => {
                    if let Some(key) = key_map.get(&keycode.unwrap_or(Keycode::Ampersand)) {
                        joypad.set_button_pressed_status(*key, true);
                    }
                }
                Event::KeyUp { keycode, .. } => {
                    if let Some(key) = key_map.get(&keycode.unwrap_or(Keycode::Ampersand)) {
                        joypad.set_button_pressed_status(*key, false);
                    }
                }
                _ => { /* do nothing */ }
            }
        }
    });

    let mut cpu = cpu::Cpu::new(bus);

    cpu.reset();
    cpu.run();
}
