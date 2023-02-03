#![windows_subsystem = "windows"]

use std::time::{Duration, Instant};

use pixels::{PixelsBuilder, SurfaceTexture};
use sdl2::audio::{AudioQueue, AudioSpecDesired};
use sdl2::event::{Event, WindowEvent};

use crate::gameboy::gameboy::Gameboy;

mod gameboy;

fn main() {
    println!("Starting Anemulator2");
    println!("{}", std::env::current_dir().unwrap().display());

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

    let mut gameboy = Gameboy::new(String::from("assets/Dr. Mario (World).gb"));

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

        // STEP EMULATION
        while !gameboy.step() {
            if gameboy.mmu.apu.is_buffer_full() {
                audio_queue.queue(&gameboy.mmu.apu.fetch_samples());
                if audio_queue.size() > 1024 * 8 {
                    std::thread::sleep(Duration::from_millis(1));
                }
            }
        }

        // RENDER TO SCREEN
        let screen = pixels.get_frame_mut();
        let ppu_buffer = gameboy.mmu.ppu.front_buffer.as_ref();
        screen[0] = ppu_buffer[0][0].r_as_u8();
        let mut byte_index = 0;
        for color in ppu_buffer.iter().flat_map(|r| r.iter()) {
            screen[byte_index] = color.r_as_u8();
            byte_index += 1;
            screen[byte_index] = color.g_as_u8();
            byte_index += 1;
            screen[byte_index] = color.b_as_u8();
            byte_index += 1;
            screen[byte_index] = color.a_as_u8();
            byte_index += 1;
        }
        pixels.render().expect("failed to render framebuffer");

        // PRINT FRAME TIME
        let duration = start.elapsed();
        println!("frame-time: {duration:?}");
    }
}
