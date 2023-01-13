use std::mem::size_of_val;

mod my_phantom {
    ghost_lite::ghost! {
        pub struct MyPhantom<T: ?Sized>
    }
}

pub fn my_phantom() -> my_phantom::MyPhantom<str> {
    my_phantom::MyPhantom
}

#[test]
fn test() {
    assert_eq!(size_of_val(&my_phantom()), 0)
}
