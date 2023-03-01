use crate::game::*;
use macroquad::prelude::*;
use macroquad::window::Conf;

pub mod game;

// pub fn piston_run() {
//     let (width, height) = (PONG_WIDTH, PONG_HEIGHT);
//     let opengl = OpenGL::V4_5;
//     let mut window: PistonWindow = WindowSettings::new("Pong", [width, height])
//         .graphics_api(opengl)
//         .resizable(false)
//         .exit_on_esc(true)
//         .build()
//         .expect("Could not create window.");
//     window.set_max_fps(60);
//     window.set_ups(60);
//
//     let mut game = Pong::new();
//
//     let font_path = Path::new("assets/blippo_bold.ttf");
//     let texture_context = window.create_texture_context();
//     let mut glyphs = Glyphs::new(&font_path, texture_context, TextureSettings::new()).unwrap();
//
//     // Need to tighten input logic and then contain the paddle within the windows
//     while let Some(event) = window.next() {
//         if let Event::Input(Input::Button(args), _) = &event {
//             if args.state == ButtonState::Press {
//                 if let Button::Keyboard(key) = args.button {
//                     match key {
//                         Key::S => game.update_left_paddle_velocity([0.0, PADDLE_VELOCITY]),
//                         Key::W => game.update_left_paddle_velocity([0.0, -PADDLE_VELOCITY]),
//                         Key::Down => game.update_right_paddle_velocity([0.0, PADDLE_VELOCITY]),
//                         Key::Up => game.update_right_paddle_velocity([0.0, -PADDLE_VELOCITY]),
//                         _ => (),
//                     }
//                 }
//             } else if args.state == ButtonState::Release {
//                 if let Button::Keyboard(key) = args.button {
//                     match key {
//                         Key::S => game.update_left_paddle_velocity([0.0, 0.0]),
//                         Key::W => game.update_left_paddle_velocity([0.0, 0.0]),
//                         Key::Up => game.update_right_paddle_velocity([0.0, 0.0]),
//                         Key::Down => game.update_right_paddle_velocity([0.0, 0.0]),
//                         _ => (),
//                     }
//                 }
//             }
//         }
//
//         game.update();
//
//         let text_width = glyphs
//             .width(30, &game.get_player_one_score().to_string())
//             .unwrap();
//
//         window.draw_2d(&event, |ctx, renderer, _device| {
//             clear(color::TRANSPARENT, renderer);
//
//             for entity in game.get_entities().iter() {
//                 let size = entity.get_size();
//                 let color = entity.get_color();
//                 rectangle(color, size, ctx.transform, renderer);
//             }
//
//             text(
//                 color::WHITE,
//                 30,
//                 &game.get_player_one_score().to_string(),
//                 &mut glyphs,
//                 ctx.transform.trans(100.0 - text_width, 30.0),
//                 renderer,
//             )
//             .unwrap();
//         });
//     }
// }

pub async fn macroquad_run() {
    let font = load_ttf_font("assets/blippo_bold.ttf").await.unwrap();

    let mut game = Pong::new();

    loop {
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
        for entity in game.get_entities() {
            let size = entity.get_size();
            draw_rectangle(
                size[0] as f32,
                size[1] as f32,
                size[2] as f32,
                size[3] as f32,
                WHITE,
            );
        }

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

        next_frame().await
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
