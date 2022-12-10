#![windows_subsystem = "windows"]

mod gameboy;

use crate::gameboy::gameboy::Gameboy;
use pixels::{PixelsBuilder, SurfaceTexture};
use sdl2::audio::{AudioQueue, AudioSpecDesired};
use sdl2::event::{Event, WindowEvent};
use std::time::Instant;

fn main() {
    println!("Starting Anemulator2");

    let sdl = sdl2::init().expect("failed to initialize SDL");
    let audio_subsystem = sdl
        .audio()
        .expect("failed to initialize SDL audio subsystem");
    let video_subsystem = sdl
        .video()
        .expect("failed to initialize SDL video subsystem");
    let window = video_subsystem
        .window("Anemulator2", 480, 432)
        .position_centered()
        .resizable()
        .build()
        .expect("failed to create a window");

    let mut pixels = {
        let window_size = window.drawable_size();
        let texture = SurfaceTexture::new(window_size.0, window_size.1, &window);
        PixelsBuilder::new(160, 144, texture)
            .enable_vsync(true)
            .build()
            .expect("failed to create a Pixels instance")
    };

    let desired_spec = AudioSpecDesired {
        freq: Some(48000),
        channels: Some(2),
        samples: Some(1024),
    };

    let audio_queue: AudioQueue<f32> = audio_subsystem
        .open_queue(None, &desired_spec)
        .expect("failed to create audio queue");

    audio_queue.resume();

    let mut event_pump = sdl.event_pump().expect("failed to get the event_pump");

    let gameboy = Gameboy::new(String::from("test.rom"));

    'main: loop {
        let start = Instant::now();

        // Handle window events if any
        for event in event_pump.poll_iter() {
            match event {
                //     Event::KeyDown {
                //         scancode: Some(code),
                //         ..
                //     } => {
                //         if let Some(key) = map_scancode_key(code) {
                //            // argentum.key_down(key);
                //         }
                //     }
                //
                //     Event::KeyUp {
                //         scancode: Some(code),
                //         ..
                //     } => {
                //         if let Some(key) = map_scancode_key(code) {
                //          //   argentum.key_up(key);
                //         }
                //     }
                //
                Event::Quit { .. } => {
                    break 'main;
                }

                Event::Window {
                    win_event: WindowEvent::Resized(width, height),
                    ..
                } => {
                    pixels.resize_surface(width as u32, height as u32);
                }

                _ => {}
            }
        }

        pixels.render().expect("failed to render framebuffer");

        // PRINT FRAME TIME
        let duration = start.elapsed();
        println!("frametime: {:?}", duration);
    }
}
