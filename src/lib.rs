#![doc = include_str!("../README.md")]

#[doc(hidden)]
pub mod macro_docs {
    pub mod inner_attr {
        /// Specify a `mod` name which exports the value namespace.
        /// Defaults to `__impl_value_namespace`.
        pub mod mod_value_namespace {}

        /// Generate code for some traits.
        pub mod derive {
            pub use ::core::clone::Clone;
            pub use ::core::cmp::{Eq, Ord, PartialEq, PartialOrd};
            pub use ::core::default::Default;
            pub use ::core::fmt::Debug;
            pub use ::core::hash::Hash;
            pub use ::core::marker::Copy;
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! expand_a_or_b {
    ([][$($b:tt)*]) => {
        $($b)*
    };
    ([$($a:tt)+][$($b:tt)*]) => {
        $($a)+
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_value_namespace {
    ([] $vis:tt $name:ident) => {
        $crate::__impl_value_namespace! {
            [__impl_value_namespace] $vis $name
        }
    };
    ([$mod_value_namespace:ident] {$($vis:tt)*} $name:ident) => {
        mod $mod_value_namespace {
            $crate::__impl_value_namespace_use! {
                $name $($vis)*
            }
        }

        #[doc(hidden)]
        $($vis)* use $mod_value_namespace::*;
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_value_namespace_use {
    ($name:ident) => {
        pub(super) use super::$name::$name;
    };
    ($name:ident pub) => {
        pub use super::$name::$name;
    };
    ($name:ident pub($(in)? self $($p:tt)*)) => {
        pub(in super $($p)*) use super::$name::$name;
    };
    ($name:ident pub($(in)? super $($p:tt)*)) => {
        pub(in super::super $($p)*) use super::$name::$name;
    };
    ($name:ident pub($(in)? crate $($p:tt)*)) => {
        pub(in crate $($p)*) use super::$name::$name;
    };
    ($name:ident $($vis:tt)+) => {
        pub(super) use super::$name::$name;
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_phantom_types {
    ([$($type_generic:tt)*]) => {
        (
            $( $crate::__impl_phantom_type![$type_generic] ,)*
        )
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_phantom_type {
    ({$lt:lifetime}) => {
        & $lt ()
    };
    ({{const $const_ident:ident} $($t:tt)*}) => {
        ()
    };
    ({{} $($t:tt)*}) => {
        ::core::marker::PhantomData<$($t)*>
        // compile_error!{stringify!($($t)*)}
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_inner_attr {
    (
        $vis:tt $name:ident $data:tt
        mod_value_namespace $(= $mod_value_namespace:ident)?
    ) => {
        $crate::__impl_value_namespace! {
            [$($mod_value_namespace)?]
            $vis
            $name
        }
    };
    (
        $vis:tt $name:ident $data:tt
        derive $(( $($derive_macro_name:ident),* $(,)? ))?
    ) => {
        $($(
            $crate::__impl_derive_macro! {
                $derive_macro_name
                $derive_macro_name
                $name $data
            }
        )*)?
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_derive_macro {
    (Copy $derive_macro_name:ident $name:ident { $rest:tt [$($impl_generics:tt)*][$($type_generics:tt)*][$($where_clause:tt)*] } ) => {
        impl<$($impl_generics)*> $crate::macro_docs::inner_attr::derive::$derive_macro_name
            for $name<$($type_generics)*> $($where_clause)* {}
    };
    (Clone $derive_macro_name:ident $name:ident { $rest:tt [$($impl_generics:tt)*][$($type_generics:tt)*][$($where_clause:tt)*] } ) => {
        impl<$($impl_generics)*> $crate::macro_docs::inner_attr::derive::$derive_macro_name
            for $name<$($type_generics)*> $($where_clause)* {
            #[inline]
            fn clone(&self) -> Self {
                Self::$name
            }
        }
    };
    (Default $derive_macro_name:ident $name:ident { $rest:tt [$($impl_generics:tt)*][$($type_generics:tt)*][$($where_clause:tt)*] } ) => {
        impl<$($impl_generics)*> $crate::macro_docs::inner_attr::derive::$derive_macro_name
            for $name<$($type_generics)*> $($where_clause)* {
            #[inline]
            fn default() -> Self {
                Self::$name
            }
        }
    };
    (Hash $derive_macro_name:ident $name:ident { $rest:tt [$($impl_generics:tt)*][$($type_generics:tt)*][$($where_clause:tt)*] } ) => {
        impl<$($impl_generics)*> $crate::macro_docs::inner_attr::derive::$derive_macro_name
            for $name<$($type_generics)*> $($where_clause)* {
            #[inline]
            fn hash<H: ::core::hash::Hasher>(&self, _: &mut H) {}
        }
    };
    (PartialOrd $derive_macro_name:ident $name:ident { $rest:tt [$($impl_generics:tt)*][$($type_generics:tt)*][$($where_clause:tt)*] } ) => {
        impl<$($impl_generics)*> $crate::macro_docs::inner_attr::derive::$derive_macro_name
            for $name<$($type_generics)*> $($where_clause)* {
            fn partial_cmp(&self, _other: &Self) -> Option<::core::cmp::Ordering> {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal)
            }
        }
    };
    (Ord $derive_macro_name:ident $name:ident { $rest:tt [$($impl_generics:tt)*][$($type_generics:tt)*][$($where_clause:tt)*] } ) => {
        impl<$($impl_generics)*> $crate::macro_docs::inner_attr::derive::$derive_macro_name
            for $name<$($type_generics)*> $($where_clause)* {
            fn cmp(&self, _other: &Self) -> ::core::cmp::Ordering {
                ::core::cmp::Ordering::Equal
            }
        }
    };
    (PartialEq $derive_macro_name:ident $name:ident { $rest:tt [$($impl_generics:tt)*][$($type_generics:tt)*][$($where_clause:tt)*] } ) => {
        impl<$($impl_generics)*> $crate::macro_docs::inner_attr::derive::$derive_macro_name
            for $name<$($type_generics)*> $($where_clause)* {
            fn eq(&self, _other: &Self) -> ::core::primitive::bool {
                true
            }
        }
    };
    (Eq $derive_macro_name:ident $name:ident { $rest:tt [$($impl_generics:tt)*][$($type_generics:tt)*][$($where_clause:tt)*] } ) => {
        impl<$($impl_generics)*> $crate::macro_docs::inner_attr::derive::$derive_macro_name
            for $name<$($type_generics)*> $($where_clause)* {
        }
    };
    (Debug $derive_macro_name:ident $name:ident { $rest:tt [$($impl_generics:tt)*][$($type_generics:tt)*][$($where_clause:tt)*] } ) => {
        impl<$($impl_generics)*> $crate::macro_docs::inner_attr::derive::$derive_macro_name
            for $name<$($type_generics)*> $($where_clause)* {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                write!(f, ::core::stringify!($name))
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_ghost {
    ($data:tt $phantom_types:tt) => {
        $crate::__impl_ghost_enum! { $data $phantom_types }
        $crate::__impl_ghost_inner_attrs! { $data }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_ghost_enum {
    ({
        {
            $attrs_inner:tt
            {$($attr:tt)*}
            {$($vis:tt)*}
            $name:ident
        }
        {$($rest:tt)+}
        $impl_generics:tt
        $type_generics:tt
        $where_clause:tt
    } $phantom_types:tt) => {
        $($attr)*
        $($vis)* enum $name<$($rest)+ {
            $name,
            #[doc(hidden)]
            __Phantom(
                ::core::convert::Infallible,
                ::core::marker::PhantomData<
                    $crate::__impl_phantom_types![$phantom_types]
                >
            ),
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_ghost_inner_attrs {
    ({
        {
            {$(#![$inner_attr_name:ident $($inner_attr:tt)* ])*}
            $attrs:tt
            $vis:tt
            $name:ident
        }
        $rest:tt
        $impl_generics:tt
        $type_generics:tt
        $where_clause:tt
    }) => {
        $(
            $crate::__impl_inner_attr! {
                $vis $name
                {$rest $impl_generics $type_generics $where_clause}
                $inner_attr_name $($inner_attr)*
            }
        )*
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_resolve_type_generics_expand {
    (
        $data:tt
        $rest:tt
    ) => {
        $crate::__impl_resolve_type_generics! {
            $data
            $rest
            $rest
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_resolve_type_generics {
    (
        $data:tt
        $rest:tt
        {
            $(
                $lt:lifetime
                $(:
                    $($lt_bound:lifetime)?
                    $(+ $lt_bounds:lifetime)*
                )?
            ),*
            $($($ignore_comma:literal)? ,)?
            $(
                $const_or_tp:ident
                $($tp:ident)?
                $(:
                    $($tp_lt_bound:lifetime)?
                    $(+ $tp_lt_bounds:lifetime)*
                    $($($ignore:literal)? +)?
                    $(
                        $(? $tp_bound_relax_ignore:vis)?
                        $tp_bound:path
                    )?
                )?
            ),*
            $(,)?
            >
            $(where $($where_clause:tt)*)?
        }
    ) => {
        $crate::__impl_ghost! {
            {
                $data
                $rest
                // impl generics
                [
                    $(
                        $lt
                        $(:
                            $($lt_bound)?
                            $(+ $lt_bounds)*
                        )?
                    ),*
                    $($($ignore_comma)? ,)?
                    $(
                        $const_or_tp
                        $($tp)?
                        $(:
                            $($tp_lt_bound)?
                            $(+ $tp_lt_bounds)*
                            $($($ignore)? +)?
                            $(
                                $(? $tp_bound_relax_ignore)?
                                $tp_bound
                            )?
                        )?
                    ),*
                ]
                // type generics
                [
                    $(
                        $lt
                    ),*
                    $($($ignore_comma)? ,)?
                    $(
                        $crate::expand_a_or_b![
                            [$($tp)?]
                            [$const_or_tp]
                        ]
                    ),*
                ]
                // where clause
                [$(where $($where_clause)*)?]
            }
            // phantom types
            [
                $({ $lt })*
                $({
                    {$(const $tp)?}
                    $crate::expand_a_or_b![
                        [$($tp)?]
                        [$const_or_tp]
                    ]
                })*
            ]
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_default_mod_value_namespace {
    (
        $vis:tt $name:ident
        $([mod_value_namespace $(= $mod_value_namespace:ident)?])?
        $([derive $($rest:tt)*])*
    ) => {
        $crate::__impl_default_mod_value_namespace_impl! {
            $vis $name
            $(mod_value_namespace $(= $mod_value_namespace)?)?
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_default_mod_value_namespace_impl {
    ($vis:tt $name:ident mod_value_namespace $($rest:tt)*) => {};
    ($vis:tt $name:ident) => {
        $crate::__impl_value_namespace! {
            []
            $vis
            $name
        }
    };
}

#[macro_export]
macro_rules! ghost {
    (
        $(#![$attr_inner_name:ident $($attr_inner:tt)*])*
        $(#[$($attr:tt)*])*
        $(pub $( ($($vis_path:tt)*) )?)?
        struct $name:ident <
            $($rest:tt)+
    ) => {
        $(
            #[allow(unused_imports)]
            use $crate::macro_docs::inner_attr::$attr_inner_name as _;
        )*

        $crate::__impl_resolve_type_generics_expand! {
            {
                {$(#![$attr_inner_name $($attr_inner)*])*}
                {$(#[$($attr)*])*}
                {$(pub $( ($($vis_path)*) )?)?}
                $name
            }
            {$($rest)+}
        }

        $crate::__impl_default_mod_value_namespace! {
            {$(pub $( ($($vis_path)*) )?)?}
            $name
            $([$attr_inner_name $($attr_inner)*])*
        }
    };
}
