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

pub struct Pong {
    height: f64,
    width: f64,
    paddle_left: GameEntity,
    paddle_right: GameEntity,
    ball: GameEntity,
}

pub struct GameEntity {
    height: f64,
    width: f64,
    motion: MotionPhysics,
    color: [f32; 4],
}

impl Pong {
    pub fn new(height: f64, width: f64) -> Pong {
        Pong {
            height,
            width,
            paddle_left: GameEntity::new(GameEntityType::Paddle(PaddleType::Left)),
            paddle_right: GameEntity::new(GameEntityType::Paddle(PaddleType::Right)),
            ball: GameEntity::new(GameEntityType::Ball),
        }
    }

    pub fn update(&mut self) {
        self.paddle_left.update();
        self.paddle_right.update();
        self.ball.update();
        self.resolve_collisions();
    }

    pub fn update_left_paddle_velocity(&mut self, velocity: Vec2d<f64>) {
        self.paddle_left.motion.set_velocity(velocity);
    }

    pub fn get_entities(&self) -> Vec<&GameEntity> {
        vec![&self.paddle_right, &self.paddle_left, &self.ball]
    }

    fn resolve_collisions(&mut self) {
        // TODO resolve y bound collisions for the paddles as well as x-y for the ball and paddle-ball
        keep_paddle_on_board(&mut self.paddle_left, self.height);
        keep_paddle_on_board(&mut self.paddle_right, self.height);
    }
}

fn keep_paddle_on_board(paddle: &mut GameEntity, height: f64) {
    if paddle.motion.get_position()[1] < 0.0 {
        paddle.motion.set_y_position(0.0);
    }
    if paddle.motion.get_position()[1] > height - paddle.height {
        paddle.motion.set_y_position(height - paddle.height);
    }
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
