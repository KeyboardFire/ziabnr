extern crate ncurses;
extern crate rand;
use rand::Rng;

struct MapTile {
    ch: char,
    color: i16
}

fn main() {
    let map = unsafe {
        let mut map: [[MapTile; 77]; 19] = ::std::mem::uninitialized();
        for row in map.iter_mut() {
            for x in row.iter_mut() {
                ::std::ptr::write(x, MapTile {
                    ch: rand::thread_rng().gen_ascii_chars().next().unwrap(),
                    color: rand::thread_rng().gen_range(1, 8)
                });
            }
        }
        map
    };

    ncurses::initscr();
    ncurses::start_color();
    for i in 1i16..8i16 {
        // red, green, yellow, blue, magenta, cyan, white
        ncurses::init_pair(i, i, ncurses::COLOR_BLACK);
    }

    for (i, row) in map.iter().enumerate() {
        for (j, tile) in row.iter().enumerate() {
            ncurses::attron(ncurses::COLOR_PAIR(tile.color));
            ncurses::mvaddch(i as i32 + 2, j as i32 + 1, tile.ch as u64);
        }
    }

    ncurses::refresh();

    ncurses::getch();
    ncurses::endwin();
}
