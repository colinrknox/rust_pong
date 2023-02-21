use graphics::math::Vec2d;
use piston_window::*;
use std::collections::HashSet;

enum PaddlePosition {
    Left,
    Right,
}

struct Paddle {
    height: f64,
    width: f64,
    position: Vec2d<f64>,
    color: [f32; 4],
    velocity_multiplier: f64,
}

struct Ball {
    height: f64,
    width: f64,
    position: Vec2d<f64>,
    velocity: Vec2d<f64>,
    acceleration: Vec2d<f64>,
    color: [f32; 4],
}

impl Paddle {
    fn new(position: PaddlePosition) -> Self {
        match position {
            PaddlePosition::Left => Paddle {
                height: 30.0,
                width: 10.0,
                position: [80.0, 0.0],
                color: [1.0, 1.0, 1.0, 0.99],
                velocity_multiplier: 0.0,
            },
            PaddlePosition::Right => Paddle {
                height: 30.0,
                width: 10.0,
                position: [1200.0, 0.0],
                color: [1.0, 1.0, 1.0, 0.99],
                velocity_multiplier: 0.0,
            },
        }
    }

    fn update(&mut self) {
        self.position[1] += self.velocity_multiplier;
    }
}

fn main() {
    let (width, height) = (1280.0, 960.0);
    let mut window: PistonWindow = WindowSettings::new("Pong", [width, height])
        .exit_on_esc(true)
        .build()
        .expect("Could not create window.");

    let mut objects = Vec::new();
    objects.push(Paddle::new(PaddlePosition::Right));
    objects.push(Paddle::new(PaddlePosition::Left));

    // Need to tighten input logic and then contain the paddle within the window
    while let Some(event) = window.next() {
        if let Event::Input(Input::Button(args), _) = &event {
            if args.state == ButtonState::Press {
                if let Button::Keyboard(key) = args.button {
                    match key {
                        Key::S => objects[1].velocity_multiplier = 2.0,
                        Key::W => objects[1].velocity_multiplier = -2.0,
                        _ => (),
                    }
                }
            } else if args.state == ButtonState::Release {
                objects[1].velocity_multiplier = 0.0;
            }
        }

        objects[1].update();

        window.draw_2d(&event, |ctx, renderer, _device| {
            clear(color::TRANSPARENT, renderer);

            for obj in &mut objects {
                let size = [obj.position[0], obj.position[1], obj.width, obj.height];
                rectangle(obj.color, size, ctx.transform, renderer);
            }
        });
    }
}
