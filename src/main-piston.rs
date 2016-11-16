extern crate pong;

use pong::*;

extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate cgmath;

use piston::window::WindowSettings;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use piston::event_loop::*;
use piston::input::{UpdateArgs, RenderArgs, Key, Button, RenderEvent, UpdateEvent, PressEvent, ReleaseEvent};
use opengl_graphics::glyph_cache::GlyphCache;
use std::path::Path;
use cgmath::{ Vector2 };
use std::collections::HashMap;

const FONT_PATH: &'static str = "res/ttf/DejaVuSans.ttf";

const BACKGROUND_COLOR: [f32; 4] = [0.1, 0.1, 0.1, 1.0];
const PADDLE_COLOR: [f32; 4] = [0.8, 0.8, 0.8, 1.0];
const BALL_COLOR: [f32; 4] = [0.9, 0.9, 0.9, 1.0];
const TEXT_COLOR: [f32; 4] = [0.5, 0.5, 0.5, 1.0];

const WINDOW_SIZE: Vector2<i64> = Vector2 {
    x: 800,
    y: 480
};

const GAME_WINDOW_RATIO: Vector2<i64> = Vector2 {
    x: 1000,
    y: 1000
};

const CFG: GameConfiguration = GameConfiguration {
    area: Vector2 {
        x: WINDOW_SIZE.x * GAME_WINDOW_RATIO.x/2,
        y: WINDOW_SIZE.y * GAME_WINDOW_RATIO.y/2
    },
    paddle: Vector2 {
        x: WINDOW_SIZE.x * GAME_WINDOW_RATIO.x / 100,
        y: WINDOW_SIZE.y * GAME_WINDOW_RATIO.y / 16
    },
    ball_size: WINDOW_SIZE.x * GAME_WINDOW_RATIO.x / 100
};

struct Sprite {
    entity: Entity,
    pos: Vector2<f64>
}

pub struct App<'a> {
    gl: GlGraphics,
    glyph_cache: GlyphCache<'a>,
    sprites: HashMap<u64, Sprite>,
    left_score: u8,
    right_score: u8,
    t: u64,
}

impl <'a>App<'a> {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        let gl = &mut self.gl;
        let sprites = &self.sprites;
        let character_cache = &mut self.glyph_cache;
        let left_score_string = self.left_score.to_string();
        let right_score_string = self.right_score.to_string();

