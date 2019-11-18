use crate::Game;
use super::{FSIZE,CELL_COUNT};
use piston::input::{GenericEvent, Button, MouseButton};
use crate::model::Orientation;

pub struct GameController {
    pub game: super::model::Game,
    pub game_state: GameState,
    pub position:([usize;2],Orientation),
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


impl GameController {
    pub fn new(game: Game) -> Self {
        let cur_board_size = game.cell_size()[0] * FSIZE;
        Self {
            game,
            game_state: GameState::Prepare,
            position: ([1,1], Orientation::Top),
            cursor_pos: [0_f64; 2],
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
        match self.game_state {
            _ => {}
        }
    }
}
