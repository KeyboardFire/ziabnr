#[derive(Clone,Copy,PartialEq)]
pub struct Pos {
    pub row: i32,
    pub col: i32
}

impl Pos {
    pub fn new(row: i32, col: i32) -> Pos {
        Pos { row: row, col: col }
    }
}

pub const LEFT:  Pos = Pos { row:  0, col: -1 };
pub const DOWN:  Pos = Pos { row:  1, col:  0 };
pub const UP:    Pos = Pos { row: -1, col:  0 };
pub const RIGHT: Pos = Pos { row:  0, col:  1 };
