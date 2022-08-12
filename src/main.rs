extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("GOL", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator.create_texture_streaming(sdl2::pixels::PixelFormatEnum::ARGB8888, WIDTH, HEIGHT).unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut frame = 0;
    //let mut pxl_vec: [u8; (WIDTH * HEIGHT * 4) as usize]= [255; (WIDTH * HEIGHT * 4) as usize];
    let mut pxl_vec = vec![255u8; (WIDTH * HEIGHT * 4) as usize].into_boxed_slice();

    'running: loop {
        frame = (frame + 1) % 255;
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                let offset:usize = (i * WIDTH * 4 + j * 4) as usize;
                pxl_vec[offset    ] = frame;
                pxl_vec[offset + 1] = frame;
                pxl_vec[offset + 2] = frame;
                pxl_vec[offset + 3] = 255;
            }
        }
        texture.with_lock(
            None,
            |bytearray, _|{
                for i in 0..bytearray.len() {
                    bytearray[i] = pxl_vec[i];
                }
            }
        ).unwrap();
        canvas.clear();
        canvas.copy(&texture, None, None).unwrap();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => ()
            }
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0,1_000_000_000u32 / 60));
    }
}

