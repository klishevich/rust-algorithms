mod test_get_path_length_to_room {
    use crate::get_path_length_to_room;

    #[test]
    fn test1() {
        let amphipod_position_in_hallway = 3;
        let room_position_in_hallway = 7;
        let hallway: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let res = get_path_length_to_room(
            amphipod_position_in_hallway,
            room_position_in_hallway,
            &hallway,
        );
        assert_eq!(res, 4);
    }
    #[test]
    fn test2() {
        let amphipod_position_in_hallway = 7;
        let room_position_in_hallway = 3;
        let hallway: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let res = get_path_length_to_room(
            amphipod_position_in_hallway,
            room_position_in_hallway,
            &hallway,
        );
        assert_eq!(res, 4);
    }
    #[test]
    fn test3() {
        let amphipod_position_in_hallway = 3;
        let room_position_in_hallway = 7;
        let hallway: Vec<u8> = vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0];
        let res = get_path_length_to_room(
            amphipod_position_in_hallway,
            room_position_in_hallway,
            &hallway,
        );
        assert_eq!(res, -1);
    }
    #[test]
    fn test4() {
        let amphipod_position_in_hallway = 7;
        let room_position_in_hallway = 3;
        let hallway: Vec<u8> = vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0];
        let res = get_path_length_to_room(
            amphipod_position_in_hallway,
            room_position_in_hallway,
            &hallway,
        );
        assert_eq!(res, -1);
    }
}
