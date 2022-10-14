use dsu::Dsu;

#[test]
fn test_1() {
    let mut dsu = Dsu::new(10);

    dsu.union(1, 2);
    dsu.union(2, 3);
    dsu.union(2, 7);

    assert_eq!(dsu.lookup(2).unwrap(), dsu.lookup(7).unwrap());
    assert_eq!(dsu.lookup(1).unwrap(), dsu.lookup(3).unwrap());
    assert_ne!(dsu.lookup(1).unwrap(), dsu.lookup(8).unwrap());
    assert_eq!(dsu.lookup(9).unwrap(), 9);
}

#[test]
fn test_2() {
    let mut dsu = Dsu::new(10);
    dsu.union(1, 77);
}
