extern crate serial_number_arithmetic;

use serial_number_arithmetic::SerialNumber;

#[test]
fn inc_0() {
    assert_eq!(0u8.sn_add(1u8), Some(1u8));
}

#[test]
fn inc_1() {
    assert_eq!(1u8.sn_add(1u8), Some(2u8));
}

#[test]
fn inc_254() {
    assert_eq!(254u8.sn_add(1u8), Some(255u8));
}

#[test]
fn inc_255() {
    assert_eq!(255u8.sn_add(1u8), Some(0u8));
}

#[test]
fn hundred_plus_hundred() {
    assert_eq!(100u8.sn_add(100u8), Some(200u8));
}

#[test]
fn two_hundred_plus_hundred() {
    assert_eq!(200u8.sn_add(100u8), Some(44u8));
}

#[test]
fn zero_plus_127() {
    assert_eq!(0u8.sn_add(127u8), Some(127u8));
}

#[test]
fn zero_plus_128() {
    assert_eq!(0u8.sn_add(128u8), None);
}

fn test_lt(a: u8, b: u8, ab: bool, ba: bool) {
    assert_eq!(a.sn_lt(b), ab);
    assert_eq!(b.sn_lt(a), ba);
}

#[test]
fn lt_0_1() { test_lt(0, 1, true, false) }

#[test]
fn lt_0_44() { test_lt(0, 44, true, false) }

#[test]
fn lt_0_100() { test_lt(0, 100, true, false) }

#[test]
fn lt_44_100() { test_lt(44, 100, true, false) }

#[test]
fn lt_100_200() { test_lt(100, 200, true, false) }

#[test]
fn lt_200_255() { test_lt(200, 255, true, false) }

#[test]
fn lt_255_0() { test_lt(255, 0, true, false) }

#[test]
fn lt_255_100() { test_lt(255, 100, true, false) }

#[test]
fn lt_200_0() { test_lt(200, 0, true, false) }

#[test]
fn lt_200_44() { test_lt(200, 44, true, false) }

#[test]
fn lt_0_128() { test_lt(0, 128, false, false) }
