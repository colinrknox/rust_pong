use graphics::math::Vec2d;
use piston_window::*;
use std::collections::HashSet;

enum GameEntityType {
    Paddle(PaddleType),
    Ball,
}

enum PaddleType {
    Left,
    Right,
}

struct GameBoard {
    entities: Vec<Box<dyn GameEntity>>,
}

trait GameEntity {
    fn update_position(&mut self);
    fn update_velocity(&mut self, value: [f64; 2]);
    fn get_size(&self) -> [f64; 4];
    fn get_color(&self) -> [f32; 4];
}

struct Paddle {
    height: f64,
    width: f64,
    position: Vec2d<f64>,
    color: [f32; 4],
    velocity: [f64; 2],
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
    fn new(position: GameEntityType) -> Self {
        match position {
            GameEntityType::Paddle(PaddleType::Left) => Paddle {
                height: 30.0,
                width: 10.0,
                position: [80.0, 0.0],
                color: [1.0, 1.0, 1.0, 0.99],
                velocity: [0.0, 0.0],
            },
            GameEntityType::Paddle(PaddleType::Right) => Paddle {
                height: 30.0,
                width: 10.0,
                position: [1200.0, 0.0],
                color: [1.0, 1.0, 1.0, 0.99],
                velocity: [0.0, 0.0],
            },
            _ => panic!("Invalid paddle type being passed to paddle constructor"),
        }
    }
}

impl GameEntity for Paddle {
    fn update_position(&mut self) {
        self.position[1] += self.velocity[1];
    }

    fn update_velocity(&mut self, value: [f64; 2]) {
        self.velocity = value;
    }

    fn get_size(&self) -> [f64; 4] {
        [self.position[0], self.position[1], self.width, self.height]
    }

    fn get_color(&self) -> [f32; 4] {
        self.color
    }
}

impl Ball {
    fn new() -> Ball {
        Ball {
            height: 8.0,
            width: 8.0,
            position: [640.0, 480.0],
            velocity: [0.0, 0.0],
            acceleration: [0.0, 0.0],
            color: [1.0, 1.0, 1.0, 1.0],
        }
    }
}

impl GameEntity for Ball {
    fn update_position(&mut self) {
        self.position[0] += self.velocity[0];
        self.position[1] += self.velocity[1];
    }

    fn update_velocity(&mut self, value: [f64; 2]) {
        self.velocity = value;
    }

    fn get_size(&self) -> [f64; 4] {
        [self.position[0], self.position[1], self.width, self.height]
    }

    fn get_color(&self) -> [f32; 4] {
        self.color
    }
}

fn main() {
    let (width, height) = (1280.0, 960.0);
    let mut window: PistonWindow = WindowSettings::new("Pong", [width, height])
        .exit_on_esc(true)
        .build()
        .expect("Could not create window.");

    let mut board = GameBoard {
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
