use std::mem::size_of_val;

pub fn char_bool_unit_fn() {
    let c1: char = 'a';
    assert_eq!(size_of_val(&c1), 4)
}
