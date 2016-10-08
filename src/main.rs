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

struct MapTile {
    disp: Disp
}

trait Object {
    fn get_disp(&self) -> Disp;
    fn get_pos(&self) -> Pos;
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
}

fn main() {
    let map = unsafe {
        let mut map: [[MapTile; 77]; 19] = ::std::mem::uninitialized();
        for row in map.iter_mut() {
            for x in row.iter_mut() {
                ::std::ptr::write(x, MapTile { disp: Disp {
                    ch: rand::thread_rng().gen_ascii_chars().next().unwrap(),
                    color: rand::thread_rng().gen_range(1, 8)
                }});
            }
        }
        map
    };

    let mut objects: Vec<Box<Object>> = Vec::new();
    objects.push(Box::new(Player { pos: Pos { row: 5, col: 5 }}));

    ncurses::initscr();
    ncurses::start_color();
    for i in 1i16..8i16 {
        // red, green, yellow, blue, magenta, cyan, white
        ncurses::init_pair(i, i, ncurses::COLOR_BLACK);
    }

    for (i, row) in map.iter().enumerate() {
        for (j, tile) in row.iter().enumerate() {
            tile.disp.draw(&Pos { row: i as i32, col: j as i32 });
        }
    }
    for object in objects.iter().rev() {
        object.get_disp().draw(&object.get_pos());
    }
    ncurses::addch(8); // backspace; put cursor on top of player

    ncurses::refresh();

    ncurses::getch();
    ncurses::endwin();
}
