#![allow(non_snake_case)]

extern crate pong;
extern crate cgmath;

#[macro_use]
extern crate qml;

use cgmath::{ Vector2 };
use qml::*;
use pong::*;

const CFG: GameConfiguration = GameConfiguration {
    area: Vector2 {
        x: 600000,
        y: 400000,
    },
    paddle: Vector2 {
        x: 10000,
        y: 50000,
    },
    ball_size: 10000
};

pub struct Pong {
    game: Game
}

impl Default for Pong {
    fn default() -> Self {
        Pong { game: Game::new(CFG.clone()) }
    }
}

Q_OBJECT!(
pub Pong as QPong {
    signals:
        // Relative coordinates to simplify QML side
        fn createLeft(id: i32, x: f64, y: f64);
        fn createRight(id: i32, x: f64, y: f64);
        fn createBall(id: i32, x: f64, y: f64);
        fn destroyEntity(id: i32);
        fn moveEntity(id: i32, x: f64, y: f64);
        fn goalLeft(score: i32);
        fn goalRight(score: i32);
        fn reseted();
        fn roundStart();
    slots:
        fn initialize();
        fn start();
        fn reset(seed: i32);
        fn time(t: i32);
        // TODO: Simulate enums instead of mapping all combinations?
        fn moveLeftUp();
        fn moveLeftDown();
        fn moveLeftStop();
        fn moveRightUp();
        fn moveRightDown();
        fn moveRightStop();
    properties:
        paddleRelativeWidth: f64; read: get_paddle_relative_width, write: set_paddle_relative_width, notify: paddle_relative_width_changed;
        paddleRelativeHeight: f64; read: get_paddle_relative_height, write: set_paddle_relative_height, notify: paddle_relative_height_changed;
        ballRelativeWidth: f64; read: get_ball_relative_width, write: set_ball_relative_width, notify: ball_relative_width_changed;
        ballRelativeHeight: f64; read: get_ball_relative_height, write: set_ball_relative_height, notify: ball_relative_height_changed;
});

impl QPong {
    fn handle_event(&mut self, e: Event) {
        fn to_rel_x(x: i64) -> f64 {
            (x as f64) / (CFG.area.x as f64)
        }
        fn to_rel_y(y: i64) -> f64 {
            (y as f64) / (CFG.area.y as f64)
        }
        match e {
            // FIXME: Ugly forced pong->qml casts galore
            Event::Create { id, entity, x, y } => {
                match entity {
                    Entity::LeftPaddle => { self.createLeft(id as i32, to_rel_x(x), to_rel_y(y)) }
                    Entity::RightPaddle => { self.createRight(id as i32, to_rel_x(x), to_rel_y(y)) }
                    Entity::Ball => { self.createBall(id as i32, to_rel_x(x), to_rel_y(y)) }
                };
            }
            Event::Destroy { id } => {
                self.destroyEntity(id as i32);
            }
            Event::Move { id, x, y } => {
                self.moveEntity(id as i32, to_rel_x(x), to_rel_y(y));
            }
            Event::Goal { player, score } => {
                match player {
                    Player::Left => { self.goalLeft(score as i32); }
                    Player::Right => { self.goalRight(score as i32); }
                };
            }
            Event::Reset => {
                self.reseted();
            }
            Event::RoundStart => {
                self.roundStart();
            }
        }
    }
    fn process(&mut self, action: Action) {
        let mut events = Vec::new();
        self.game.process(action, |e| events.push(e)).unwrap();
        for e in events {
            self.handle_event(e);
        }
    }

    fn initialize(&mut self) -> Option<&QVariant>{
        // TODO: Better place for property initializations?
        self.set_paddle_relative_width((CFG.paddle.x as f64) / (CFG.area.x as f64));
        self.set_paddle_relative_height((CFG.paddle.y as f64) / (CFG.area.y as f64));
        self.set_ball_relative_width((CFG.ball_size as f64) / (CFG.area.x as f64));
        self.set_ball_relative_height((CFG.ball_size as f64) / (CFG.area.y as f64));

        self.process(Action::Initialize);
        None
    }
    fn start(&mut self) -> Option<&QVariant> {
        self.process(Action::Start);
        None
    }
    fn reset(&mut self, seed: i32) -> Option<&QVariant> {
        self.process(Action::Reset{seed: seed as i64});
        None
    }
    fn time(&mut self, t: i32) -> Option<&QVariant> {
        self.process(Action::Time{t: t as u64});
        None
    }
    fn moveLeftUp(&mut self) -> Option<&QVariant> {
        self.process(Action::Move{player: Player::Left, direction: Direction::Up});
        None
    }
    fn moveLeftDown(&mut self) -> Option<&QVariant> {
        self.process(Action::Move{player: Player::Left, direction: Direction::Down});
        None
    }
    fn moveLeftStop(&mut self) -> Option<&QVariant> {
        self.process(Action::Move{player: Player::Left, direction: Direction::Neutral});
        None
    }
    fn moveRightUp(&mut self) -> Option<&QVariant> {
        self.process(Action::Move{player: Player::Right, direction: Direction::Up});
        None
    }
    fn moveRightDown(&mut self) -> Option<&QVariant> {
        self.process(Action::Move{player: Player::Right, direction: Direction::Down});
        None
    }
    fn moveRightStop(&mut self) -> Option<&QVariant> {
        self.process(Action::Move{player: Player::Right, direction: Direction::Neutral});
        None
    }
}

Q_REGISTERABLE_QML!(QPong: Pong as PongGame 1=>0, from Pong);

fn main() {
    let mut qqae = QmlEngine::new();
    Q_REGISTER_QML!(QPong);
    qqae.load_file("res/qml/pong.qml");
    qqae.exec();
    qqae.quit();
}
