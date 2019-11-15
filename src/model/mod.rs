use super::{FSIZE, SIZE};

#[derive(Default, Debug)]
pub struct Game {
    board: Board,
}

impl Game {
    pub fn new() -> Self {
        Game { board: Default::default() }
    }
    pub fn lvl1(&mut self) {
        let mut cells = Vec::new();
        let size: usize = 20;
        let y = vec![Cell::Clear; size];
        for x in 0..size {
            cells.push(y.clone());
        }
        cells[10][10] = Cell::Wall;
        cells[9][10] = Cell::Wall;
        cells[10][9] = Cell::Wall;
        cells[9][9] = Cell::Wall;
        let board = Board {
            size: [size as f64; 2],
            cell: cells,
        };
        self.board = board;
    }
}

#[derive(Default, Debug, PartialOrd, PartialEq)]
struct Board {
    size: [f64; 2],
    cell: Vec<Vec<(Cell)>>,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Cell {
    Clear,
    Water,
    Wall,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
enum Orientation {
    Top,
    Right,
    Bottom,
    Left,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_game_test() {
        let g = Game::new();
        assert_eq!(g.board.cell, Vec::<Vec<Cell>>::new());
        assert_eq!(g.board.size, [0.0; 2]);
    }

    #[test]
    fn lvl1_test(){
        let mut g = Game::new();
        g.lvl1();
        let v:Vec<&Cell>=g.board.cell.iter()
            .flat_map(|x|
                x.iter().filter(|c|*c == &Cell::Wall))
            .collect();
        assert_eq!(v.len(),4);
    }
}