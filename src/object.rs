use ncurses;
use rand;
use rand::Rng;

use pos;
use pos::Pos;
use map::Map;
use disp::Disp;
use util;

pub trait Object {
    fn get_disp(&self) -> Disp;
    fn passable(&self, object: &Object) -> bool;
    fn get_pos(&self) -> Pos;
    fn turn(&mut self, map: &mut Map, before: &mut [Box<Object>], after: &mut [Box<Object>]);
}

pub struct Player {
    pub pos: Pos
}

impl Object for Player {
    fn get_disp(&self) -> Disp { Disp { ch: '@', color: ncurses::COLOR_WHITE } }
    fn passable(&self, _: &Object) -> bool { return false; }
    fn get_pos(&self) -> Pos { self.pos }

    fn turn(&mut self, map: &mut Map, before: &mut [Box<Object>], after: &mut [Box<Object>]) {
        let ch = ncurses::getch() as u8 as char;
        if ch == 'h' || ch == 'y' || ch == 'b' {
            if let Some(pos) = util::move_relative(self, &pos::LEFT, map, before, after) {
                self.pos = pos;
            }
        }
        if ch == 'j' || ch == 'b' || ch == 'n' {
            if let Some(pos) = util::move_relative(self, &pos::DOWN, map, before, after) {
                self.pos = pos;
            }
        }
        if ch == 'k' || ch == 'y' || ch == 'u' {
            if let Some(pos) = util::move_relative(self, &pos::UP, map, before, after) {
                self.pos = pos;
            }
        }
        if ch == 'l' || ch == 'u' || ch == 'n' {
            if let Some(pos) = util::move_relative(self, &pos::RIGHT, map, before, after) {
                self.pos = pos;
            }
        }
    }
}

pub struct RandomWalker {
    pub pos: Pos
}

impl Object for RandomWalker {
    fn get_disp(&self) -> Disp { Disp::new('W', ncurses::COLOR_RED) }
    fn passable(&self, _: &Object) -> bool { return false; }
    fn get_pos(&self) -> Pos { self.pos }

    fn turn(&mut self, map: &mut Map, before: &mut [Box<Object>], after: &mut [Box<Object>]) {
        if let Some(new_pos) = util::move_relative(self, &Pos::new(
                rand::thread_rng().gen_range(-1, 2),
                rand::thread_rng().gen_range(-1, 2)), map, before, after) {
            self.pos = new_pos;
        }
    }
}
