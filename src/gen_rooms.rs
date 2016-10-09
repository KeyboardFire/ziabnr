use rand;
use rand::Rng;

use map;
use map::Map;

struct Room {
    pos: Vec2,
    repulsion: Vec2,
    width: f32,
    height: f32
}

struct Vec2 {
    x: f32, y: f32
}

#[derive(Clone,Copy,PartialEq)]
enum GenData {
    Empty,
    Wall,
    Interior,
    InteriorFilled,
    Door,
    Corridor
}

#[derive(Clone,Copy,PartialEq)]
enum GenState {
    Searching,
    Filling
}

const REPULSION_NUM: usize = 20;
const REPULSION_FACTOR: f32 = 0.5;
const REPULSION_TRIES: usize = 500;
const REPULSION_POW: f32 = 8.5;

pub fn gen_rooms() -> Map {
    let mut map = unsafe {
        let mut map: Map = ::std::mem::uninitialized();
        for row in map.iter_mut() {
            for x in row.iter_mut() {
                ::std::ptr::write(x, Box::new(map::EmptyTile {}));
            }
        }
        map
    };

    let mut rooms: Vec<Room> = (0..REPULSION_NUM).map(|_| Room {
        pos: Vec2 {
            x: rand::thread_rng().gen_range(5.0, 72.0),
            y: rand::thread_rng().gen_range(3.0, 16.0)
        },
        repulsion: Vec2 { x: 0.0, y: 0.0 },
        width: rand::thread_rng().gen_range(2.0, 5.0),
        height: rand::thread_rng().gen_range(2.0, 3.5)
    }).collect();

    for _ in 0..REPULSION_TRIES {
        for i in 0..rooms.len() {
            for j in i+1..rooms.len() {
                let repulsion = Vec2 {
                    x: 1.0 / (rooms[i].pos.x - rooms[j].pos.x),
                    y: 1.0 / (rooms[i].pos.y - rooms[j].pos.y)
                };
                let repulsion = Vec2 {
                    x: repulsion.x.abs().powf(REPULSION_POW) * repulsion.x.signum(),
                    y: repulsion.y.abs().powf(REPULSION_POW) * repulsion.y.signum()
                };
                rooms[i].repulsion.x += rooms[j].width  * repulsion.x;
                rooms[i].repulsion.y += rooms[j].height * repulsion.y;
                rooms[j].repulsion.x += rooms[i].width  * repulsion.x;
                rooms[j].repulsion.y += rooms[i].height * repulsion.y;
            }
        }

        for mut room in rooms.iter_mut() {
            room.pos.x += room.repulsion.x * REPULSION_FACTOR;
            if room.pos.x < room.width { room.pos.x = rand::thread_rng().gen_range(room.width, room.width * 2.0); }
            if room.pos.x > 77.0 - room.width { room.pos.x = 77.0 - rand::thread_rng().gen_range(room.width, room.width * 2.0); }

            room.pos.y += room.repulsion.y * REPULSION_FACTOR;
            if room.pos.y < room.height { room.pos.y = rand::thread_rng().gen_range(room.height, room.height * 2.0); }
            if room.pos.y > 19.0 - room.height { room.pos.y = 19.0 - rand::thread_rng().gen_range(room.height, room.height * 2.0); }

            room.repulsion = Vec2 { x: 0.0, y: 0.0 };
        }
    }

    let mut data = [[GenData::Empty; 77]; 19];
    for room in rooms.iter() {
        for col in (room.pos.x - room.width).round() as usize..(room.pos.x + room.width).round() as usize {
            for row in (room.pos.y - room.height).round() as usize..(room.pos.y + room.height).round() as usize {
                data[row][col] = GenData::Wall;
            }
        }
    }

    for row in 0..19 {
        for col in 0..77 {
            if data[row][col] == GenData::Wall {
                if [-1,0,1].iter().all(|&i| [-1,0,1].iter().all(|&j|
                    data.get(row.wrapping_add(i as usize)).map_or(false, |x|
                        x.get(col.wrapping_add(j as usize)).map_or(false, |&y|
                            y != GenData::Empty)))) {
                    data[row][col] = GenData::Interior;
                }
            }
        }
    }

    while data.iter().any(|x| x.iter().any(|&y| y == GenData::Interior)) {
        flood_fill(&mut data, rand::thread_rng().gen_range(0, 19), rand::thread_rng().gen_range(0, 77), 0, 0, GenState::Searching);
    }

    for row in 0..19 {
        for col in 0..77 {
            match data[row][col] {
                GenData::Wall => {
                    if row == 0 || row == 19-1 ||
                       data[row-1][col] != GenData::Wall || data[row+1][col] != GenData::Wall {
                        map[row][col] = Box::new(map::HorizWall {});
                    } else {
                        map[row][col] = Box::new(map::VertWall {});
                    }
                },
                GenData::Door => {
                    map[row][col] = Box::new(map::Door {});
                },
                GenData::Corridor => {
                    map[row][col] = Box::new(map::Corridor {});
                },
                _ => {}
            }
        }
    }

    map
}

