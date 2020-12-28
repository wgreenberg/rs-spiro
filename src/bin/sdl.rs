use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::render::BlendMode;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use std::time::Duration;
use std::f64::consts::PI;
use spiro::spirograph::Spirograph;
use spiro::complex::Complex;

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
    let width = 1600;
    let height = 1600;
 
    let window = video_subsystem.window("spiro", width, height)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
    let mut graphs: Vec<Spirograph> = (0..64).map(|i| {
        Spirograph::new(0.2, 0.1 * i as f64, 2.0 * PI, 2.0)
    }).collect();
    let mut t = 0.0;
    let mut last_points: Vec<Point> = graphs.iter()
        .map(|s| point_to_px(s.sample(t), width, height)).collect();
 
    canvas.set_draw_color(Color::RGB(0x07, 0x36, 0x42));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        t += 0.001;
        canvas.set_draw_color(Color::RGBA(0x93, 0xa1, 0xa1, 32));
        canvas.set_blend_mode(BlendMode::Add);
        let points: Vec<Point> = graphs.iter()
            .map(|s| point_to_px(s.sample(t), width, height)).collect();
        points.iter().zip(last_points).for_each(|(p1, p2)| canvas.draw_line(*p1, *p2).unwrap());
        last_points = points;
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
