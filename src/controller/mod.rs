use crate::Game;
use super::FSIZE;
use piston::input::{GenericEvent, Button, MouseButton};

pub struct GameController {
    pub game: super::model::Game,
    pub game_state: GameState,
    _cur_board_size: f64,
    cursor_pos: [f64; 2],
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum GameState {
    Prepare,
    InProcess,
    BetweenLvls,
    GameOver,
}

impl GameController {
    pub fn new(game: Game) -> Self {
        let cur_board_size = game.cell_size()[0] * FSIZE;
        Self {
            game,
            game_state: GameState::Prepare,
            _cur_board_size:cur_board_size,
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
                let cell_x = (x / size * 30.0) as usize;
                let cell_y = (y / size * 30.0) as usize;
                dbg!("x:{} y:{}",cell_x,cell_y);
                dbg!("cell:{}",self.game.board()[cell_x][cell_y]);

            }
        }
        match self.game_state {
            _ => {}
        }
    }
}
