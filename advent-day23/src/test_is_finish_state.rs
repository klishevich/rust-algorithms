mod test_is_finish_state {
    use crate::is_finish_state;
    use crate::BurrowState;
    use crate::HALLWAY_SIZE;

    #[test]
    fn test1() {
        let s: BurrowState = BurrowState {
            rooms: vec![vec![1, 1, 1, 1], vec![2, 2, 2, 2], vec![3, 3, 3, 3], vec![4, 4, 4, 4]],
            hallway: vec![0; HALLWAY_SIZE],
        };
        let res = is_finish_state(&s);
        assert_eq!(res, true);
    }
    #[test]
    fn test2() {
        let s: BurrowState = BurrowState {
            rooms: vec![vec![1, 2, 1, 1], vec![2, 1, 2, 2], vec![3, 3, 3, 3], vec![4, 4, 4, 4]],
            hallway: vec![0; HALLWAY_SIZE],
        };
        let res = is_finish_state(&s);
        assert_eq!(res, false);
    }
}
