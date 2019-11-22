use crate::types::*;
use crate::CELL_COUNT;
use std::fmt;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Area {
    Clear,
    Water,
    Wall,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Debug, Clone)]
pub struct Player {
    pub location: Location,
    pub lives: usize,
    pub state: State,
}

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
}

#[derive(Debug, Clone)]
pub struct State {}

pub trait GameObject {
    fn direction(&self) -> Direction;
    fn area(&self) -> Area;
    fn clone_object(&self) ->Box<dyn GameObject>{
        match self.area(){
            Area::Clear =>Box::new(Nothing::new()),
            Area::Wall => Box::new(Wall::new(self.direction())),
            Area::Water => Box::new(Water::new(self.direction())),
        }
    }
}

impl fmt::Display for dyn GameObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?})", self.area())
    }
}

impl fmt::Debug for dyn GameObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?})", self.area())
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

    fn area(&self) -> Area {
        Area::Water
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
    state: usize,
}

impl GameObject for Wall {
    fn direction(&self) -> Direction {
        self.direction
    }

    fn area(&self) -> Area {
        Area::Wall
    }
}

impl Wall {
    pub fn new(direction: Direction) -> Self {
        let state = 2;
        Self { direction, state }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Nothing {
}

impl GameObject for Nothing {
    fn direction(&self) -> Direction {
        Direction::Top
    }

    fn area(&self) -> Area {
        Area::Clear
    }
}

impl Nothing {
    pub fn new() -> Self {
        Self{}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Direction;

    #[test]
    fn new_player_test() {
        for x in 0..CELL_COUNT {
            for y in 0..CELL_COUNT {
                let location = ([x, y], Direction::Top);
                let player = Player::new(location);

                let position = [x, y];
                let start_lives = 3;

                assert_eq!(player.location.0, position);
                assert_eq!(player.lives, 3);
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
