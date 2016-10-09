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

const REPULSION_NUM: usize = 30;
const REPULSION_FACTOR: f32 = 0.00001;
const REPULSION_TRIES: usize = 1000;
const REPULSION_POW: f32 = 4.5;

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
            if room.pos.x < room.width { room.pos.x = room.width; }
            if room.pos.x > 77.0 - room.width { room.pos.x = 77.0 - room.width; }

            room.pos.y += room.repulsion.y * REPULSION_FACTOR;
            if room.pos.y < room.height { room.pos.y = room.height; }
            if room.pos.y > 19.0 - room.height { room.pos.y = 19.0 - room.height; }

            room.repulsion = Vec2 { x: 0.0, y: 0.0 };
        }
    }

    let mut data: [[i32; 77]; 19] = [[0; 77]; 19];
    for room in rooms.iter() {
        for col in (room.pos.x - room.width).round() as usize..(room.pos.x + room.width).round() as usize {
            for row in (room.pos.y - room.height).round() as usize..(room.pos.y + room.height).round() as usize {
                data[row][col] = 1;
            }
        }
    }
    for row in 0..19 {
        for col in 0..77 {
            if data[row][col] == 1 {
                if [-1,0,1].iter().all(|&i| [-1,0,1].iter().all(|&j|
                    data.get(row.wrapping_add(i as usize)).map_or(false, |x|
                        x.get(col.wrapping_add(j as usize)).map_or(false, |&y|
                            y != 0)))) {
                    data[row][col] = 2;
                }
            }
        }
    }
    for row in 0..19 {
        for col in 0..77 {
            if data[row][col] == 1 {
                if row == 0 || row == 19-1 ||
                   data[row-1][col] != 1 || data[row+1][col] != 1 {
                    map[row][col] = Box::new(map::HorizWall {});
                } else {
                    map[row][col] = Box::new(map::VertWall {});
                }
            }
        }
    }

    map
}
