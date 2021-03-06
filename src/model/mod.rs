pub mod board_objects;

pub use board_objects::{Area, Direction, GameObjectType};

use super::CELL_COUNT;
use super::types::*;
use crate::model::board_objects::{Player, GameObject};
use std::collections::HashMap;
use std::ops::Deref;
use crate::model::board_objects::GameObjectType::Water;


#[derive(Debug, Default)]
pub struct Game {
    board: Board,
    player: Player,
    objects: HashMap<[usize; 2], Box<dyn GameObject>>,
}

impl Game {
    pub fn new() -> Self {
        let mut cells = Vec::new();
        for _ in 0..CELL_COUNT {
            let y = vec![(Area::Clear, Direction::Top); CELL_COUNT];
            cells.push(y);
        };
        let board = Board { size: [CELL_COUNT as f64; 2], fields: cells };
        let objects = HashMap::new();
        Game { board, player: Player::new(([0, 0], Direction::Bottom)), objects }
    }

    pub fn objects(&self) -> &HashMap<[usize; 2], Box<dyn GameObject>> {
        &self.objects
    }

    /// Return cells of gameboard.
    pub fn board(&self) -> &Vec<Vec<Field>> {
        &self.board.fields
    }

    pub fn lives(&self) -> usize {
        self.player.lives
    }

    pub fn location(&self) -> Location {
        self.player.location
    }

    /// Return size of gameboard.
    pub fn cell_size(&self) -> [f64; 2] {
        self.board.size
    }

    /// Level1
    ///ToDo Invent normal lvl creator
    pub fn lvl1(&mut self) {
        let mut objects: HashMap<[usize; 2], Box<dyn GameObject>> = HashMap::new();
        objects.insert([15, 14], Game::wall());
        objects.insert([15, 15], Game::wall());
        objects.insert([14, 15], Game::wall());
        objects.insert([14, 14], Game::wall());
        for i in 10..19 {
            objects.insert([i, 25], Game::wall());
        }

        let min = 8_usize;
        let max = 21_usize;

        for x in min..=max {
            for y in min..=max {
                if (x == min && (y >= min && y <= max)) || (y == min && (x > min && x < max))
                    || (x == max && (y >= min && y <= max)) || y == max && (x > min && x < max) {
                    objects.insert([x, y], Game::water());
                }
            }
        }

        objects.remove(&[21, 19]).expect("Element 21,19 not found");
        self.objects = objects;
    }

    fn wall() -> Box<dyn GameObject> { Box::new(board_objects::Wall::new(Direction::Top)) }
    fn water() -> Box<dyn GameObject> { Box::new(board_objects::Water::new(Direction::Top)) }

    pub fn put_object(&mut self, object: Box<dyn GameObject>, coordinates: [usize; 2]) {
        if self.objects.get(&coordinates).is_none() && self.player.location.0 != coordinates {
            self.objects.insert(coordinates, object);
        }
        self.objects.iter().filter(|(k, v)| v.game_object() != Water)
            .for_each(|(k, v)| println!("{:?} {:?}", k, v));
    }


    ///Check move possibility then move if possible
    pub fn move_in_direction_if_possible(&mut self, direction: Direction) -> bool {
        let prev_location = self.location();
        let new_position = self.get_new_position_or_current_if_board(direction);
        let new_location = (new_position, direction);
        self.player.location = self.return_new_location_if_area_is_clear_or_current(new_location);
        self.player.location != prev_location
    }
    fn get_new_position_or_current_if_board(&self, direction: Direction) -> [usize; 2] {
        let src = self.player.location.0;
        let (x, y) = (src[0], src[1]);
        match direction {
            Direction::Top => if y > 0 { [x, y - 1] } else { [x, y] },
            Direction::Right => if x < CELL_COUNT - 1 { [x + 1, y] } else { [x, y] },
            Direction::Bottom => if y < CELL_COUNT - 1 { [x, y + 1] } else { [x, y] },
            Direction::Left => if x > 0 { [x - 1, y] } else { [x, y] },
        }
    }
    fn return_new_location_if_area_is_clear_or_current(&mut self, location: Location) -> Location {
        let (x, y) = (location.0[0], location.0[1]);
        match self.objects.get(&[x, y]) {
            Some(g) if !g.can_pick() => (self.player.location.0, location.1),
            Some(g) => {
                self.player.pick(g);
                &self.objects.remove(&[x, y]);
                location
            }
            None => location
        }
    }
}


#[derive(Default, Debug, PartialOrd, PartialEq, Clone)]
pub struct Board {
    size: [f64; 2],
    fields: Vec<Vec<(Area, Direction)>>,
}


///ToDo Тесты для функций
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn objects_test() {
        let g = Game::new();
        for i in 0..CELL_COUNT {
            for j in 0..CELL_COUNT {
                assert_eq!(g.objects().get(&[i, j]).is_none(), true);
            }
        }
    }

    #[test]
    fn new_game_test() {
        let g = Game::new();
        assert_eq!(g.board.size, [30.0; 2]);
    }

    #[test]
    fn lvl1_test() {
        let mut g = Game::new();
        g.lvl1();//: Vec<&Box<board_objects::GameObject>>
        let v = g.objects.values().filter(|c| c.game_object() == GameObjectType::Wall).count();

        assert_eq!(v, 13);
    }

    #[test]
    fn return_new_location_if_area_is_clear_or_current() {
        let mut g = Game::new();
        g.lvl1();
        let pos: Location = ([14, 14], Direction::Top);
        g.player.location = pos;
        let try_to_water = g.return_new_location_if_area_is_clear_or_current(pos);
        assert_eq!(try_to_water, pos);
        let try_correct_move = g.return_new_location_if_area_is_clear_or_current(([11, 13], Direction::Top));
        assert_eq!(try_correct_move, ([11, 13], Direction::Top));
    }

    #[test]
    fn move_in_direction_if_possible_test() {
        let mut g = Game::new();
        g.lvl1();
        let location = ([29, 29], Direction::Right);
        g.player.location = location;
        assert_eq!(g.move_in_direction_if_possible(Direction::Right), false);
        assert_eq!(g.location(), location);
        assert_eq!(g.move_in_direction_if_possible(Direction::Left), true);
        assert_eq!(g.location(), ([28, 29], Direction::Left));
    }
}