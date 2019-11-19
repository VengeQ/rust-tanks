use crate::Game;
use super::CELL_COUNT;
use piston::input::{GenericEvent, Button, MouseButton, Key};
use crate::model::Direction;

#[derive(Clone, Debug)]
pub struct GameController {
    pub game: super::model::Game,
    pub game_state: GameState,
    pub position: ([usize; 2], Direction),
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

#[allow(dead_code)]
pub struct EndLevel(usize);

///ToDo Separate 'position' to 'position' and 'orientation'
impl GameController {
    pub fn new(game: Game) -> Self {
        Self {
            game,
            game_state: GameState::Prepare,
            position: ([0, 0], Direction::Bottom),
            cursor_pos: [0_f64; 2],
        }
    }
    //return new position or previous if new position incorrect
    fn move_tank(&mut self, direction: Direction) {
        self.position = (self.game.move_from_cell_with_direction(self.position.0, direction), direction);
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
            self.move_tank(Direction::Left);
        }
        if let Some(Button::Keyboard(Key::Right)) = event.press_args() {
            self.move_tank(Direction::Right);
        }
        if let Some(Button::Keyboard(Key::Down)) = event.press_args() {
            self.move_tank(Direction::Bottom);
        }
        if let Some(Button::Keyboard(Key::Up)) = event.press_args() {
            self.move_tank(Direction::Top);
        }
        match self.game_state {
            _ => {}
        }
    }
}
