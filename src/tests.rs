use std::mem::size_of;

#[test]
fn size_of_i64_test() {

    let size = size_of::<i64>();
    assert_eq!(size, 8);

    let size = size_of::<Option::<i64>>();
    assert_eq!(size, 16);
}

#[test]
fn size_of_u8_test() {

    let size = size_of::<u8>();
    assert_eq!(size, 1);

    let size = size_of::<Option::<u8>>();
    assert_eq!(size, 2);
}

#[test]
fn size_of_bool_test() {

    let size = size_of::<bool>();
    assert_eq!(size, 1);

    let size = size_of::<Option::<bool>>();
    assert_eq!(size, 1); // what
}

#[test]
fn size_of_bool_array_test() {

    let size = size_of::<[bool; 8]>();
    assert_eq!(size, 8);
}

#[test]
fn rust_suggestions_test() {
    let byte = Bytes(10);

    assert_eq!(byte.as_usize(), 10);
}

pub struct Bytes(usize);

impl Bytes {

    pub fn as_usize(&self) -> usize {
        self.0
    }

    pub fn from_usize(n: usize) -> Self {
        Self(n)
    }
}