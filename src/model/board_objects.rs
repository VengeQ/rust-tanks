use crate::types::*;
use std::fmt;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Area {
    Clear
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum GameObjectType{
    Water,
    Wall,
    Live
}

#[allow(dead_code)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

impl Default for  Direction{
    fn default() -> Self {
        Direction::Top
    }
}

#[derive(Debug, Clone,Default)]
pub struct Player {
    pub location: Location,
    pub lives: usize,
    pub state: State,
}

#[allow(dead_code)]
impl Player {
    pub fn new(location: Location) -> Self {
        Self {
            location,
            lives: 3,
            state: State {},
        }
    }

    fn add_live(&mut self) {
        self.lives += 1;
    }

    ///ToDo Game is over!
    fn drop_live(&mut self) {
        if self.lives > 1 {
            self.lives -= 1
        } else {}
    }

    pub fn get_live(&mut self){
        self.add_live();

    }
}

#[derive(Debug, Clone,Default)]
pub struct State {}

pub trait GameObject {
    fn direction(&self) -> Direction;
    fn game_object(&self) -> GameObjectType;
    fn clone_object(&self) ->Box<dyn GameObject>{
        match self.game_object(){
            GameObjectType::Wall => Box::new(Wall::new(self.direction())),
            GameObjectType::Water => Box::new(Water::new(self.direction())),
            GameObjectType::Live => Box::new(Live::new()),
        }
    }
}

impl fmt::Display for dyn GameObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?})", self.game_object())
    }
}

impl fmt::Debug for dyn GameObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?})", self.game_object())
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Water {
    direction: Direction
}

impl GameObject for Water {
    fn direction(&self) -> Direction {
        self.direction
    }

    fn game_object(&self) -> GameObjectType {
        GameObjectType::Water
    }
}

impl Water {
    pub fn new(direction: Direction) -> Self {
        Self { direction }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Wall {
    direction: Direction,
    durability: usize,
}

impl GameObject for Wall {
    fn direction(&self) -> Direction {
        self.direction
    }

    fn game_object(&self) -> GameObjectType {
        GameObjectType::Wall
    }
}

impl Wall {
    pub fn new(direction: Direction) -> Self {
        let durability = 2;
        Self { direction, durability }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Live{
    duration_left:isize,
    direction:Direction
}

impl GameObject for Live {
    fn direction(&self) -> Direction {
        self.direction
    }

    fn game_object(&self) -> GameObjectType {
        GameObjectType::Live
    }
}

impl Live {
    pub fn new() -> Self{
        Self{ duration_left: 1000, direction: Default::default() }
    }
    pub fn lost_duration(&mut self){
        self.duration_left-=1;
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Direction;
    use crate::CELL_COUNT;

    #[test]
    fn new_player_test() {
        for x in 0..CELL_COUNT {
            for y in 0..CELL_COUNT {
                let location = ([x, y], Direction::Top);
                let player = Player::new(location);

                let position = [x, y];
                let start_lives = 3;

                assert_eq!(player.location.0, position);
                assert_eq!(player.lives, start_lives);
            }
        }
    }

    #[test]
    fn add_live_test() {
        let location = ([1, 1], Direction::Top);
        let mut player = Player::new(location);
        player.add_live();
        let start_lives = 3_usize;

        assert_eq!(player.lives, start_lives + 1)
    }

    #[test]
    fn drop_live_test() {
        let location = ([1, 1], Direction::Top);
        let mut player = Player::new(location);
        player.drop_live();
        let start_lives = 3_usize;
        assert_eq!(player.lives, start_lives - 1)
    }
}
