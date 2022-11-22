mod test_get_available_hallway_positions {
    use crate::get_available_hallway_positions;

    #[test]
    fn test1() {
        let h = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let res = get_available_hallway_positions(&h, 2);
        assert_eq!(res, (0, 11));
    }

    #[test]
    fn test2() {
        let h = vec![0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0];
        let res = get_available_hallway_positions(&h, 2);
        assert_eq!(res, (2, 2));
    }

    #[test]
    fn test3() {
        let h = vec![1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0];
        let res = get_available_hallway_positions(&h, 2);
        assert_eq!(res, (1, 4));
    }

    #[test]
    fn test4() {
        let h = vec![0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0];
        let res = get_available_hallway_positions(&h, 2);
        assert_eq!(res, (2, 2));
    }
}
