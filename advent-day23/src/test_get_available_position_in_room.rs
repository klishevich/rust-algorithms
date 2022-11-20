mod test_get_available_position_in_room {
    use crate::get_available_position_in_room;

    #[test]
    fn test1() {
        let amphipod: u8 = 1;
        let room: Vec<u8> = vec![0, 0];
        let res = get_available_position_in_room(amphipod, &room);
        assert_eq!(res, 0);
    }

    #[test]
    fn test2() {
        let amphipod: u8 = 1;
        let room: Vec<u8> = vec![1, 0];
        let res = get_available_position_in_room(amphipod, &room);
        assert_eq!(res, 1);
    }

    #[test]
    fn test3() {
        let amphipod: u8 = 1;
        let room: Vec<u8> = vec![2, 0];
        let res = get_available_position_in_room(amphipod, &room);
        assert_eq!(res, -1);
    }
    
    #[test]
    fn test4() {
        let amphipod: u8 = 1;
        let room: Vec<u8> = vec![1, 2];
        let res = get_available_position_in_room(amphipod, &room);
        assert_eq!(res, -1);
    }
}
