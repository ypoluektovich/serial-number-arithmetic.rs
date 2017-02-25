pub trait SerialNumber: Sized + Copy + Eq {
    fn sn_add(self, n: Self) -> Option<Self>;
    fn sn_lt(self, n: Self) -> bool;
}

/*
 The module contains implementations of SerialNumber for u8, u16, u32, u64;
 in those, SERIAL_BITS is equal to each type's bit count.
 */
mod primitives;

// todo: arbitrary SERIAL_BITS up to 64

// todo: arbitrary SERIAL_BITS above 64
