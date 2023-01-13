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

ghost_lite::ghost! {
    #![mod_value_namespace = unsized_impl]
    #![derive(Clone, Copy, Default, Hash, PartialOrd, Ord, PartialEq, Eq, Debug)]
    struct MyPhantomDataUnsized<T: ?Sized>
}

#[test]
fn test_unsized() {
    fn test() -> impl Clone
           + Copy
           + Default
           + ::core::hash::Hash
           + PartialOrd
           + Ord
           + PartialEq
           + Eq
           + ::core::fmt::Debug {
        MyPhantomDataUnsized::<str>
    }

    let _ = test;
}
