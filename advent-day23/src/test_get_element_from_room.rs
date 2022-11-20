mod test_get_element_from_room {
    use crate::get_element_from_room;

    #[test]
    fn test1() {
        let v = vec![0, 0];
        let res = get_element_from_room(&v, 1);
        assert_eq!(res, -1);
    }

    #[test]
    fn test2() {
        let v = vec![1, 0];
        let res = get_element_from_room(&v, 1);
        assert_eq!(res, -1);
    }

    #[test]
    fn test3() {
        let v = vec![2, 0];
        let res = get_element_from_room(&v, 1);
        assert_eq!(res, 0);
    }

    #[test]
    fn test4() {
        let v = vec![1, 1];
        let res = get_element_from_room(&v, 1);
        assert_eq!(res, -1);
    }

    #[test]
    fn test5() {
        let v = vec![1, 2];
        let res = get_element_from_room(&v, 1);
        assert_eq!(res, 1);
    }

    #[test]
    fn test6() {
        let v = vec![2, 1];
        let res = get_element_from_room(&v, 1);
        assert_eq!(res, 1);
    }

    #[test]
    fn test7() {
        let v = vec![2, 2];
        let res = get_element_from_room(&v, 1);
        assert_eq!(res, 1);
    }
}