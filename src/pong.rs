extern crate cgmath;

use std::ops::Sub;
use cgmath::{ Vector2 };

pub enum State {
    Uninitialized,
    Ready,
    Running
}

#[derive(Debug)]
pub enum Error {
    NotInitialized
}

pub enum Player {
    Left, Right
}

pub enum Direction {
    Up, Neutral, Down
}

pub enum Action {
    Initialize,
    Start,
    Reset { seed: i64 },
    Time { t: u64 },
    Move { player: Player, direction: Direction }
}

pub enum Entity {
    LeftPaddle, RightPaddle, Ball
}

pub type ID = u64;

pub enum Event {
    Create { id: ID, entity: Entity, x: i64, y: i64 },
    Destroy { id: ID },
    Move { id: ID, x: i64, y: i64 },
    Goal { player: Player, score: u8 },
    Reset,
    RoundStart
}

#[derive(Clone)]
pub struct GameConfiguration {
    pub area: Vector2<i64>,
    pub paddle: Vector2<i64>,
    pub ball_size: i64
}

struct PlayerData {
    id: ID,
    score: u8,
    position: Vector2<i64>,
    velocity: Vector2<i64>
}
struct BallData {
    id: ID,
    position: Vector2<i64>,
    velocity: Vector2<i64>
}
pub struct Game {
    state: State,
    cfg: GameConfiguration,
    t: u64,
    left: PlayerData,
    right: PlayerData,
    ball: BallData
}

impl Game {
    pub fn new(cfg: GameConfiguration) -> Game {
        let game = Game {
            state: State::Uninitialized,
            t: 0,
            left: PlayerData {
                id: 0,
                score: 0,
                position: Vector2::new(cfg.paddle.x - cfg.area.x, 0),
                velocity: Vector2::new(0, 0)
            },
            right: PlayerData {
                id: 1,
                score: 0,
                position: Vector2::new(cfg.area.x - cfg.paddle.x, 0),
                velocity: Vector2::new(0, 0)
            },
            ball: BallData {
                id: 2,
                position: Vector2::new(0, 0),
                velocity: Vector2::new(240, 240)
            },
            cfg: cfg
        };
        return game;
    }

    pub fn process<F>(&mut self, action: Action, callback: F) -> Result<(), Error> where F: FnMut(Event) {
        match action {
            Action::Initialize => { action_initialize(self, callback) }
            Action::Start => { action_start(self, callback) }
            Action::Reset{ seed } => { action_reset(self, seed, callback) }
            Action::Time{ t } => { action_time(self, t, callback) }
            Action::Move{ player, direction } => { action_move(self, player, direction, callback) }
        }
    }

    fn require_initialized(&self) -> Result<(), Error> {
        match self.state {
            State::Uninitialized => { Err(Error::NotInitialized) }
            _ => Ok(())
        }
    }

    fn get_player(&mut self, player: &Player) -> &mut PlayerData {
        match player {
            &Player::Left => { &mut self.left }
            &Player::Right => { &mut self.right }
        }
    }
}

fn action_initialize<F>(game: &mut Game, mut callback: F) -> Result<(), Error> where F: FnMut(Event) {
    callback(Event::Create { id: game.left.id, entity: Entity::LeftPaddle, x: game.left.position.x, y: game.left.position.y });
    callback(Event::Create { id: game.right.id, entity: Entity::RightPaddle, x: game.right.position.x, y: game.right.position.y });
    callback(Event::Create { id: game.ball.id, entity: Entity::Ball, x: game.ball.position.x, y: game.ball.position.y });
    game.state = State::Ready;
    Ok(())
}

fn action_start<F>(game: &mut Game, mut callback: F) -> Result<(), Error> where F: FnMut(Event) {
    try!(game.require_initialized());
    game.state = State::Running;
    callback(Event::RoundStart);
    Ok(())
}

