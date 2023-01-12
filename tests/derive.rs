ghost_lite::ghost! {
    #![derive(Clone, Copy, Default, Hash, PartialOrd, Ord, PartialEq, Eq, Debug)]
    struct MyPhantomData<T>
}

#[test]
fn test() {
    struct Test;
    fn test() -> impl Clone
           + Copy
           + Default
           + ::core::hash::Hash
           + PartialOrd
           + Ord
           + PartialEq
           + Eq
           + ::core::fmt::Debug {
        MyPhantomData::<Test>
    }

    let _ = test;
}
