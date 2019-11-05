mod error;
mod game;
mod player;

use crate::util::*;
pub use error::*;
use game::Game;
use std::time::Duration;

/// A collection position, velocity and scale of an entity.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform {
    pub pos: Point,
    pub vel: Vector,
    pub scale: D,
}

impl Default for Transform {
    fn default() -> Self {
        Transform {
            pos: [0., 0., 0.].into(),
            vel: [0., 0., 0.].into(),
            scale: 1.,
        }
    }
}

pub type Direction = [f32; 2];

/// Commands that can be generated from user input or AI behaviour.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Command {
    Move(Direction),
}

/// A thing that can be influenced by Commands.
/// These will not happen on every frame.
trait Commandable {
    fn handle(&mut self, c: Command) -> SimResult;
}

/// Something that may update on every frame.
trait Updateable {
    fn update(&mut self, delta: Duration) -> SimResult;
}

/// Moves a Transform by its velocity. Bound by the games bounds.
impl Updateable for Transform {
    fn update(&mut self, d: Duration) -> SimResult {
        self.pos += self.vel.scale(duration_float(d));
        self.pos = [
            self.pos[0].max(Game::LEFT_BOUND).min(Game::RIGHT_BOUND),
            self.pos[1].max(Game::LOWER_BOUND).min(Game::UPPER_BOUND),
            self.pos[2].min(0.),
        ]
        .into();
        Ok(())
    }
}
