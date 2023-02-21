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
                height: 20.0,
                width: 8.0,
                position: [0.0, 0.0],
                color: [1.0, 1.0, 1.0, 0.99],
            },
            PaddlePosition::Right => Paddle {
                height: 20.0,
                width: 8.0,
                position: [1200.0, 0.0],
                color: [1.0, 1.0, 1.0, 0.99],
            },
        }
    }

    fn down(&mut self) {
        self.position[1] += 1.0;
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

    while let Some(event) = window.next() {
        if let Event::Input(Input::Button(args), _) = &event {
            if let Button::Keyboard(key) = args.button {
                if key == Key::S {
                    objects[0].down();
                    objects[1].down();
                }
            }
        }

        window.draw_2d(&event, |ctx, renderer, _device| {
            clear(color::TRANSPARENT, renderer);

            for obj in &mut objects {
                let size = [obj.position[0], obj.position[1], obj.width, obj.height];
                rectangle(obj.color, size, ctx.transform, renderer);
            }
        });
    }
}
