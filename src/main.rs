mod player;

use std::{fs::File, io::Read, path::Path, time::Instant};

use player::Player;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Point};

use crate::player::Side;

pub const SCREEN_WIDTH: u32 = 1280;
pub const SCREEN_HEIGHT: u32 = 720;

fn read_map() -> Vec<Vec<i32>> {
    // Assumes that map in file is rectangular
    let path = Path::new("src/map.txt");
    let mut file = match File::open(&path) {
        Err(why) => panic!("can't open map file: {}", why),
        Ok(file) => file,
    };
    let mut map: Vec<Vec<i32>> = Vec::new();
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    for line in contents.lines() {
        let mut row: Vec<i32> = Vec::new();
        row.push(1); // To ensure the map has surrounding walls
        for char in line.chars() {
            if char.is_numeric() {
                row.push(char.to_digit(10).unwrap() as i32);
            }
        }
        row.push(1); // To ensure the map has surrounding walls
        map.push(row);
    }
    // Add a row of ones to top and bottom of map to ensure surrounding walls
    map.insert(0, vec![1; map[0].len() + 2]);
    map.push(vec![1; map[0].len() + 2]);
    map
}

fn main() {
    let mut player = Player::at_position(1.5, 1.5);
    let map = read_map();

    #[derive(Debug)]
    enum MoveType {
        Move,
        Rotate,
        None,
    }
    let mut movement = MoveType::None;
    let mut forward = true;

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
    let mut now = Instant::now();
    'running: loop {
        let delta = now.elapsed().as_secs_f64();
        now = Instant::now();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    movement = MoveType::Move;
                    forward = true
                }
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    movement = MoveType::Move;
                    forward = false;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    movement = MoveType::Rotate;
                    forward = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    movement = MoveType::Rotate;
                    forward = false;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::W),
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::S),
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::A),
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::D),
                    ..
                } => movement = MoveType::None,
                _ => (),
            }
        }
        match movement {
            MoveType::Move => {
                player.move_player(&map, delta, forward);
            }
            MoveType::Rotate => {
                player.rotate_player(delta, forward);
            }
            _ => (),
        }

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
    }
}
