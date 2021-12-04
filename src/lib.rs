mod enums;
mod macros;
mod util;

use crate::macros::include_years;

pub mod prelude {
    pub use crate::enums::{Day, Part, Year};
    pub use crate::solve;
    pub use crate::util::output::Unsolved;
}

include_years![2021];
