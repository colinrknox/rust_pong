use graphics::math::Vec2d;

pub enum GameEntityType {
    Paddle(PaddleType),
    Ball,
}

pub enum PaddleType {
    Left,
    Right,
}

pub struct GameBoard {
    pub entities: Vec<Box<dyn GameEntity>>,
}

pub trait GameEntity {
    fn update_position(&mut self);
    fn update_velocity(&mut self, value: [f64; 2]);
    fn get_size(&self) -> [f64; 4];
    fn get_color(&self) -> [f32; 4];
}

pub struct Paddle {
    height: f64,
    width: f64,
    position: Vec2d<f64>,
    color: [f32; 4],
    velocity: [f64; 2],
}

pub struct Ball {
    height: f64,
    width: f64,
    position: Vec2d<f64>,
    velocity: Vec2d<f64>,
    acceleration: Vec2d<f64>,
    color: [f32; 4],
}

impl Paddle {
    pub fn new(position: GameEntityType) -> Self {
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
    pub fn new() -> Ball {
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

#[cfg(test)]
mod test {}
