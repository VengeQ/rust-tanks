pub struct Game{
    board:Board,

}

impl Game{
    pub fn new() -> Self{
        Game{ board: Default::default() }
    }
}

#[derive(Default)]
 struct Board{
    size:[f64;2],
    cell:Vec<Vec<usize>>
}

enum Cell{
    Clear,
    Water,
    Wall
}