use std::cmp;
use std::collections::HashMap;

const HALLWAY_SIZE: usize = 11;
const ROOM_SIZE_i32: i32 = 2;
const BASE5: u64 = 5;

struct BurrowState {
    rooms: Vec<Vec<u8>>,
    hallway: Vec<u8>,
}

impl Default for BurrowState {
    fn default() -> Self {
        BurrowState {
            // rooms: vec![vec![1, 2], vec![4, 3], vec![3, 2], vec![1, 4]],
            rooms: vec![vec![3, 2], vec![4, 1], vec![4, 2], vec![1, 3]],
            // rooms: vec![
            //     vec![1, 4, 4, 2],
            //     vec![4, 2, 3, 3],
            //     vec![3, 1, 2, 2],
            //     vec![1, 3, 1, 4],
            // ],
            hallway: vec![0; HALLWAY_SIZE],
        }
    }
}

fn get_element_from_room(r: &Vec<u8>, target_amphipod: u8) -> i32 {
    let mut res = -1;
    if r[1] == 0 {
        if r[0] != 0 && r[0] != target_amphipod {
            res = 0;
        }
    } else {
        if (r[0] == target_amphipod && r[1] != target_amphipod) || r[0] != target_amphipod {
            res = 1;
        }
    }
    return res;
}

fn get_available_hallway_positions(h: &Vec<u8>, start_position: usize) -> (usize, usize) {
    let mut min = start_position;
    let mut max = start_position;
    if h[start_position] == 0 {
        while min > 0 {
            if h[min - 1] == 0 {
                min -= 1;
            } else {
                break;
            }
        }
        while max < HALLWAY_SIZE {
            if h[max] == 0 {
                max += 1;
            } else {
                break;
            }
        }
    }
    return (min, max);
}

fn get_available_position_in_room(amphipod: u8, room: &Vec<u8>) -> i32 {
    let mut place_in_room = -1; // no place
    if room[0] == 0 {
        place_in_room = 0;
    } else {
        if room[0] == amphipod && room[1] == 0 {
            place_in_room = 1;
        }
    }
    return place_in_room;
}

fn get_path_length_to_room(
    amphipod_position_in_hallway: usize,
    room_position_in_hallway: usize,
    hallway: &Vec<u8>,
) -> i32 {
    let min = cmp::min(amphipod_position_in_hallway, room_position_in_hallway);
    let max = cmp::max(amphipod_position_in_hallway, room_position_in_hallway);
    if amphipod_position_in_hallway <= room_position_in_hallway {
        for i in amphipod_position_in_hallway + 1..room_position_in_hallway + 1 {
            if hallway[i] != 0 {
                return -1;
            }
        }
    } else {
        for i in room_position_in_hallway..amphipod_position_in_hallway {
            if hallway[i] != 0 {
                return -1;
            }
        }
    }
    return (max - min).try_into().unwrap();
}

fn is_finish_state(s: &BurrowState) -> bool {
    // let rooms: &Vec<Vec<u8>> = s.rooms.as_ref();
    for (index, room) in s.rooms.iter().enumerate() {
        let room_amphipod: u8 = (index + 1).try_into().unwrap();
        for pos in room {
            if *pos != room_amphipod {
                return false;
            }
        }
    }
    return true;
}

fn createNewBurrowState(
    s: &BurrowState,
    h_i: usize,
    h_a: u8,
    r_i: usize,
    r_j: usize,
    r_a: u8,
) -> BurrowState {
    let mut n_hallway: Vec<u8> = Vec::new();
    for (i, v) in s.hallway.iter().enumerate() {
        if i == h_i {
            n_hallway.push(h_a);
        } else {
            n_hallway.push(*v);
        }
    }
    let mut n_rooms: Vec<Vec<u8>> = Vec::new();
    for (i, room) in s.rooms.iter().enumerate() {
        let mut n_room: Vec<u8> = Vec::new();
        for (j, v) in room.iter().enumerate() {
            if i == r_i && j == r_j {
                n_room.push(r_a);
            } else {
                n_room.push(*v);
            }
        }
        n_rooms.push(n_room);
    }
    return BurrowState {
        hallway: n_hallway,
        rooms: n_rooms,
    };
}

fn get_room_position(index: usize) -> usize {
    match index {
        0 => 2,
        1 => 4,
        2 => 6,
        3 => 8,
        _ => 0,
    }
}

fn get_cost_per_step(a: u8) -> u32 {
    match a {
        1 => 1,
        2 => 10,
        3 => 100,
        4 => 1000,
        _ => 0,
    }
}

fn get_room_amphipod(index: usize) -> u8 {
    match index {
        0 => 1,
        1 => 2,
        2 => 3,
        3 => 4,
        _ => 0,
    }
}