fn flood_fill(data: &mut [[GenData; 77]; 19], row: usize, col: usize, prow: usize, pcol: usize, state: GenState) {
    if row == -1i32 as usize || row == 19 || col == -1i32 as usize || col == 77 { return; }
    match data[row][col] {
        GenData::Empty => {
            if state == GenState::Filling {
                data[row][col] = GenData::Corridor;
                let crow = row.wrapping_add(row.wrapping_sub(prow));
                let ccol = col.wrapping_add(col.wrapping_sub(pcol));
                match data.get(crow).and_then(|x| x.get(ccol)) {
                    Some(&GenData::Empty) | Some(&GenData::Corridor) => {
                        if rand::thread_rng().gen_weighted_bool(35) {
                            let first_try = if rand::random() { 1 } else { -1i32 as usize };
                            if row == prow {
                                if row.wrapping_add(first_try) > 0 && row.wrapping_add(first_try) < 19 {
                                    flood_fill(data, row.wrapping_add(first_try), col, row, col, GenState::Filling);
                                } else {
                                    flood_fill(data, row.wrapping_sub(first_try), col, row, col, GenState::Filling);
                                }
                            } else { // col == pcol
                                if col.wrapping_add(first_try) > 0 && col.wrapping_add(first_try) < 77 {
                                    flood_fill(data, row, col.wrapping_add(first_try), row, col, GenState::Filling);
                                } else {
                                    flood_fill(data, row, col.wrapping_sub(first_try), row, col, GenState::Filling);
                                }
                            }
                        } else {
                            flood_fill(data, crow, ccol, row, col, GenState::Filling);
                        }
                    },
                    Some(&GenData::Wall) => {
                        flood_fill(data, crow, ccol, row, col, GenState::Filling);
                    },
                    Some(&GenData::Interior) | Some(&GenData::InteriorFilled) => {
                        panic!("empty touching interior in flood_fill?");
                    },
                    Some(&GenData::Door) => {
                        // how convenient
                    },
                    None => {
                        let first_try = if rand::random() { 1 } else { -1i32 as usize };
                        if row == prow {
                            if row.wrapping_add(first_try) > 0 && row.wrapping_add(first_try) < 19 {
                                flood_fill(data, row.wrapping_add(first_try), col, row, col, GenState::Filling);
                            } else {
                                flood_fill(data, row.wrapping_sub(first_try), col, row, col, GenState::Filling);
                            }
                        } else { // col == pcol
                            if col.wrapping_add(first_try) > 0 && col.wrapping_add(first_try) < 77 {
                                flood_fill(data, row, col.wrapping_add(first_try), row, col, GenState::Filling);
                            } else {
                                flood_fill(data, row, col.wrapping_sub(first_try), row, col, GenState::Filling);
                            }
                        }
                    }
                }
            }
        },
        GenData::Wall => {
            if data[prow][pcol] == GenData::Interior && rand::thread_rng().gen_weighted_bool(50) {
                let door_ok = {
                    let surrounding: Vec<Option<&GenData>> = [(1,0),(-1,0),(0,1),(0,-1)].iter().map(|&(x, y)|
                        data.get(row.wrapping_add(x as usize)).and_then(|z| z.get(col.wrapping_add(y as usize)))).collect();
                    surrounding.iter().any(|&z| z.map_or(false, |&a| a == GenData::Empty)) &&
                        surrounding.iter().any(|&z| z.map_or(false, |&a| a == GenData::Interior))
                };
                if door_ok {
                    data[row][col] = GenData::Door;
                    flood_fill(data, row+1, col, row, col, GenState::Filling);
                    flood_fill(data, row-1, col, row, col, GenState::Filling);
                    flood_fill(data, row, col+1, row, col, GenState::Filling);
                    flood_fill(data, row, col-1, row, col, GenState::Filling);
                }
            } else if data[prow][pcol] == GenData::Corridor {
                data[row][col] = GenData::Door;
            }
        },
        GenData::Interior => {
            if state == GenState::Filling {
                data[row][col] = GenData::InteriorFilled;
            }
            let mut dirs: [(i32, i32); 4] = [(1,0),(-1,0),(0,1),(0,-1)];
            rand::thread_rng().shuffle(&mut dirs);
            for &(x, y) in dirs.iter() {
                flood_fill(data, row.wrapping_add(x as usize), col.wrapping_add(y as usize), row, col, state);
            }
        },
        GenData::InteriorFilled | GenData::Door | GenData::Corridor => {}
    }
}
