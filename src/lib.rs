use game::*;
use piston_window::*;

pub mod game;

pub fn run() {
    let (width, height) = (1280.0, 960.0);
    let mut window: PistonWindow = WindowSettings::new("Pong", [width, height])
        .exit_on_esc(true)
        .build()
        .expect("Could not create window.");

    let mut game = Game::new(1280.0, 960.0);

    game.add_entity(GameEntity::new(GameEntityType::Paddle(PaddleType::Right)));
    game.add_entity(GameEntity::new(GameEntityType::Paddle(PaddleType::Left)));
    game.add_entity(GameEntity::new(GameEntityType::Ball));

    // Need to tighten input logic and then contain the paddle within the window
    while let Some(event) = window.next() {
        if let Event::Input(Input::Button(args), _) = &event {
            if args.state == ButtonState::Press {
                if let Button::Keyboard(key) = args.button {
                    match key {
                        Key::S => game.entities[1].motion.set_velocity([0.0, 2.0]),
                        Key::W => game.entities[1].motion.set_velocity([0.0, -2.0]),
                        _ => (),
                    }
                }
            } else if args.state == ButtonState::Release {
                game.entities[1].motion.set_velocity([0.0, 0.0]);
            }
        }

        game.entities[1].update();

        window.draw_2d(&event, |ctx, renderer, _device| {
            clear(color::TRANSPARENT, renderer);

            for obj in &mut game.entities {
                let size = obj.get_size();
                let color = obj.get_color();
                rectangle(color, size, ctx.transform, renderer);
            }
        });
    }
}
