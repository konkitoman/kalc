use crate::kalc_i64;

#[test]
fn add() {
    assert_eq!(kalc_i64("2 + 2").unwrap(), 4);
    assert_eq!(kalc_i64("2 + 2 + 2").unwrap(), 6)
}

#[test]
fn sub() {
    assert_eq!(kalc_i64("10 - 4").unwrap(), 6);
    assert_eq!(kalc_i64("10 - 2 - 2").unwrap(), 6);
}

#[test]
fn mul() {
    assert_eq!(kalc_i64("2 * 3").unwrap(), 6);
    assert_eq!(kalc_i64("3(3)").unwrap(), 9)
}

#[test]
fn div() {
    assert_eq!(kalc_i64("6 / 2").unwrap(), 3);
}

#[test]
fn order() {
    assert_eq!(kalc_i64("2(3-1)").unwrap(), 4);
    assert_eq!(kalc_i64("2(3-(2 - 1))").unwrap(), 4)
}
