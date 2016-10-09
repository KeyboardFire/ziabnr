extern crate ncurses;
extern crate rand;

mod disp;
mod pos;
mod map;
mod object;

use pos::Pos;
use object::Object;

mod util;
mod gen_rooms;

fn main() {
    let mut map = gen_rooms::gen_rooms();

    let mut objects: Vec<Box<Object>> = Vec::new();
    objects.push(Box::new(object::Player { pos: Pos { row: 5, col: 5 }}));
    objects.push(Box::new(object::RandomWalker { pos: Pos { row: 8, col: 8 }}));

    ncurses::initscr();
    ncurses::noecho();
    ncurses::start_color();
    for i in 1i16..8i16 {
        // red, green, yellow, blue, magenta, cyan, white
        ncurses::init_pair(i, i, ncurses::COLOR_BLACK);
    }

    loop {

        for i in 0..objects.len() {
            let (mut before, mut after) = objects.split_at_mut(i);
            let (mut object, mut after) = after.split_first_mut().unwrap();
            object.turn(&mut map, &mut before, &mut after);
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
