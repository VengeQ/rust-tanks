pub mod board_objects; //Сюда не буду относить объекты Area

use super::CELL_COUNT;
use super::types::*;
use crate::model::Area::{Clear, Water};
use crate::model::board_objects::{Player, GameObject, Wall, Nothing};
use std::ops::Deref;


#[derive(Debug)]
pub struct Game {
    board: Board,
    player: Player,
    pub objects: Vec<Vec<Box<dyn GameObject>>>,
}

impl Game {
    pub fn new() -> Self {
        let mut objects = vec![];
        let size: usize = CELL_COUNT;
        for _ in 0..size {
            let mut y: Vec<Box<dyn GameObject>> = Vec::new();
            for _ in 0..size{
                y.push(Game::nothing());
            }
            objects.push(y);
        }

        Game { board: Default::default(), player: Player::new(([0, 0], Direction::Bottom)), objects: objects }
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

        let mut objects: Vec<Vec<Box<dyn GameObject>>> = Vec::new();
        let size: usize = CELL_COUNT;
        for _ in 0..size {
            let mut y: Vec<Box<dyn GameObject>> = Vec::new();
            for _ in 0..size{
                y.push(Game::nothing());
            }
            objects.push(y);
        }
        objects[15][14] = Game::wall();
        objects[14][15] = Game::wall();
        objects[14][14] = Game::wall();
        objects[15][15] = Game::wall();

        for i in objects.iter_mut().take(19).skip(10) {
            i[25] = Game::wall();
        }

        let min = 8_usize;
        let max = 21_usize;

        /*
        objects = objects.iter()
            .enumerate().map(move |v| v.1.iter()
            .enumerate().map(move |x| {
            if (v.0 == min && (x.0 >= min && x.0 <= max)) || (x.0 == min && (v.0 > min && v.0 < max))
                || (v.0 == max && (x.0 >= min && x.0 <= max)) || x.0 == max && (v.0 > min && v.0 < max) {
                water()
            } else {
                x.1.
            }
        }).collect()).collect();
        */
        objects[21][19] = Game::nothing();
       self.objects=objects;

        let board = Board {
            size: [size as f64; 2],
            fields: vec![],
        };
        self.board = board;
    }

    fn wall() -> Box<dyn GameObject> { Box::new(board_objects::Wall::new(Direction::Top)) }
    fn water() -> Box<dyn GameObject> { Box::new(board_objects::Water::new(Direction::Top)) }
    fn nothing() -> Box<dyn GameObject> { Box::new(board_objects::Nothing::new()) }

    ///Check move possibility then move if possible
    pub fn move_in_direction_if_possible(&mut self, direction: Direction) -> bool {
        let prev_location = self.location();
        let new_position = self.get_new_position_or_current_if_board(direction);
        let new_location = (new_position, direction);
        self.player.location = self.return_new_location_if_area_is_clear_or_current(new_location);
        if self.player.location == prev_location { false } else { true }
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
    fn return_new_location_if_area_is_clear_or_current(&self, location: Location) -> Location {
        let (x, y) = (location.0[0], location.0[1]);
        if self.objects[x][y].area() == Area::Clear {
            location
        } else {
            (self.player.location.0, location.1)
        }
    }
}


#[derive(Default, Debug, PartialOrd, PartialEq, Clone)]
pub struct Board {
    size: [f64; 2],
    fields: Vec<Vec<(Area, Direction)>>,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Area {
    Clear,
    Water,
    Wall,
}

impl Area {}

#[allow(dead_code)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}


///ToDo Тесты для функций
#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Direction::Top;

    #[test]
    fn new_game_test() {
        let g = Game::new();
        assert_eq!(g.board.fields, Vec::<Vec<(Area, Direction)>>::new());
        assert_eq!(g.board.size, [0.0; 2]);
    }

    #[test]
    fn lvl1_test() {
        let mut g = Game::new();
        g.lvl1();
        let v: Vec<&(Area, Direction)> = g.board.fields.iter()
            .flat_map(|x|
                x.iter().filter(|c| *c == &(Area::Wall, Direction::Top)))
            .collect();
        assert_eq!(v.len(), 13);
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