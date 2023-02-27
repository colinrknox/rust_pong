use graphics::math::Vec2d;
use kinematics::MotionPhysics;
use rand::prelude::*;

pub mod kinematics;

pub const PADDLE_VELOCITY: f64 = 3.0;
pub const BALL_VELOCITY: f64 = 2.0;

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

    pub fn update_right_paddle_velocity(&mut self, velocity: Vec2d<f64>) {
        self.paddle_right.motion.set_velocity(velocity);
    }

    pub fn get_entities(&self) -> Vec<&GameEntity> {
        vec![&self.paddle_right, &self.paddle_left, &self.ball]
    }

    fn resolve_collisions(&mut self) {
        keep_paddle_on_board(&mut self.paddle_left, self.height);
        keep_paddle_on_board(&mut self.paddle_right, self.height);
        keep_ball_on_board(&mut self.ball, self.height, self.width);
        if self
            .ball
            .motion
            .get_position()
            .has_collided(&self.paddle_right.motion.get_position())
            || self
                .ball
                .motion
                .get_position()
                .has_collided(&self.paddle_left.motion.get_position())
        {
            let mut rng = thread_rng();
            let range: f64 = rng.gen_range(-BALL_VELOCITY..BALL_VELOCITY);
            let sign = if self.ball.motion.get_velocity()[0] < 0.0 {
                1.0
            } else {
                -1.0
            };
            self.ball
                .motion
                .set_velocity([(7.0 - range.powi(2)).sqrt() * sign, range]);
        }
    }
}

fn keep_paddle_on_board(paddle: &mut GameEntity, height: f64) {
    if paddle.motion.get_position().y < 0.0 {
        paddle.motion.set_y_position(0.0);
    }
    if paddle.motion.get_position().y > height - paddle.height {
        paddle.motion.set_y_position(height - paddle.height);
    }
}

/*
 * Need to add scoring
 */
fn keep_ball_on_board(ball: &mut GameEntity, height: f64, width: f64) {
    let ball_position = ball.motion.get_position();
    let mut position = [ball_position.x, ball_position.y];
    let mut ball_velocity = ball.motion.get_velocity();
    if position[0] < 0.0 {
        position[0] = 0.0;
        ball_velocity[0] = ball_velocity[0] * -1.0;
    }
    if position[0] > width - ball.width {
        position[0] = width - ball.width;
        ball_velocity[0] = ball_velocity[0] * -1.0;
    }
    if position[1] < 0.0 {
        position[1] = 0.0;
        ball_velocity[1] = ball_velocity[1] * -1.0;
    }
    if position[1] > height - ball.height {
        position[1] = height - ball.height;
        ball_velocity[1] = ball_velocity[1] * -1.0;
    }

    ball.motion.set_position(position);
    ball.motion.set_velocity(ball_velocity);
}

impl GameEntity {
    pub fn new(entity_type: GameEntityType) -> Self {
        match entity_type {
            GameEntityType::Paddle(PaddleType::Left) => GameEntity {
                height: 56.0,
                width: 8.0,
                motion: MotionPhysics::new([44.0, 0.0], 56.0, 8.0),
                color: [1.0, 1.0, 1.0, 0.99],
            },
            GameEntityType::Paddle(PaddleType::Right) => GameEntity {
                height: 56.0,
                width: 8.0,
                motion: MotionPhysics::new([976.0, 0.0], 56.0, 8.0),
                color: [1.0, 1.0, 1.0, 0.99],
            },
            GameEntityType::Ball => {
                let mut ball = GameEntity {
                    height: 8.0,
                    width: 8.0,
                    motion: MotionPhysics::new([512.0, 256.0], 8.0, 8.0),
                    color: [1.0, 1.0, 1.0, 0.99],
                };
                let range: f64 = thread_rng().gen_range(-BALL_VELOCITY..BALL_VELOCITY);
                ball.motion
                    .set_velocity([(7.0 - range.powi(2)).sqrt(), range]);
                ball
            }
        }
    }

    pub fn update(&mut self) {
        self.motion.update();
    }

    pub fn get_size(&self) -> [f64; 4] {
        let motion = self.motion.get_position();
        [motion.x, motion.y, self.width, self.height]
    }

    pub fn get_color(&self) -> [f32; 4] {
        self.color
    }
}

#[cfg(test)]
mod test {}
