use piston_window::*;
use rust_pong::game::*;

fn main() {
    let (width, height) = (1280.0, 960.0);
    let mut window: PistonWindow = WindowSettings::new("Pong", [width, height])
        .exit_on_esc(true)
        .build()
        .expect("Could not create window.");

    let mut board = rust_pong::game::GameBoard {
        entities: Vec::new(),
    };
    board
        .entities
        .push(Box::new(Paddle::new(GameEntityType::Paddle(
            PaddleType::Right,
        ))));
    board
        .entities
        .push(Box::new(Paddle::new(GameEntityType::Paddle(
            PaddleType::Left,
        ))));
    board.entities.push(Box::new(Ball::new()));

    // Need to tighten input logic and then contain the paddle within the window
    while let Some(event) = window.next() {
        if let Event::Input(Input::Button(args), _) = &event {
            if args.state == ButtonState::Press {
                if let Button::Keyboard(key) = args.button {
                    match key {
                        Key::S => board.entities[1].update_velocity([0.0, 2.0]),
                        Key::W => board.entities[1].update_velocity([0.0, -2.0]),
                        _ => (),
                    }
                }
            } else if args.state == ButtonState::Release {
                board.entities[1].update_velocity([0.0, 0.0]);
            }
        }

        board.entities[1].update_position();

        window.draw_2d(&event, |ctx, renderer, _device| {
            clear(color::TRANSPARENT, renderer);

            for obj in &mut board.entities {
                let size = obj.get_size();
                let color = obj.get_color();
                rectangle(color, size, ctx.transform, renderer);
            }
        });
    }
}