fn action_reset<F>(game: &mut Game, seed: i64, mut callback: F) -> Result<(), Error> where F: FnMut(Event) {
    try!(game.require_initialized());
    game.left.position.y = 0;
    game.right.position.y = 0;
    game.ball.position.x = 0;
    game.ball.position.y = 0;
    game.ball.velocity.x = ((seed % 2) - 1) * 300;
    game.state = State::Ready;
    callback(Event::Move{ id: game.left.id, x: game.left.position.x, y: 0 });
    callback(Event::Move{ id: game.right.id, x: game.right.position.x, y: 0 });
    callback(Event::Move{ id: game.ball.id, x: 0, y: 0 });
    callback(Event::Reset);
    Ok(())
}

fn action_time<F>(game: &mut Game, t: u64, mut callback: F) -> Result<(), Error> where F: FnMut(Event) {
    try!(game.require_initialized());
    match game.state {
        State::Uninitialized => { Err(Error::NotInitialized) }
        State::Ready => {
            game.t = t;
            Ok(())
        }
        State::Running => {
            let frame_time = 1000;
            while t - game.t >= frame_time {
                game.t += frame_time;
                try!(advance_frame(game, &mut callback));
            }
            game.t = t;

            Ok(())
        }
    }
}

fn advance_frame<F>(game: &mut Game, callback: &mut F) -> Result<(), Error> where F: FnMut(Event) {
    {
        let min_paddle_y = game.cfg.paddle.y - game.cfg.area.y;
        let max_paddle_y = game.cfg.area.y - game.cfg.paddle.y;
        let mut players = [&mut game.left, &mut game.right];
        for p in players.iter_mut() {
            p.position += p.velocity;
            p.position.y = clamp(p.position.y, min_paddle_y, max_paddle_y);
        }
    }

    game.ball.position += game.ball.velocity;

    let goal_x = game.cfg.area.x + game.cfg.ball_size;
    let goal = if game.ball.position.x > goal_x {
        Some(Player::Left)
    } else if game.ball.position.x < -goal_x {
        Some(Player::Right)
    } else {
        None
    };

    if let Some(player) = goal {
        game.ball.position = Vector2 { x: 0, y: 0 };
        let score = {
            let player_data = game.get_player(&player);
            player_data.score += 1;
            player_data.score
        };
        game.state = State::Ready;
        callback(Event::Goal{player: player, score: score});
    } else {
        let ball_position_y = reflect(game.ball.position.y, -game.cfg.area.y, game.cfg.area.y);

        if ball_position_y != game.ball.position.y {
            game.ball.velocity.y *= -1;
        }
        game.ball.position.y = ball_position_y;

        let paddle_collision = {
            let diff = game.ball.position - if game.ball.velocity.x < 0 {
                game.left.position
            } else {
                game.right.position
            };

            diff.x.abs() < game.cfg.paddle.x + game.cfg.ball_size
                && diff.y.abs() < game.cfg.paddle.y + game.cfg.ball_size
        };

        if paddle_collision {
            game.ball.velocity.x *= -1;
        }
    }

    callback(Event::Move{ id: game.left.id, x: game.left.position.x, y:  game.left.position.y });
    callback(Event::Move{ id: game.right.id, x: game.right.position.x, y: game.right.position.y });
    callback(Event::Move{ id: game.ball.id, x: game.ball.position.x, y: game.ball.position.y });

    Ok(())
}

fn action_move<F>(game: &mut Game, player: Player, direction: Direction, callback: F) -> Result<(), Error> where F: FnMut(Event) {
    try!(game.require_initialized());
    let p = match player {
        Player::Left => { &mut game.left }
        Player::Right => { &mut game.right }
    };

    p.velocity.y = match direction {
        Direction::Up => { 300 }
        Direction::Neutral => { 0 }
        Direction::Down => { -300 }
    };
    Ok(())
}

fn clamp<T: Ord>(x: T, a: T, b: T) -> T {
    if x < a {
        a
    } else if b < x {
        b
    } else {
        x
    }
}

fn reflect<T: Ord+Copy+Sub<T, Output = T>>(x: T, a: T, b: T) -> T {
    if x < a {
        a - (x - a)
    } else if b < x {
        b - (x - b)
    } else {
        x
    }
}

