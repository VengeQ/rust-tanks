use crate::types::*;
use crate::CELL_COUNT;

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

    fn add_live(&mut self){
        self.lives+=1;
    }

    ///ToDo Game is over!
    fn drop_live(&mut self){
        if self.lives>1 {
            self.lives -= 1
        } else {

        }
    }

}

#[derive(Debug, Clone)]
pub struct State {}

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
    fn add_live_test(){
        let location = ([1, 1], Direction::Top);
        let mut player = Player::new(location);
        player.add_live();
        let start_lives = 3_usize;

        assert_eq!(player.lives, start_lives+1)
    }

    #[test]
    fn drop_live_test(){
        let location = ([1, 1], Direction::Top);
        let mut player = Player::new(location);
        player.drop_live();
        let start_lives = 3_usize;
        assert_eq!(player.lives, start_lives-1)
    }

}