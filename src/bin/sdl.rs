use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::surface::Surface;
use sdl2::event::Event;
use sdl2::render::{BlendMode, Texture};
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use std::time::Duration;
use std::f64::consts::PI;
use spiro::spirograph::Spirograph;
use spiro::complex::Complex;
use rand::prelude::*;

// assumes x and y range from -1 to 1
fn point_to_px(point: Complex, width: u32, height: u32) -> Point {
    let scale = 2.0;
    let min = std::cmp::min(width, height);
    let x = ((point.re + scale/2.0) / scale) * min as f64;
    let y = ((point.im + scale/2.0) / scale) * min as f64;
    (x as i32, y as i32).into()
}

struct SpirographFamily {
    graphs: Vec<Spirograph>,
    color: Color,
}

impl SpirographFamily {
    fn new<F>(n: u32, color: Color, init: F) -> SpirographFamily where F: Fn(u32) -> Spirograph {
        SpirographFamily {
            graphs: (0..n).map(init).collect::<Vec<_>>(),
            color,
        }
    }

    fn sample(&self, t: f64) -> Vec<Complex> {
        self.graphs.iter().map(|s| s.sample(t)).collect::<Vec<_>>()
    }

    fn offset(&mut self, phase: f64, amplitude: f64) {
        self.graphs.iter_mut().for_each(|s| {
            s.offset_phase(phase);
            s.offset_amplitude(amplitude);
        });
    }
}
 
pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let width = 1300;
    let height = 1300;
 
    let window = video_subsystem.window("spiro", width, height)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
    let n = 4;
    let alpha = 16;
    let mut families = vec![
        SpirographFamily::new(16, Color::RGBA(0xff, 0x33, 0xf0, alpha), |i| {
            let a = i % n;
            let b = i / n;
            let inner_ratio = (n as f64 - 0.1).recip() * a as f64 + 0.1;
            let pen_ratio = (n as f64 - 0.1).recip() * b as f64 + 0.1;
            let pen_freq_ratio = 2.0;
            Spirograph::new(inner_ratio, pen_ratio, 2.0 * PI, pen_freq_ratio)
        }),
        SpirographFamily::new(16, Color::RGBA(0x33, 0xff, 0xbf, alpha), |i| {
            let a = i % n;
            let b = i / n;
            let inner_ratio = (n as f64 - 0.1).recip() * a as f64 + 0.1;
            let pen_ratio = (n as f64 - 0.1).recip() * b as f64 + 0.1;
            let pen_freq_ratio = 3.0;
            Spirograph::new(inner_ratio, pen_ratio, 2.0 * PI, pen_freq_ratio)
        }),
    ];
    canvas.set_draw_color(Color::RGB(0x2d, 0x2a, 0x32));
    canvas.clear();
    canvas.present();
    canvas.set_blend_mode(BlendMode::Add);
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut rng = rand::thread_rng();
    let mut i = 0;
    'running: loop {
        i += 1;
        for _ in 0..1000 {
            let mut t: f64 = rng.gen();
            let jitter = 5.0;
            let dy: i32 = ((rng.gen::<f64>() - 0.5) * jitter) as i32;
            let dx: i32 = ((rng.gen::<f64>() - 0.5) * jitter) as i32;
            t *= 5.0;
            for family in &families {
                let points: Vec<Point> = family.sample(t).iter()
                    .map(|p| point_to_px(*p, width, height).offset(dx, dy))
                    .collect();
                canvas.set_draw_color(family.color);
                canvas.draw_points(&points[..]);
            }
        }
        families[0].offset(0.005 * (0.1 * i as f64).sin(), -0.001);
        families[1].offset(0.001, 0.001 * (0.01 * i as f64).sin());
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    let mut surface = Surface::new(width, height, PixelFormatEnum::RGB24).unwrap();
                    let data = canvas.read_pixels(None, PixelFormatEnum::RGB24).unwrap();
                    surface.with_lock_mut(|mut pixels| {
                        for (i, mut px) in pixels.iter_mut().enumerate() {
                            *px = data[i];
                        }
                    });
                    surface.save_bmp(format!("screenshot-{}", i)).unwrap();
                },
                _ => {}
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
