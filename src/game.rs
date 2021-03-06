use std::cell;

use wasm_bindgen::prelude::*;
use js_sys;
use log;
use crate::coord::{Coord, Direction};
use crate::draw::Draw;
use crate::field::{Field, CellType};
use crate::snake::Snake;
use web_sys::console::debug;
use log::logger;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

/// Main game context
#[wasm_bindgen]
pub struct Game {
    draw: Draw,
    field: Field,
    snake: Snake,
    delay: f32,
    cnt: i32,
    speed_up: f32,
    is_died: bool
}

fn get_direction(e: web_sys::KeyboardEvent) -> Option<Direction> {
    match e.key().as_str() {
        "ArrowUp" => Some(Direction::Up),
        "ArrowDown" => Some(Direction::Down),
        "ArrowLeft" => Some(Direction::Left),
        "ArrowRight" => Some(Direction::Right),
        _ => None,
    }
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Game {
        log::info!("Game initialized");
        let mut game = Game {
            field: Field::new(30, 30),
            draw: Draw::new("canvas_game"),
            snake: Snake::new(),
            delay: 15.0,
            speed_up: 0.5,
            cnt: 0,
            is_died: false
        };

        game.snake.add_to_field(&mut game.field);
        game.place_food();
        game.draw.draw(&game.field);

        return game;
    }

    // Key is pressed. External event from JS
    pub fn key_down(&mut self, e: web_sys::KeyboardEvent) {
        match get_direction(e) {
            Some(direction) => {
                self.snake.rotate(direction);
                self._update();
            },
            _ => return
        }
    }

    // Game tick. External event from JS
    pub fn update(&mut self) {
        if !self.skip() {
            self._update();
        }
    }

    fn skip(&mut self) -> bool {
        self.cnt += 1;
        if !self.is_died && self.cnt >= self.delay as i32 {
            self.cnt = 0;
            return false;
        }
        return true
    }

    fn _update(&mut self) {
        if self._step().is_none() {
            self.is_died = true;
            log::info!("Died");
            alert("You died!");
        }
        self.draw.draw(&self.field);
    }

    fn _step(&mut self) -> Option<()> {
        if self.snake.check_food(&self.field)? {
            log::info!("Eaten");
            self.snake.increase(&mut self.field)?;
            self.speed_up();
            self.place_food();
        } else {
            self.snake.shift(&mut self.field)?;
        }
        Some(())
    }

    fn speed_up(&mut self) {
        self.delay -= self.speed_up;
        if self.delay < 0.0 { self. delay = 0.0;  }
    }

    fn place_food(&mut self) {
        let mut coord = self.get_random_coord();
        //log::debug!("{:?}", coord);
        while self.field[coord] == CellType::Snake || self.field[coord] == CellType::Head {
            coord = self.get_random_coord();
            //log::debug!("{:?}", coord);
        }
        self.field[coord] = CellType::Food;
    }

    fn get_random_coord(&self) -> Coord {
        // fuck rust
        Coord {
            x: (js_sys::Math::random() * self.field.width as f64) as i32,
            y: (js_sys::Math::random() * self.field.height as f64) as i32,
        }
    }
}
