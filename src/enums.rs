#![allow(clippy::zero_prefixed_literal)]

use std::convert::TryFrom;

macro_rules! create_numerical_enums {($($name:tt ($repr_type:tt) {$($value:tt),*})*) => (paste::paste! {
    $(

        #[repr($repr_type)]
        #[derive(Debug, Clone, Copy)]
        pub enum $name {
            $([<$name$value>] = $value,)*
        }

        impl TryFrom<$repr_type> for $name {
            type Error = String;

            fn try_from(value: $repr_type) -> Result<Self, Self::Error> {
                match value {
                    $($value => Ok($name::[<$name$value>]),)*
                    _ => Err(format!("Invalid {}: {}", stringify!($name), value)),
                }
            }
        }

        impl std::str::FromStr for $name {
            type Err = String;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                let numerical_value = value.parse::<$repr_type>().map_err(|_| {
                    format!("Invalid {}: {}", stringify!($name), value)
                })?;

                Self::try_from(numerical_value).map_err(|err| err.to_string())
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    $($name::[<$name$value>] => write!(f, "{}", $value),)*
                }
            }
        }
    )*
})}

create_numerical_enums!(
    Year(u32) {2015, 2016, 2017, 2018, 2019, 2020, 2021}

    Day(u8) {
        01, 02, 03, 04, 05,
        06, 07, 08, 09, 10,
        11, 12, 13, 14, 15,
        16, 17, 18, 19, 20,
        21, 22, 23, 24, 25
    }

    Part(u8) {1, 2}
);
