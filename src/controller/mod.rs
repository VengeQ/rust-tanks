use crate::Game;
use super::{FSIZE, SIZE};
use piston::input::GenericEvent;

pub struct GameController {
    pub game: super::model::Game,
    pub game_state: GameState,
    cur_board_size: f64,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum GameState {
    Prepare,
    InProcess,
    BetweenLvls,
    GameOver,
}

impl GameController {

    pub fn new(game: Game) -> Self {
        let cur_board_size =  game.cell_size()[0] * FSIZE;
        Self {
            game,
            game_state: GameState::Prepare,
            cur_board_size
        }
    }

    pub fn event<E: GenericEvent>(&mut self, pos: [f64; 2], size: f64, event: &E) {

        if let Some(pos) = event.release_args(){
            dbg!(pos);
        }

        match self.game_state {
            _ => {}
        }
    }
}
