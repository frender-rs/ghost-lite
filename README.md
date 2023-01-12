# `::ghost_lite`

A lightweight implementation of [`::ghost`](https://github.com/dtolnay/ghost) with `macro_rules`.

## Usage

```rust
use ghost_lite::ghost;

ghost! {
    /// `ghost` macro defines a custom `PhantomData`,
    /// which can be used as both a type and a value.
    pub struct MyPhantomData<T>
}

fn main() {
    let _: MyPhantomData<i32> = MyPhantomData;
    let _ = MyPhantomData::<&str>;
}
```

## Caveats

1.  `derive` should be in inner attributes (`#![derive(...)]`).
    Only the following traits are supported.

    ```rust
    ghost_lite::ghost! {
        #![derive(Clone, Copy, Default, Hash, PartialOrd, Ord, PartialEq, Eq, Debug)]
        pub struct MyPhantom<T>
    }

    /// `String` is not copy, but `MyPhantom` is always Copy, like `PhantomData`
    fn test() -> impl Copy {
        MyPhantom::<String>
    }
    # fn main() {}
    ```

    `derive` in outer attributes will be directly prepended to the generated `enum`,
    which works like normal derive macros.

    ```rust,compile_fail
    ghost_lite::ghost! {
        /// MyPhantom is `Clone` and `Copy` only if T is `Clone` and `Copy`
        #[derive(Clone, Copy)]
        pub struct MyPhantom<T>
    }

    /// `String` is not copy, so `MyPhantom` is not Copy
    fn test() -> impl Copy {
        MyPhantom::<String>
    }
    # fn main() {}
    ```

2.  The implementation relies on a `mod` name.
    To define multiple custom PhantomData types in the same module,
    you must provide custom `mod` name with
    `#![mod_value_namespace = my_phantom_data]`.

    ```rust,compile_fail
    ghost_lite::ghost! {
        struct MyPhantomData1<T>
    }

    ghost_lite::ghost! {
        struct MyPhantomData2<T>
    }
    # fn main() {}
    ```

    ```rust
    ghost_lite::ghost! {
        struct MyPhantomData1<T>
    }

    ghost_lite::ghost! {
        #![mod_value_namespace = my_phantom_data_2]
        struct MyPhantomData2<T>
    }
    # fn main() {}
    ```

3.  Move type generic bounds to `where` clause if `ghost!` reports error.

    Parsing tokens is limited with `macro_rules`,
    so complex type bounds are not supported.

    For example:

    ```rust,compile_fail
    ghost_lite::ghost! {
        struct MyPhantomData<T: Clone + PartialEq>
    }
    # fn main() {}
    ```

    ```rust
    ghost_lite::ghost! {
        struct MyPhantomData<T> where T: Clone + PartialEq
    }
    # fn main() {}
    ```

4.  Please don't add a trailing semicolon `;`.

    ```rust,compile_fail
    ghost_lite::ghost! {
        struct MyPhantomData<T>;
    }
    # fn main() {}
    ```

    ```rust
    ghost_lite::ghost! {
        struct MyPhantomData<T>
    }
    # fn main() {}
    ```
