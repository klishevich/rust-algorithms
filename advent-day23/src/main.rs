use std::cmp;

const HALLWAY_SIZE: usize = 11;

struct BurrowState {
    vec2: Vec<u8>,
    vec4: Vec<u8>,
    vec6: Vec<u8>,
    vec8: Vec<u8>,
    vec_h: Vec<u8>,
}

impl Default for BurrowState {
    fn default() -> Self {
        BurrowState {
            vec2: vec![1, 2],
            vec4: vec![4, 3],
            vec6: vec![3, 2],
            vec8: vec![1, 4],
            vec_h: vec![0; HALLWAY_SIZE],
        }
    }
}

fn get_element_from_room(r: &Vec<u8>, target_amphipod: u8) -> i8 {
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

fn get_available_position_in_room(
    amphipod: u8,
    amphipod_position_in_hallway: usize,
    room_position_in_hallway: usize,
    hallway: &Vec<u8>,
    room: &Vec<u8>
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

struct Req {
    results: Vec<i32>,
}

impl Req {
    pub fn run(&mut self, s: &BurrowState, cost: i32) {
        // if s.vec2[1] != 0 {
        //     if s.vec2[1]
        // }
    }
}

fn main() {
    let vec2: Vec<u8> = vec![1, 2];
    let vec4: Vec<u8> = vec![4, 3];
    let vec6: Vec<u8> = vec![3, 2];
    let vec8: Vec<u8> = vec![1, 4];
    let vec_h: Vec<u8> = vec![0; 11];
}

mod test {
    use crate::get_available_hallway_positions;
    use crate::get_element_from_room;

    #[test]
    fn get_element_from_room_test1() {
        let v = vec![0, 0];
        let res = get_element_from_room(&v, 1);
        assert_eq!(res, -1);
    }

    #[test]
    fn get_element_from_room_test2() {
        let v = vec![1, 0];
        let res = get_element_from_room(&v, 1);
        assert_eq!(res, -1);
    }

    #[test]
    fn get_element_from_room_test3() {
        let v = vec![2, 0];
        let res = get_element_from_room(&v, 1);
        assert_eq!(res, 0);
    }

    #[test]
    fn get_element_from_room_test4() {
        let v = vec![1, 1];
        let res = get_element_from_room(&v, 1);
        assert_eq!(res, -1);
    }

    #[test]
    fn get_element_from_room_test5() {
        let v = vec![1, 2];
        let res = get_element_from_room(&v, 1);
        assert_eq!(res, 1);
    }

    #[test]
    fn get_element_from_room_test6() {
        let v = vec![2, 1];
        let res = get_element_from_room(&v, 1);
        assert_eq!(res, 1);
    }

    #[test]
    fn get_element_from_room_test7() {
        let v = vec![2, 2];
        let res = get_element_from_room(&v, 1);
        assert_eq!(res, 1);
    }

    #[test]
    fn get_available_hallway_positions_test1() {
        let h = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let res = get_available_hallway_positions(&h, 2);
        assert_eq!(res, (0, 11));
    }

    #[test]
    fn get_available_hallway_positions_test2() {
        let h = vec![0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0];
        let res = get_available_hallway_positions(&h, 2);
        assert_eq!(res, (2, 2));
    }

    #[test]
    fn get_available_hallway_positions_test3() {
        let h = vec![1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0];
        let res = get_available_hallway_positions(&h, 2);
        assert_eq!(res, (1, 4));
    }
}
