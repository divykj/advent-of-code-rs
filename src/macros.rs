macro_rules! include_years {($($year:tt)*) => (paste::paste! {
    $(mod [<year_$year>];)*

    #[allow(clippy::zero_prefixed_literal)]
    pub fn solve(year: $crate::enums::Year, day: $crate::enums::Day, part: $crate::enums::Part) -> Result<String, $crate::util::output::Unsolved> {
        match year {
            $($crate::enums::Year::[<Year$year>] => {
                Ok(format!("{}", $crate::[<year_$year>]::solve(day, part)?))
            },)*
            _ => Err($crate::util::output::Unsolved::Year),
        }
    }
})}

macro_rules! include_days {($($day:tt)*) => (paste::paste! {
    $(mod [<day_$day>];)*

    #[allow(clippy::zero_prefixed_literal)]
    pub(crate) fn solve(day: $crate::enums::Day, part: $crate::enums::Part) -> Result<String, $crate::util::output::Unsolved> {
        match day {
            $($crate::enums::Day::[<Day$day>] => {
                Ok(format!("{}", [<day_$day>]::solve(part)?))
            },)*
            _ => Err($crate::util::output::Unsolved::Day),
        }
    }
})}

pub(crate) use include_days;
pub(crate) use include_years;
