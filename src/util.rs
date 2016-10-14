use object::Object;
use pos::Pos;
use map::Map;

pub fn move_relative(object: &Object, offset: &Pos, map: &Map,
                     before: &[Box<Object>], after: &[Box<Object>]) -> Option<Pos> {
    let to = Pos { row: object.get_pos().row + offset.row,
                   col: object.get_pos().col + offset.col };
    if to.row >= 0 && to.row < 77 && to.col >= 0 && to.col < 19 &&
            map[to.row as usize][to.col as usize].passable(object) &&
            before.iter().chain(after.iter()).all(|obj|
                obj.get_pos() != to || obj.passable(object)) {
        Some(to)
    } else {
        None
    }
}
