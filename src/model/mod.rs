use super::CELL_COUNT;


///ToDo position из GameController все-таки относится к игре и ее состоянию, да оно меняется
/// при перехвате событий, но правильно его отнести сюда.
/// Кроме того стоит разбить ее на position и orientation, либо использовать
/// отдельную struct или type. Стоит подумать.
#[derive(Default, Debug, Clone)]
pub struct Game {
    board: Board,
}

impl Game {
    pub fn new() -> Self {
        Game { board: Default::default() }
    }

    /// Return cells of gameboard.
    pub fn board(&self) -> &Vec<Vec<(Cell, Direction)>> {
        &self.board.cell
    }

    /// Return size of gameboard.
    pub fn cell_size(&self) -> [f64; 2] {
        self.board.size
    }

    /// Level1
    ///ToDo Invent normal lvl creator
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

        for i in cells.iter_mut().take(19).skip(10) {
            i[25] = Cell::Wall;
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

        let cells = cells.iter()
            .map(|x| x.iter().map(|y| (*y, Direction::Top)).collect()).collect();

        let board = Board {
            size: [size as f64; 2],
            cell: cells,
        };
        self.board = board;
    }

    ///Check move possibility then move if possible
    pub fn move_from_cell_with_direction(&self, src: [usize; 2], direction: Direction) -> [usize; 2] {
        let (x, y) = (src[0], src[1]);
        dbg!("current: {} {}", x,y);
        let try_position = match direction {
            Direction::Top => if y > 0 { [x, y - 1] } else { [x, y] },
            Direction::Right => if x < CELL_COUNT - 1 { [x + 1, y] } else { [x, y] },
            Direction::Bottom => if y < CELL_COUNT - 1 { [x, y + 1] } else { [x, y] },
            Direction::Left => if x > 0 { [x - 1, y] } else { [x, y] },
        };
        let (new_x, new_y) = (try_position[0], try_position[1]);
        dbg!("next: {} {}", new_x,new_y);
        if self.board()[new_x][new_y].0 == Cell::Clear {
            [new_x, new_y]
        } else {
            src
        }
    }
}


#[derive(Default, Debug, PartialOrd, PartialEq, Clone)]
pub struct Board {
    size: [f64; 2],
    cell: Vec<Vec<(Cell, Direction)>>,
}


#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Cell {
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


///ToDo Тесты для функций
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_game_test() {
        let g = Game::new();
        assert_eq!(g.board.cell, Vec::<Vec<(Cell, Direction)>>::new());
        assert_eq!(g.board.size, [0.0; 2]);
    }

    #[test]
    fn lvl1_test() {
        let mut g = Game::new();
        g.lvl1();
        let v: Vec<&(Cell, Direction)> = g.board.cell.iter()
            .flat_map(|x|
                x.iter().filter(|c| *c == &(Cell::Wall, Direction::Top)))
            .collect();
        assert_eq!(v.len(), 13);
    }
}