        gl.draw(args.viewport(), |c, gl| {
            clear(BACKGROUND_COLOR, gl);

            for (_, s) in sprites {
               s.draw(&c, gl); 
            }

            let text = text::Text::new_color(TEXT_COLOR, 20);
            text.draw(&left_score_string, character_cache,
                      &c.draw_state, c.transform.trans(5.0, 20.0), gl);
            text.draw(&right_score_string, character_cache,
                      &c.draw_state, c.transform.trans(WINDOW_SIZE.x as f64 - 20.0, 20.0), gl);
        });

    }

    fn update(&mut self, args: &UpdateArgs, game: &mut Game) {
        self.t += (args.dt * 1000000.0) as u64;
        game.process(Action::Time{t: self.t}, |e| self.handle_event(e)).unwrap();
    }

    fn control(&mut self, button: Button, pressed: bool, game: &mut Game) {
        if pressed {
            match button {
        
                Button::Keyboard(Key::Up) => {
                    game.process(Action::Move {
                        player: Player::Right,
                        direction: Direction::Up },
                        |e| self.handle_event(e)).unwrap();
                }
                Button::Keyboard(Key::Down) => {
                    game.process(Action::Move {
                        player: Player::Right,
                        direction: Direction::Down },
                        |e| self.handle_event(e)).unwrap();
                }
                Button::Keyboard(Key::A) => {
                    game.process(Action::Move {
                        player: Player::Left,
                        direction: Direction::Up },
                        |e| self.handle_event(e)).unwrap();
                }
                Button::Keyboard(Key::Z) => {
                    game.process(Action::Move {
                        player: Player::Left,
                        direction: Direction::Down },
                        |e| self.handle_event(e)).unwrap();
                }
                Button::Keyboard(Key::R) => {
                    game.process(Action::Reset { seed: 0 },
                        |e| self.handle_event(e)).unwrap();
                }
                Button::Keyboard(Key::Space) => {
                    game.process(Action::Start,
                        |e| self.handle_event(e)).unwrap();
                }
                _ => ()
            }
        } else {
            match button {
                Button::Keyboard(Key::Up) => {
                    game.process(Action::Move {
                        player: Player::Right,
                        direction: Direction::Neutral },
                        |e| self.handle_event(e)).unwrap();
                }
                Button::Keyboard(Key::Down) => {
                    game.process(Action::Move {
                        player: Player::Right,
                        direction: Direction::Neutral },
                        |e| self.handle_event(e)).unwrap();
                }
                Button::Keyboard(Key::A) => {
                    game.process(Action::Move {
                        player: Player::Left,
                        direction: Direction::Neutral },
                        |e| self.handle_event(e)).unwrap();
                }
                Button::Keyboard(Key::Z) => {
                    game.process(Action::Move {
                        player: Player::Left,
                        direction: Direction::Neutral },
                        |e| self.handle_event(e)).unwrap();
                }
                _ => ()
            }
        }
    }
    fn handle_event(&mut self, e: Event) {
        fn to_screen_pos(x: i64, y: i64) -> Vector2<f64> {
            Vector2 {
                x: (x / GAME_WINDOW_RATIO.x + WINDOW_SIZE.x / 2) as f64,
                y: (-y / GAME_WINDOW_RATIO.y + WINDOW_SIZE.y / 2) as f64
            }
        }
        match e {
            Event::Create { id, entity, x, y } => {
                self.sprites.insert(id, Sprite {
                    entity: entity,
                    pos: to_screen_pos(x, y)
                });
            }
            Event::Destroy { id } => {
                self.sprites.remove(&id);
            }
            Event::Move { id, x, y } => {
                let s = self.sprites.get_mut(&id).unwrap();
                s.pos = to_screen_pos(x, y);
            }
            Event::Goal { player, score } => {
                match player {
                    Player::Left => { self.left_score = score; }
                    Player::Right => { self.right_score = score; }
                };
            }
            Event::Reset => {
                self.left_score = 0;
                self.right_score = 0;
            }
            Event::RoundStart => {

            }
        }
    }

}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("rust-pong", [800, 480])
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    if let Ok(glyph_cache) = GlyphCache::new(Path::new(FONT_PATH)) {

        let mut app = App {
            gl: GlGraphics::new(opengl),
            glyph_cache: glyph_cache,
            sprites: HashMap::new(),
            left_score: 0,
            right_score: 0,
            t: 0
        };

        let mut game = Game::new(CFG.clone());
        game.process(Action::Initialize, |e| app.handle_event(e)).unwrap();

        let mut events = window.events();
        while let Some(e) = events.next(&mut window) {
            if let Some(r) = e.render_args() {
                app.render(&r);
            }

            else if let Some(u) = e.update_args() {
                app.update(&u, &mut game);
            }

            else if let Some(button) = e.press_args() {
                app.control(button, true, &mut game);
            }

            else if let Some(button) = e.release_args() {
                app.control(button, false, &mut game);
            }
        }
    } else {
        println!("Could not load font at {}", FONT_PATH);
    }
}

impl Sprite {
    fn draw(&self, c: &graphics::Context, gl: &mut GlGraphics) {
        use graphics::*;
        match self.entity {
            
            Entity::Ball => {
                let rect = rectangle::centered_square(self.pos.x, self.pos.y,
                                                      (CFG.ball_size/1000) as f64);
                rectangle(BALL_COLOR, rect, c.transform, gl);
            }
            Entity::LeftPaddle | Entity::RightPaddle => {
                let rect = rectangle::centered([self.pos.x, self.pos.y,
                                               (CFG.paddle.x/1000) as f64,
                                               (CFG.paddle.y/1000) as f64]);
                rectangle(PADDLE_COLOR, rect, c.transform, gl);
            }
        };
    }
}