fn get_room_index(amphipod: u8) -> usize {
    match amphipod {
        1 => 0,
        2 => 1,
        3 => 2,
        4 => 3,
        _ => 10,
    }
}

fn get_distance(i: usize, j: usize) -> u32 {
    if i > j {
        return (i - j).try_into().unwrap();
    } else {
        return (j - i).try_into().unwrap();
    }
}

fn calc_hash(s: &BurrowState) -> u64 {
    let mut i: u32 = 0;
    let mut res: u64 = 0;
    for a in &s.hallway {
        let m = BASE5.pow(i) * u64::try_from(*a).unwrap();
        res += m;
        i += 1;
    }
    for r in &s.rooms {
        for v in r {
            let m = BASE5.pow(i) * u64::try_from(*v).unwrap();
            res += m;
            i += 1;
        }
    }
    return res;
}

struct Req {
    min_cost: u32,
    cache: HashMap<u64, u32>
}

impl Req {
    pub fn run(&mut self, s: &BurrowState, cost: u32) -> () {
        let hash = calc_hash(s);
        match self.cache.get(&hash) {
            Some(&v) => {
                if cost >= v {
                    return;
                }
            },
            None => ()
        };
        self.cache.insert(hash, cost);
        if cost >= self.min_cost {
            return;
        }
        if is_finish_state(s) {
            if cost < self.min_cost {
                self.min_cost = cost;
            }
        }
        // Hallway
        for (h_i, &h_a) in s.hallway.iter().enumerate() {
            if h_a != 0 {
                let room_index: usize = (h_a - 1).try_into().unwrap();
                let room = &s.rooms[room_index];
                let place = get_available_position_in_room(h_a, room);
                if place != -1 {
                    let length =
                        get_path_length_to_room(h_i, get_room_position(room_index), &s.hallway);
                    if length != -1 {
                        let total_length: u32 =
                            (length + ROOM_SIZE_i32 - place).try_into().unwrap();
                        let new_cost = total_length * get_cost_per_step(h_a);
                        let place_usize: usize = place.try_into().unwrap();
                        let new_state =
                            createNewBurrowState(&s, h_i, 0, room_index, place_usize, h_a);
                        Req::run(self, &new_state, cost + new_cost);
                    }
                }
            }
        }
        // Rooms
        for (r_i, room) in s.rooms.iter().enumerate() {
            let place_from = get_element_from_room(room, get_room_amphipod(r_i));
            if place_from != -1 {
                let place_from_usize: usize = place_from.try_into().unwrap();
                let amphipod = room[place_from_usize];
                let room_from_position = get_room_position(r_i);
                let target_room_index = get_room_index(amphipod);
                if r_i != target_room_index {
                    let place_to =
                        get_available_position_in_room(amphipod, &s.rooms[target_room_index]);
                    if place_to != -1 {
                        let place_to_usize: usize = place_to.try_into().unwrap();
                        let from_room_position = get_room_position(r_i);
                        let to_room_position = get_room_position(target_room_index);
                        let length_to = get_path_length_to_room(
                            from_room_position,
                            to_room_position,
                            &s.hallway,
                        );
                        if s.hallway[from_room_position] == 0 && length_to != -1 {
                            let total_length: u32 =
                                (length_to + ROOM_SIZE_i32 - place_from + ROOM_SIZE_i32 - place_to)
                                    .try_into()
                                    .unwrap();
                            let new_cost = total_length * get_cost_per_step(amphipod);
                            let new_state = createNewBurrowState(
                                &s,
                                from_room_position,
                                amphipod,
                                r_i,
                                place_from_usize,
                                0,
                            );
                            let new_state2 = createNewBurrowState(
                                &new_state,
                                from_room_position,
                                0,
                                target_room_index,
                                place_to_usize,
                                amphipod,
                            );
                            Req::run(self, &new_state2, cost + new_cost);
                            continue;
                        }
                    }
                }

                let (min, max) = get_available_hallway_positions(&s.hallway, room_from_position);
                for h_i in min..max {
                    let total_length: u32 = get_distance(room_from_position, h_i)
                        + u32::try_from(ROOM_SIZE_i32 - place_from).unwrap();
                    let new_cost = total_length * get_cost_per_step(amphipod);
                    let new_state =
                        createNewBurrowState(&s, h_i, amphipod, r_i, place_from_usize, 0);
                    Req::run(self, &new_state, cost + new_cost);
                }
            }
        }
    }
}

fn main() {
    let state: BurrowState = BurrowState {
        ..Default::default()
    };
    let mut req: Req = Req { min_cost: u32::MAX, cache: HashMap::new() };
    req.run(&state, 0);
    println!("res {}", req.min_cost);
}

#[cfg(test)]
mod test_get_available_hallway_positions;
mod test_get_available_position_in_room;
mod test_get_element_from_room;
mod test_get_path_length_to_room;
mod test_is_finish_state;
