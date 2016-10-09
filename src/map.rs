use ncurses;

use disp::Disp;
use object::Object;

pub type Map = [[Box<MapTile>; 77]; 19];
pub trait MapTile {
    fn get_disp(&self) -> Disp;
    fn passable(&self, object: &Object) -> bool;
}

pub struct EmptyTile {}
impl MapTile for EmptyTile {
    fn get_disp(&self) -> Disp { Disp::new(' ', ncurses::COLOR_WHITE) }
    fn passable(&self, _: &Object) -> bool { return false; }
}

pub struct Floor {}
impl MapTile for Floor {
    fn get_disp(&self) -> Disp { Disp::new('.', ncurses::COLOR_WHITE) }
    fn passable(&self, _: &Object) -> bool { return true; }
}

pub struct Wall {
    pub vert: bool
}
impl MapTile for Wall {
    fn get_disp(&self) -> Disp { Disp::new(if self.vert { '|' } else { '-' },
                                           ncurses::COLOR_WHITE) }
    fn passable(&self, _: &Object) -> bool { return false; }
}

pub struct Door {
    pub vert: bool,
    pub open: bool
}
impl MapTile for Door {
    fn get_disp(&self) -> Disp { Disp::new(if self.open {
        if self.vert { '-' } else { '|' }
    } else { '+' }, ncurses::COLOR_YELLOW) }
    fn passable(&self, _: &Object) -> bool { return self.open; }
}

pub struct Corridor {}
impl MapTile for Corridor {
    fn get_disp(&self) -> Disp { Disp::new('#', ncurses::COLOR_WHITE) }
    fn passable(&self, _: &Object) -> bool { return true; }
}
