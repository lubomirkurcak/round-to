use round_to::*;

#[test]
fn readme() {
    assert_eq!(0.4.round_to_i32(), 0);
    assert_eq!(0.6.round_to_i64(), 1);

    let a: i8 = 0.4.round_to();
    assert_eq!(a, 0);

    assert_eq!(0.5.round_to_i32(), 0);
    assert_eq!(0.5.floor_to_i32(), 0);
    assert_eq!(0.5.ceil_to_i32(), 1);
}
