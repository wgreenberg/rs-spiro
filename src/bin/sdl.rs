use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use std::time::Duration;
use std::f64::consts::PI;
use spiro::spirograph::Spirograph;
use spiro::complex::Complex;

// assumes x and y range from -1 to 1
fn point_to_px(point: Complex, width: u32, height: u32) -> (i32, i32) {
    let min = std::cmp::min(width, height);
    let x = ((point.re + 1.0) / 2.0) * min as f64;
    let y = ((point.im + 1.0) / 2.0) * min as f64;
    (x as i32, y as i32)
}
 
pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let width = 800;
    let height = 800;
 
    let window = video_subsystem.window("spiro", width, height)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
    let spirograph = Spirograph::new_basic(0.6, 0.4, 2.0 * PI);
 
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    let mut t = 0.0;
    let mut last_point = point_to_px(spirograph.sample(t), width, height);
    'running: loop {
        i = (i + 1) % 255;
        t += 0.01;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        let point = point_to_px(spirograph.sample(t), width, height);
        canvas.draw_line(last_point, point);
        last_point = point;
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
