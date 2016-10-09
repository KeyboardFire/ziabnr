use ncurses;

use pos::Pos;

pub struct Disp {
    pub ch: char,
    pub color: i16
}

impl Disp {
    pub fn new(ch: char, color: i16) -> Disp {
        Disp { ch: ch, color: color }
    }

    pub fn draw(&self, pos: &Pos) {
        ncurses::attron(ncurses::COLOR_PAIR(self.color));
        ncurses::mvaddch(pos.row + 2, pos.col + 1, self.ch as u64);
        ncurses::attroff(ncurses::COLOR_PAIR(self.color));
    }
}
