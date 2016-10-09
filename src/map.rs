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
    fn passable(&self, _: &Object) -> bool { return true; }
}

pub struct VertWall {}

impl MapTile for VertWall {
    fn get_disp(&self) -> Disp { Disp::new('|', ncurses::COLOR_WHITE) }
    fn passable(&self, _: &Object) -> bool { return false; }
}

pub struct HorizWall {}

impl MapTile for HorizWall {
    fn get_disp(&self) -> Disp { Disp::new('-', ncurses::COLOR_WHITE) }
    fn passable(&self, _: &Object) -> bool { return false; }
}

pub struct Door {}

impl MapTile for Door {
    fn get_disp(&self) -> Disp { Disp::new('+', ncurses::COLOR_YELLOW) }
    fn passable(&self, _: &Object) -> bool { return true; }
}

pub struct Corridor {}

impl MapTile for Corridor {
    fn get_disp(&self) -> Disp { Disp::new('#', ncurses::COLOR_WHITE) }
    fn passable(&self, _: &Object) -> bool { return true; }
}
