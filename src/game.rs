use graphics::math::Vec2d;
use kinematics::MotionPhysics;

pub mod kinematics;

pub enum GameEntityType {
    Paddle(PaddleType),
    Ball,
}

pub enum PaddleType {
    Left,
    Right,
}

pub struct GameBoard {
    pub entities: Vec<Box<GameEntity>>,
}

pub struct GameEntity {
    height: f64,
    width: f64,
    pub motion: MotionPhysics,
    color: [f32; 4],
}

impl GameEntity {
    pub fn new(entity_type: GameEntityType) -> Self {
        match entity_type {
            GameEntityType::Paddle(PaddleType::Left) => GameEntity {
                height: 30.0,
                width: 10.0,
                motion: MotionPhysics::new([80.0, 0.0]),
                color: [1.0, 1.0, 1.0, 0.99],
            },
            GameEntityType::Paddle(PaddleType::Right) => GameEntity {
                height: 30.0,
                width: 10.0,
                motion: MotionPhysics::new([1200.0, 0.0]),
                color: [1.0, 1.0, 1.0, 0.99],
            },
            GameEntityType::Ball => GameEntity {
                height: 8.0,
                width: 8.0,
                motion: MotionPhysics::new([640.0, 480.0]),
                color: [1.0, 1.0, 1.0, 0.99],
            },
            _ => panic!("Invalid paddle type being passed to paddle constructor"),
        }
    }

    pub fn update(&mut self) {
        self.motion.update();
    }

    pub fn get_size(&self) -> [f64; 4] {
        let motion = self.motion.get_position();
        [motion[0], motion[1], self.width, self.height]
    }

    pub fn get_color(&self) -> [f32; 4] {
        self.color
    }
}

#[cfg(test)]
mod test {}
