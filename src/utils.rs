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
