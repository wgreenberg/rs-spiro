use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::render::BlendMode;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use std::time::Duration;
use std::f64::consts::PI;
use spiro::spirograph::Spirograph;
use spiro::complex::Complex;
use rand::prelude::*;

// assumes x and y range from -1 to 1
fn point_to_px(point: Complex, width: u32, height: u32) -> Point {
    let scale = 4.0;
    let min = std::cmp::min(width, height);
    let x = ((point.re + scale/2.0) / scale) * min as f64;
    let y = ((point.im + scale/2.0) / scale) * min as f64;
    (x as i32, y as i32).into()
}
 
pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let width = 1000;
    let height = 1000;
 
    let window = video_subsystem.window("spiro", width, height)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
    let mut graphs: Vec<Spirograph> = (0..32).map(|i| {
        Spirograph::new(0.2, 0.1 * i as f64, 2.0 * PI, 2.5)
    }).collect();
    canvas.set_draw_color(Color::RGB(0x07, 0x36, 0x42));
    canvas.clear();
    canvas.present();
    canvas.set_draw_color(Color::RGBA(0x93, 0xa1, 0xa1, 2));
    canvas.set_blend_mode(BlendMode::Add);
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut rng = rand::thread_rng();
    'running: loop {
        for _ in 0..1000 {
            let mut t: f64 = rng.gen();
            t *= 5.0;
            let points: Vec<Point> = graphs.iter()
                .map(|s| point_to_px(s.sample(t), width, height)).collect();
            canvas.draw_points(&points[..]);
            for mut s in &mut graphs {
                s.offset_phase(0.000001);
            }
        }
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
