use crate::Game;
use super::CELL_COUNT;
use piston::input::{GenericEvent, Button, MouseButton, Key};
use crate::model::{Direction};
use super::types::*;
use crate::model::board_objects::GameObject;
use std::collections::HashMap;


#[derive(Debug)]
pub struct GameController {
    pub game: super::model::Game,
    game_state: GameState,
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

impl GameController {
    pub fn new(game: Game) -> Self {
        Self {
            game,
            game_state: GameState::Prepare,
            cursor_pos: [0_f64; 2],
        }
    }

    pub fn game_state(&self) -> GameState {
        self.game_state
    }

    //move player tank if possible
    fn move_tank(&mut self, direction: Direction) {
        self.game.move_in_direction_if_possible(direction);
    }

    //Данные объекты необходимы вьюшке для отрисовки
    pub fn player_location(& self) -> Location{
        self.game.location()
    }
    pub fn gameboard_field(&self, xy: [usize; 2]) -> Field {
        self.game.board()[xy[0]][xy[1]]
    }
    pub fn objects(&self) -> &HashMap<[usize; 2], Box<dyn GameObject>>{
        self.game.objects()
    }

    #[allow(unused_variables)]
    pub fn event<E: GenericEvent>(&mut self, pos: [f64; 2], size: f64, event: &E) {
        if let Some(pos) = event.mouse_cursor_args() {
            self.cursor_pos = pos;
        }
        if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
            let x = self.cursor_pos[0] - pos[0];
            let y = self.cursor_pos[1] - pos[1];
            if x >= 0.0 && x < size && y >= 0.0 && y < size {
                let cell_x = (x / size * CELL_COUNT as f64) as usize;
                let cell_y = (y / size * CELL_COUNT as f64) as usize;
                dbg!("x:{} y:{}", cell_x, cell_y);
                dbg!("cell:{}", self.game.board()[cell_x][cell_y]);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Area, GameObjectType};

    #[test]
    fn player_location_test(){
        let mut game = Game::new();
        game.lvl1();
        let location=game.location();
        let game_controller = GameController::new(game);

        assert_eq!(game_controller.player_location(),location)
    }

    #[test]
    fn gameboard_field_test(){
        let mut game = Game::new();
        game.lvl1();
        let game_controller = GameController::new(game);
        for i in 0..30{
            for j in 0..30{
                assert_eq!(game_controller.gameboard_field([i,j]),(Area::Clear, Direction::Top))
            }
        }
    }

    #[test]
    fn objects_test(){
        let mut game = Game::new();
        game.lvl1();
        let game_controller = GameController::new(game);
        let object = game_controller.objects().get(&[14,14]).unwrap();
        assert_eq!(object.game_object(),GameObjectType::Wall);
    }

    #[test]
    fn game_state_test(){
        let game_controller = GameController::new(Game::new());
        assert_eq!(game_controller.game_state, GameState::Prepare);

    }
}