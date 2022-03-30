use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use std::f32::consts::PI;
use std::time::Duration;

const TITLE: &str = "";
const HEIGHT: u32 = 600;
const WIDTH: u32 = 800;
const MOVEMENT: f32 = 0.2;
const ANGLE_MOVEMENT: f32 = PI as f32 / 10_f32;

const MAP_SIZE: usize = 12;
#[cfg_attr(rustfmt, rustfmt_skip)]
const MAP: &[u8; MAP_SIZE * MAP_SIZE] = &[
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1,
    1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
];

const CELL_SIZE: u32 = HEIGHT / MAP_SIZE as u32;
const PLAYER_SIZE: u32 = CELL_SIZE / 5;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(TITLE, WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump()?;

    let mut player_angle = 0_f32;
    let mut player_x = 5_f32;
    let mut player_y = 5_f32;

    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }

                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::Up => {
                        player_y += MOVEMENT * player_angle.sin();
                        player_x += MOVEMENT * player_angle.cos();
                    }
                    Keycode::Down => {
                        player_y -= MOVEMENT * player_angle.sin();
                        player_x -= MOVEMENT * player_angle.cos();
                    }
                    Keycode::Left => {
                        player_angle -= ANGLE_MOVEMENT;
                        if player_angle < 0_f32 {
                            player_angle = PI;
                        }
                    }
                    Keycode::Right => {
                        player_angle += ANGLE_MOVEMENT;
                        if player_angle > 2_f32 * PI {
                            player_angle = 0_f32;
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        // Update

        // Render
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        for (i, item) in MAP.into_iter().enumerate() {
            if *item == 0_u8 {
                canvas.set_draw_color(Color::RGB(50, 0, 0));
            } else {
                canvas.set_draw_color(Color::RGB(50, 0, 100));
            }

            let x = (i % MAP_SIZE) * CELL_SIZE as usize;
            let y = (i as f64 / MAP_SIZE as f64).floor() as u32 * CELL_SIZE;

            canvas.fill_rect(Rect::new(x as i32, y as i32, CELL_SIZE, CELL_SIZE))?;
        }

        canvas.set_draw_color(Color::RGB(0, 0, 255));
        let player_x = (player_x * CELL_SIZE as f32) as i32;
        let player_y = (player_y * CELL_SIZE as f32) as i32;

        canvas.draw_line(
            Point::new(player_x, player_y),
            Point::new(
                (player_x as f32 + player_angle.cos() * CELL_SIZE as f32) as i32,
                (player_y as f32 + player_angle.sin() * CELL_SIZE as f32) as i32,
            ),
        )?;
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.fill_rect(Rect::new(
            player_x - PLAYER_SIZE as i32 / 2,
            player_y - PLAYER_SIZE as i32 / 2,
            PLAYER_SIZE,
            PLAYER_SIZE,
        ))?;
        canvas.present();

        // Time management!
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
