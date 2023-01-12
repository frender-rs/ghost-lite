ghost_lite::ghost! {
    struct MyPhantomData<T>
}

ghost_lite::ghost! {
    #![mod_value_namespace = my_phantom_data_2]
    struct MyPhantomData2<T>
}

#[test]
fn test() {
    let _ = MyPhantomData::<()>;
    let _: MyPhantomData<&str> = MyPhantomData;
    let _ = MyPhantomData2::<&i32>;
    let _: MyPhantomData2<&str> = MyPhantomData2;
}
