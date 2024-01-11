mod player;

use std::time::Duration;

use player::Player;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Point, sys::Screen};

use crate::player::Side;

pub const SCREEN_WIDTH: u32 = 1280;
pub const SCREEN_HEIGHT: u32 = 720;

fn main() {
    let mut player = Player::at_position(1.5, 1.5);
    fn simple_map() -> Vec<Vec<i32>> {
        vec![
            vec![1, 1, 1, 1, 1],
            vec![1, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 1],
            vec![1, 1, 1, 1, 1],
        ]
    }
    let map = simple_map();
    // Placeholder sdl setup code from docs
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Raycasting", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        let (dists, sides) = player.get_wall_distances(&map, SCREEN_WIDTH);
        let inverse_dists: Vec<f64> = dists.iter().map(|i| (1.0 / i)).collect();
        let line_heights: Vec<i32> = inverse_dists
            .iter()
            .map(|i| (SCREEN_HEIGHT as f64 * i) as i32)
            .collect();
        let start_points: Vec<i32> = line_heights
            .iter()
            .map(|i| SCREEN_HEIGHT as i32 / 2 - i / 2)
            .collect();
        let end_points: Vec<i32> = line_heights
            .iter()
            .map(|i| SCREEN_HEIGHT as i32 / 2 + i / 2)
            .collect();

        for x in 0..(line_heights.len() as i32) {
            let color = if sides[x as usize] == Side::Vertical {
                Color::RGB(0, 127, 127)
            } else {
                Color::RGB(0, 255, 255)
            };
            canvas.set_draw_color(color);
            canvas
                .draw_line(
                    Point::new(x, start_points[x as usize]),
                    Point::new(x, end_points[x as usize]),
                )
                .unwrap();
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
