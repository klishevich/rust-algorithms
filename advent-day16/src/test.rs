mod test {
    use crate::task;

    #[test]
    fn test01() {
        let (ver, _) = task("src/data01.txt");
        assert_eq!(ver, 6);
    }
    #[test]
    fn test02() {
        let (ver, _) = task("src/data02.txt");
        assert_eq!(ver, 9);
    }
    #[test]
    fn test03() {
        let (ver, _) = task("src/data03.txt");
        assert_eq!(ver, 14);
    }
    #[test]
    fn test11() {
        let (ver, _) = task("src/data11.txt");
        assert_eq!(ver, 16);
    }
    #[test]
    fn test12() {
        let (ver, _) = task("src/data12.txt");
        assert_eq!(ver, 12);
    }
    #[test]
    fn test13() {
        let (ver, _) = task("src/data13.txt");
        assert_eq!(ver, 23);
    }
    #[test]
    fn test14() {
        let (ver, _) = task("src/data14.txt");
        assert_eq!(ver, 31);
    }
    #[test]
    fn test21() {
        let (_, sum) = task("src/data21.txt");
        assert_eq!(sum, 3);
    }
    #[test]
    fn test22() {
        let (_, product) = task("src/data22.txt");
        assert_eq!(product, 54);
    }
    #[test]
    fn test23() {
        let (_, product) = task("src/data23.txt");
        assert_eq!(product, 7);
    }
    #[test]
    fn test24() {
        let (_, product) = task("src/data24.txt");
        assert_eq!(product, 9);
    }
    #[test]
    fn test25() {
        let (_, product) = task("src/data25.txt");
        assert_eq!(product, 1);
    }
    #[test]
    fn test26() {
        let (_, product) = task("src/data26.txt");
        assert_eq!(product, 0);
    }
    #[test]
    fn test27() {
        let (_, product) = task("src/data27.txt");
        assert_eq!(product, 0);
    }
    #[test]
    fn test28() {
        let (_, product) = task("src/data28.txt");
        assert_eq!(product, 1);
    }
}