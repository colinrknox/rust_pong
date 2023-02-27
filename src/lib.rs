use game::*;
use piston_window::*;

pub mod game;

pub fn run() {
    let (width, height) = (1024.0, 512.0);
    let mut window: PistonWindow = WindowSettings::new("Pong", [width, height])
        .exit_on_esc(true)
        .build()
        .expect("Could not create window.");
    window.set_max_fps(60);
    window.set_ups(60);

    let mut game = Pong::new(height, width);

    // Need to tighten input logic and then contain the paddle within the window
    while let Some(event) = window.next() {
        if let Event::Input(Input::Button(args), _) = &event {
            if args.state == ButtonState::Press {
                if let Button::Keyboard(key) = args.button {
                    match key {
                        Key::S => game.update_left_paddle_velocity([0.0, PADDLE_VELOCITY]),
                        Key::W => game.update_left_paddle_velocity([0.0, -PADDLE_VELOCITY]),
                        Key::Down => game.update_right_paddle_velocity([0.0, PADDLE_VELOCITY]),
                        Key::Up => game.update_right_paddle_velocity([0.0, -PADDLE_VELOCITY]),
                        _ => (),
                    }
                }
            } else if args.state == ButtonState::Release {
                if let Button::Keyboard(key) = args.button {
                    match key {
                        Key::S => game.update_left_paddle_velocity([0.0, 0.0]),
                        Key::W => game.update_left_paddle_velocity([0.0, 0.0]),
                        Key::Up => game.update_right_paddle_velocity([0.0, 0.0]),
                        Key::Down => game.update_right_paddle_velocity([0.0, 0.0]),
                        _ => (),
                    }
                }
            }
        }

        game.update();

        window.draw_2d(&event, |ctx, renderer, _device| {
            clear(color::TRANSPARENT, renderer);

            for entity in game.get_entities() {
                let size = entity.get_size();
                let color = entity.get_color();
                rectangle(color, size, ctx.transform, renderer);
            }
        });
    }
}
