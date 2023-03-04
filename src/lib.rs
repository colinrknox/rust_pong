use std::time::{Duration, Instant};

use crate::game::*;
use macroquad::prelude::*;
use macroquad::window::Conf;

pub mod game;

pub async fn macroquad_run() {
    let font = load_ttf_font("assets/blippo_bold.ttf").await.unwrap();

    let mut game = Pong::new();

    let mut prev = Instant::now();
    loop {
        if prev.elapsed() > Duration::from_millis(33) {
            if is_key_down(KeyCode::Escape) {
                break;
            }

            if is_key_down(KeyCode::S) {
                game.update_left_paddle_velocity([0.0, PADDLE_VELOCITY])
            }
            if is_key_down(KeyCode::W) {
                game.update_left_paddle_velocity([0.0, -PADDLE_VELOCITY]);
            }
            if is_key_released(KeyCode::W) | is_key_released(KeyCode::S) {
                game.update_left_paddle_velocity([0.0, 0.0]);
            }

            if is_key_down(KeyCode::Down) {
                game.update_right_paddle_velocity([0.0, PADDLE_VELOCITY])
            }
            if is_key_down(KeyCode::Up) {
                game.update_right_paddle_velocity([0.0, -PADDLE_VELOCITY]);
            }
            if is_key_released(KeyCode::Down) | is_key_released(KeyCode::Up) {
                game.update_right_paddle_velocity([0.0, 0.0]);
            }

            game.update();

            clear_background(BLACK);
            for entity in &game.get_entities()[0..2] {
                let size = entity.get_size();
                draw_rectangle(
                    size[0] as f32,
                    size[1] as f32,
                    size[2] as f32,
                    size[3] as f32,
                    WHITE,
                );
            }

            draw_circle(
                game.get_entities()[2].get_size()[0] as f32,
                game.get_entities()[2].get_size()[1] as f32,
                game.get_entities()[2].get_size()[2] as f32,
                WHITE,
            );

            draw_text_ex(
                &game.get_player_one_score().to_string(),
                PONG_WIDTH as f32 / 4.,
                40.,
                TextParams {
                    font_size: 50,
                    font,
                    ..Default::default()
                },
            );

            draw_text_ex(
                &game.get_player_two_score().to_string(),
                (PONG_WIDTH as f32 / 4.) * 3.,
                40.,
                TextParams {
                    font_size: 50,
                    font,
                    ..Default::default()
                },
            );
            prev = Instant::now();
            next_frame().await
        }
    }
}

pub fn config() -> Conf {
    Conf {
        window_title: "Pong".to_string(),
        window_height: PONG_HEIGHT as i32,
        window_width: PONG_WIDTH as i32,
        window_resizable: false,
        ..Default::default()
    }
}
