mod vis_pub {
    ghost_lite::ghost! {
        pub struct MyPhantomData<T>
    }
}

mod vis_inherit {
    ghost_lite::ghost! {
        struct MyPhantomData<T>
    }
}

mod vis_pub_crate {
    ghost_lite::ghost! {
        pub(crate) struct MyPhantomData<T>
    }
}

mod vis_pub_self {
    ghost_lite::ghost! {
        pub(self) struct MyPhantomData<T>
    }
}

mod vis_pub_super {
    ghost_lite::ghost! {
        pub(super) struct MyPhantomData<T>
    }
}
