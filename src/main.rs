use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::path::Path;
mod memory;


pub fn main() {

    let args: Vec<String> = std::env::args().collect();

    if args[1] != "repl" {
        let _memory = memory::load_rom(&Path::new(&args[1]));
    }

    let sdl_context = sdl2::init().expect("Couldn't initialize SDL2!");
    let video_subsystem = sdl_context.video()
        .expect("Couldn't create SDL2 video context!");

    let window = video_subsystem.window("Rust8", 800, 600)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut windowsurface = window.surface(&event_pump)
        .expect("Couldn't get window surface!");

    windowsurface.fill_rect(None, Color::RGB(76, 11, 0))
        .expect("Couldn't fill surface!");

    windowsurface.update_window()
        .expect("Couldn't update window surface!");

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
    }
}
