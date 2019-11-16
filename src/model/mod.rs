use super::CELL_COUNT;

#[derive(Default, Debug)]
pub struct Game {
    board: Board,
}

impl Game {
    pub fn new() -> Self {
        Game { board: Default::default() }
    }

    pub fn board(&self) -> &Vec<Vec<Cell>> {
        &self.board.cell
    }

    pub fn cell_size(&self) -> [f64; 2] {
        self.board.size
    }

    pub fn lvl1(&mut self) {
        let mut cells = Vec::new();
        let size: usize = CELL_COUNT;
        let y = vec![Cell::Clear; size];
        for _ in 0..size {
            cells.push(y.clone());
        }
        cells[15][14] = Cell::Wall;
        cells[14][15] = Cell::Wall;
        cells[14][14] = Cell::Wall;
        cells[15][15] = Cell::Wall;

        for i in 10..19{
            cells[i][25] = Cell::Wall;
        }

        let min = 8_usize;
        let max = 21_usize;

        cells = cells.iter()
            .enumerate().map(move |v| v.1.iter()
            .enumerate().map(move |x| {
            if (v.0 == min && (x.0 >= min && x.0 <= max)) || (x.0 == min && (v.0 > min && v.0 < max))
                || (v.0 == max && (x.0 >= min && x.0 <= max)) || x.0 == max && (v.0 > min && v.0 < max) {
                Cell::Water
            } else {
                *x.1
            }
        }).collect()).collect();

        let board = Board {
            size: [size as f64; 2],
            cell: cells,
        };
        self.board = board;
    }
}

#[derive(Default, Debug, PartialOrd, PartialEq)]
pub struct Board {
    size: [f64; 2],
    cell: Vec<Vec<Cell>>,
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
    fn lvl1_test() {
        let mut g = Game::new();
        g.lvl1();
        let v: Vec<&Cell> = g.board.cell.iter()
            .flat_map(|x|
                x.iter().filter(|c| *c == &Cell::Wall))
            .collect();
        assert_eq!(v.len(), 4);
    }
}