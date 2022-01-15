use std::fmt::Display;
use std::fmt::Write;
use std::iter::Iterator;

macro_rules! named_enum {
    ($(#[$attributes:meta])*
        pub enum $name:ident {
        $(#[$doc:meta] $variant:ident),*,
    }) => {
        $(#[$attributes])*
        pub enum $name {
            $(#[$doc] $variant),*
        }

        impl $name {
            /// Get the name of the enum variant.
            pub fn name(&self) -> &'static str {
                match self {
                    $($name::$variant => stringify!($variant)),*
                }
            }
        }
    };
}

pub trait Join {
    fn join(&mut self, separator: &str) -> String;
}

impl<I> Join for I
where
    I: Iterator,
    I::Item: Display,
{
    fn join(&mut self, separator: &str) -> String {
        let mut joined = String::new();
        for (i, item) in self.enumerate() {
            if i > 0 {
                joined.push_str(separator);
            }
            write!(joined, "{}", item).unwrap();
        }
        joined
    }
}
