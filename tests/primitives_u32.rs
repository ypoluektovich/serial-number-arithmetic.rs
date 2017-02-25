extern crate serial_number_arithmetic;

use serial_number_arithmetic::SerialNumber;

#[test]
fn inc_0() {
    assert_eq!(0u32.sn_add(1u32), Some(1u32));
}

#[test]
fn inc_1() {
    assert_eq!(1u32.sn_add(1u32), Some(2u32));
}

#[test]
fn inc_m1() {
    assert_eq!(0xFFFFFFFE.sn_add(1u32), Some(0xFFFFFFFFu32));
}

#[test]
fn inc_m() {
    assert_eq!(0xFFFFFFFF.sn_add(1u32), Some(0u32));
}

fn test_lt(a: u32, b: u32, ab: bool, ba: bool) {
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
fn lt_m_0() { test_lt(0xFFFFFFFF, 0, true, false) }

#[test]
fn lt_0_mid() { test_lt(0, 0x80000000, false, false) }
