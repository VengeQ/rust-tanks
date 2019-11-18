use crate::Game;
use super::{FSIZE, CELL_COUNT};
use piston::input::{GenericEvent, Button, MouseButton, Key};
use crate::model::{Orientation, Cell};

pub struct GameController {
    pub game: super::model::Game,
    pub game_state: GameState,
    pub position: ([usize; 2], Orientation),
    cursor_pos: [f64; 2],
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum GameState {
    Prepare,
    InProcess,
    EndLevel,
    GameOver,
}

pub struct EndLevel(usize);

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Direction{
    Top,
    Right,
    Bottom,
    Left
}

impl GameController {
    pub fn new(game: Game) -> Self {
        let cur_board_size = game.cell_size()[0] * FSIZE;
        Self {
            game,
            game_state: GameState::Prepare,
            position: ([0, 0], Orientation::Bottom),
            cursor_pos: [0_f64; 2],
        }
    }
    //return new position
    fn move_tank(&mut self, pos: [f64; 2], direction: Direction) -> ([usize; 2], Orientation) {
        let (x, y) = (self.position.0[0], self.position.0[1]);

        dbg!("current: {} {}", x,y);

        let try_position = match direction {
            Direction::Top => if y>0 {[x, y - 1]} else {[x,y]},
            Direction::Right => if x< CELL_COUNT-1 {[x+1, y]} else{ [x,y] },
            Direction::Bottom => if y< CELL_COUNT-1 {[x, y + 1]} else{ [x,y] },
            Direction::Left => if x>0 {[x-1, y]} else {[x,y]},
        };
        dbg!("next: {} {}", try_position[0],try_position[1]);
        if self.game.board()[try_position[0] as usize][try_position[1] as usize].0 == Cell::Clear {
            ([try_position[0] as usize,try_position[1] as usize], match direction{
                Direction::Top => {Orientation::Top},
                Direction::Right => {Orientation::Right},
                Direction::Bottom => {Orientation::Bottom},
                Direction::Left => {Orientation::Left},
            })
        } else {
            (self.position.0, self.position.1)
        }
    }

    #[allow(unused_variables)]
    pub fn event<E: GenericEvent>(&mut self, pos: [f64; 2], size: f64, event: &E) {
        if let Some(pos) = event.mouse_cursor_args() {
            //dbg!(pos);
            self.cursor_pos = pos;
        }

        if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
            let x = self.cursor_pos[0] - pos[0];
            let y = self.cursor_pos[1] - pos[1];
            if x >= 0.0 && x < size && y >= 0.0 && y < size {
                let cell_x = (x / size * CELL_COUNT as f64) as usize;
                let cell_y = (y / size * CELL_COUNT as f64) as usize;
                dbg!("x:{} y:{}",cell_x,cell_y);
                dbg!("cell:{}",self.game.board()[cell_x][cell_y]);
            }
        }


        if let Some(Button::Keyboard(Key::Left)) = event.press_args() {
            self.position = self.move_tank(pos,Direction::Left);
        }
        if let Some(Button::Keyboard(Key::Right)) = event.press_args() {
            self.position = self.move_tank(pos,Direction::Right);
        }
        if let Some(Button::Keyboard(Key::Down)) = event.press_args() {
            self.position = self.move_tank(pos,Direction::Bottom);
        }
        if let Some(Button::Keyboard(Key::Up)) = event.press_args() {
            self.position = self.move_tank(pos,Direction::Top);
        }
        match self.game_state {
            _ => {}
        }
    }
}
