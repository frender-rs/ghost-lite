ghost_lite::ghost! {
    struct MMMM<T>
}

mod one_tp {
    use std::mem::size_of;

    ghost_lite::ghost! {
        struct MyPhantomData<T>
    }

    #[test]
    fn test() {
        let _ = MyPhantomData::<()>;
        let _: MyPhantomData<&str> = MyPhantomData;

        assert_eq!(size_of::<MyPhantomData<String>>(), 0);
    }
}

mod one_lt {
    use std::mem::size_of;

    ghost_lite::ghost! {
        struct MyPhantomData<'a>
    }

    #[test]
    fn test() {
        let _ = MyPhantomData::<'_>;
        let _ = MyPhantomData::<'static>;
        let _: MyPhantomData<'_> = MyPhantomData;
        let _: MyPhantomData<'static> = MyPhantomData;
        let _ = MyPhantomData;

        assert_eq!(size_of::<MyPhantomData>(), 0);
    }
}

mod only_tp {
    use std::mem::size_of;

    ghost_lite::ghost! {
        struct MyPhantomData<T, R>
    }

    #[test]
    fn test() {
        let _ = MyPhantomData::<(), i32>;
        let _: MyPhantomData<&str, ()> = MyPhantomData;

        assert_eq!(size_of::<MyPhantomData<String, &()>>(), 0);
    }
}

mod only_lt {
    use std::mem::size_of;

    ghost_lite::ghost! {
        struct MyPhantomData<'a, 'b>
    }

    #[test]
    fn test() {
        let _ = MyPhantomData::<'_, '_>;
        let _ = MyPhantomData::<'static, '_>;
        let _ = MyPhantomData::<'_, 'static>;
        let _ = MyPhantomData::<'static, 'static>;
        let _: MyPhantomData<'_, '_> = MyPhantomData;
        let _: MyPhantomData<'static, '_> = MyPhantomData;
        let _: MyPhantomData<'_, '_> = MyPhantomData;
        let _: MyPhantomData<'static, 'static> = MyPhantomData;
        let _ = MyPhantomData;

        assert_eq!(size_of::<MyPhantomData>(), 0);
    }
}

mod bounds {
    use std::mem::size_of;

    ghost_lite::ghost! {
        struct MyPhantomData<
            'lifetime,
            NoBound,
            OneBound: Copy,
            OneRelaxed: ?Sized,
            LifetimeAndOneBound: 'static + ToString,
            LifetimesAndOneBound: 'static + 'static + ToString,
            WhereClauseAndBounds: 'static + ToString,
            const N: usize,
        > where WhereClauseAndBounds: ?Sized
    }

    #[test]
    fn test() {
        assert_eq!(
            size_of::<
                MyPhantomData<
                    //
                    '_,
                    i32,
                    i8,
                    dyn ToString,
                    &'static str,
                    String,
                    dyn ToString,
                    1,
                >,
            >(),
            0
        );
    }
}
