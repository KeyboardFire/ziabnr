extern crate ncurses;
extern crate rand;
use rand::Rng;

struct Disp {
    ch: char,
    color: i16
}

impl Disp {
    fn draw(&self, pos: &Pos) {
        ncurses::attron(ncurses::COLOR_PAIR(self.color));
        ncurses::mvaddch(pos.row + 2, pos.col + 1, self.ch as u64);
        ncurses::attroff(ncurses::COLOR_PAIR(self.color));
    }
}

#[derive(Clone,Copy,PartialEq)]
struct Pos {
    row: i32,
    col: i32
}

impl Pos {
    fn new(row: i32, col: i32) -> Pos {
        Pos { row: row, col: col }
    }
}

const LEFT: Pos = Pos { row: 0, col: -1 };
const DOWN: Pos = Pos { row: 1, col: 0 };
const UP: Pos = Pos { row: -1, col: 0 };
const RIGHT: Pos = Pos { row: 0, col: 1 };

type Map = [[Box<MapTile>; 77]; 19];

trait MapTile {
    fn get_disp(&self) -> Disp;
    fn passable(&self, object: &Object) -> bool;
}

struct EmptyTile {}

impl MapTile for EmptyTile {
    fn get_disp(&self) -> Disp { Disp { ch: ' ', color: ncurses::COLOR_WHITE } }
    fn passable(&self, object: &Object) -> bool { return true; }
}

struct VertWall {}

impl MapTile for VertWall {
    fn get_disp(&self) -> Disp { Disp { ch: '|', color: ncurses::COLOR_WHITE } }
    fn passable(&self, object: &Object) -> bool { return false; }
}

struct HorizWall {}

impl MapTile for HorizWall {
    fn get_disp(&self) -> Disp { Disp { ch: '-', color: ncurses::COLOR_WHITE } }
    fn passable(&self, object: &Object) -> bool { return false; }
}

trait Object {
    fn get_disp(&self) -> Disp;
    fn passable(&self, object: &Object) -> bool;
    fn get_pos(&self) -> Pos;
    fn turn(&mut self, map: &Map, before: &[Box<Object>], after: &[Box<Object>]);
}

struct Player {
    pos: Pos
}

impl Object for Player {
    fn get_disp(&self) -> Disp { Disp { ch: '@', color: ncurses::COLOR_WHITE } }
    fn passable(&self, object: &Object) -> bool { return false; }
    fn get_pos(&self) -> Pos { self.pos }

    fn turn(&mut self, map: &Map, before: &[Box<Object>], after: &[Box<Object>]) {
        let ch = ncurses::getch() as u8 as char;
        if ch == 'h' || ch == 'y' || ch == 'b' {
            if let Some(pos) = move_relative(self, &LEFT, map, before, after) {
                self.pos = pos;
            }
        }
        if ch == 'j' || ch == 'b' || ch == 'n' {
            if let Some(pos) = move_relative(self, &DOWN, map, before, after) {
                self.pos = pos;
            }
        }
        if ch == 'k' || ch == 'y' || ch == 'u' {
            if let Some(pos) = move_relative(self, &UP, map, before, after) {
                self.pos = pos;
            }
        }
        if ch == 'l' || ch == 'u' || ch == 'n' {
            if let Some(pos) = move_relative(self, &RIGHT, map, before, after) {
                self.pos = pos;
            }
        }
    }
}

struct RandomWalker {
    pos: Pos
}

impl Object for RandomWalker {
    fn get_disp(&self) -> Disp { Disp { ch: 'W', color: ncurses::COLOR_RED } }
    fn passable(&self, object: &Object) -> bool { return false; }
    fn get_pos(&self) -> Pos { self.pos }

    fn turn(&mut self, map: &Map, before: &[Box<Object>], after: &[Box<Object>]) {
        if let Some(new_pos) = move_relative(self, &Pos::new(
                rand::thread_rng().gen_range(-1, 2),
                rand::thread_rng().gen_range(-1, 2)), map, before, after) {
            self.pos = new_pos;
        }
    }
}

fn move_relative(object: &Object, offset: &Pos, map: &Map,
                 before: &[Box<Object>], after: &[Box<Object>]) -> Option<Pos> {
    let to = Pos { row: object.get_pos().row + offset.row,
                   col: object.get_pos().col + offset.col };
    if map[to.row as usize][to.col as usize].passable(object) &&
            before.iter().chain(after.iter()).all(|obj|
                obj.get_pos() != to || obj.passable(object)) {
        Some(to)
    } else {
        None
    }
}

fn main() {
    let mut map = unsafe {
        let mut map: Map = ::std::mem::uninitialized();
        for row in map.iter_mut() {
            for x in row.iter_mut() {
                ::std::ptr::write(x, Box::new(EmptyTile {}));
            }
        }
        map
    };

    // draw a simple room
    for row in 3..13 {
        map[row][3] = Box::new(VertWall {});
        map[row][13] = Box::new(VertWall {});
    }
    for col in 3..14 {
        map[3][col] = Box::new(HorizWall {});
        map[13][col] = Box::new(HorizWall {});
    }

    let mut objects: Vec<Box<Object>> = Vec::new();
    objects.push(Box::new(Player { pos: Pos { row: 5, col: 5 }}));
    objects.push(Box::new(RandomWalker { pos: Pos { row: 8, col: 8 }}));

    ncurses::initscr();
    ncurses::noecho();
    ncurses::start_color();
    for i in 1i16..8i16 {
        // red, green, yellow, blue, magenta, cyan, white
        ncurses::init_pair(i, i, ncurses::COLOR_BLACK);
    }

    loop {
        for i in 0..objects.len() {
            let (mut before_object, mut after_object) = objects.split_at_mut(i);
            let (mut object, mut after_object) = after_object.split_first_mut().unwrap();
            object.turn(&map, before_object, after_object);
        }
        for (i, row) in map.iter().enumerate() {
            for (j, tile) in row.iter().enumerate() {
                tile.get_disp().draw(&Pos { row: i as i32, col: j as i32 });
            }
        }
        for object in objects.iter().rev() {
            object.get_disp().draw(&object.get_pos());
        }
        ncurses::addch(8); // backspace; put cursor on top of player
        ncurses::refresh();
    }

    ncurses::endwin();
}
