use game::*;
use piston_window::*;

pub mod game;

pub fn run() {
    let (width, height) = (1280.0, 960.0);
    let mut window: PistonWindow = WindowSettings::new("Pong", [width, height])
        .exit_on_esc(true)
        .build()
        .expect("Could not create window.");
    window.set_max_fps(30);

    let mut game = Pong::new(height, width);

    // Need to tighten input logic and then contain the paddle within the window
    while let Some(event) = window.next() {
        if let Event::Input(Input::Button(args), _) = &event {
            if args.state == ButtonState::Press {
                if let Button::Keyboard(key) = args.button {
                    match key {
                        Key::S => game.update_left_paddle_velocity([0.0, 2.0]),
                        Key::W => game.update_left_paddle_velocity([0.0, -2.0]),
                        _ => (),
                    }
                }
            } else if args.state == ButtonState::Release {
                game.update_left_paddle_velocity([0.0, 0.0]);
            }
        }

        game.update();

        window.draw_2d(&event, |ctx, renderer, _device| {
            clear(color::TRANSPARENT, renderer);

            for obj in game.get_entities() {
                let size = obj.get_size();
                let color = obj.get_color();
                rectangle(color, size, ctx.transform, renderer);
            }
        });
    }
}
