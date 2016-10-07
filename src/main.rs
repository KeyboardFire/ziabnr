extern crate ncurses;

fn main() {
    ncurses::initscr();
    ncurses::printw("ziabnr\n");
    ncurses::refresh();
    ncurses::getch();
    ncurses::endwin();
}
