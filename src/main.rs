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

#[derive(Clone,Copy)]
struct Pos {
    row: i32,
    col: i32
}

impl Pos {
    fn new(row: i32, col: i32) -> Pos {
        Pos { row: row, col: col }
    }
}

type Map = [[Box<MapTile>; 77]; 19];

trait MapTile {
    fn get_disp(&self) -> Disp;
    fn passable(&self, object: &Object) -> bool;
}

struct EmptyTile {}

impl MapTile for EmptyTile {
    fn get_disp(&self) -> Disp {
        Disp { ch: ' ', color: ncurses::COLOR_WHITE }
    }
    fn passable(&self, object: &Object) -> bool {
        return true;
    }
}

struct VertWall {}

impl MapTile for VertWall {
    fn get_disp(&self) -> Disp {
        Disp { ch: '|', color: ncurses::COLOR_WHITE }
    }
    fn passable(&self, object: &Object) -> bool {
        return false;
    }
}

struct HorizWall {}

impl MapTile for HorizWall {
    fn get_disp(&self) -> Disp {
        Disp { ch: '-', color: ncurses::COLOR_WHITE }
    }
    fn passable(&self, object: &Object) -> bool {
        return false;
    }
}

trait Object {
    fn get_disp(&self) -> Disp;
    fn get_pos(&self) -> Pos;
    fn turn(&mut self, map: &Map);
}

struct Player {
    pos: Pos
}

impl Object for Player {
    fn get_disp(&self) -> Disp {
        Disp { ch: '@', color: ncurses::COLOR_WHITE }
    }
    fn get_pos(&self) -> Pos {
        self.pos
    }

    fn turn(&mut self, map: &Map) {
        let ch = ncurses::getch() as u8 as char;
        if ch == 'h' || ch == 'y' || ch == 'b' {
            if let Some(new_pos) = move_relative(self, &Pos::new(0, -1), map) {
                self.pos = new_pos;
            }
        }
        if ch == 'j' || ch == 'b' || ch == 'n' {
            if let Some(new_pos) = move_relative(self, &Pos::new(1, 0), map) {
                self.pos = new_pos;
            }
        }
        if ch == 'k' || ch == 'y' || ch == 'u' {
            if let Some(new_pos) = move_relative(self, &Pos::new(-1, 0), map) {
                self.pos = new_pos;
            }
        }
        if ch == 'l' || ch == 'u' || ch == 'n' {
            if let Some(new_pos) = move_relative(self, &Pos::new(0, 1), map) {
                self.pos = new_pos;
            }
        }
    }
}

fn move_relative(object: &Object, offset: &Pos, map: &Map) -> Option<Pos> {
    let to = Pos { row: object.get_pos().row + offset.row,
                   col: object.get_pos().col + offset.col };
    if map[to.row as usize][to.col as usize].passable(object) {
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

    ncurses::initscr();
    ncurses::noecho();
    ncurses::start_color();
    for i in 1i16..8i16 {
        // red, green, yellow, blue, magenta, cyan, white
        ncurses::init_pair(i, i, ncurses::COLOR_BLACK);
    }

    loop {
        for mut object in objects.iter_mut() {
            object.turn(&map);
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
