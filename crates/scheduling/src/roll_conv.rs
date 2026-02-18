use std::cmp::min;

use chrono::{Datelike, NaiveDate};

use crate::period::Period;

pub trait Roll {
    /// Roll input ``dt`` to the appropriate date based on this roll convention.
    fn adjust(&self, dt: NaiveDate) -> NaiveDate;

    fn next(&self, dt: NaiveDate, period: Period) -> NaiveDate {
        self.adjust(dt + period)
    }

    fn prev(&self, dt: NaiveDate, period: Period) -> NaiveDate {
        self.adjust(dt - period)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RollConvention {
    // Roll to a specific day of the month
    DayOfMonth(u32),
    // Roll to the first calendar day of the month
    Bom,
    // Roll to the last calendar day of the month
    Eom,
    // Don't do any adjustment
    None,
}

impl Roll for RollConvention {
    fn adjust(&self, dt: NaiveDate) -> NaiveDate {
        match *self {
            Self::DayOfMonth(d) => self.day_of_month(dt, d),
            Self::Bom => dt.with_day(1).unwrap(),
            Self::Eom => dt.with_day(dt.num_days_in_month() as u32).unwrap(),
            Self::None => dt,
        }
    }
}

impl RollConvention {
    fn day_of_month(&self, dt: NaiveDate, d: u32) -> NaiveDate {
        let max = dt.num_days_in_month() as u32;
        dt.with_day(min(d, max)).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Datelike, NaiveDate};

    use super::{Roll, RollConvention};

    #[test]
    fn test_day_of_month() {
        let year = 2026;
        let cases = [
            (15u32, 31u32),
            (15u32, 28u32),
            (15u32, 31u32),
            (15u32, 30u32),
            (15u32, 31u32),
            (15u32, 30u32),
            (15u32, 31u32),
            (15u32, 31u32),
            (15u32, 30u32),
            (15u32, 31u32),
            (15u32, 30u32),
            (15u32, 31u32),
        ];

        let roll1 = RollConvention::DayOfMonth(15);
        let roll2 = RollConvention::DayOfMonth(31);

        for (i, (mid, end)) in cases.into_iter().enumerate() {
            let month = (i as u32) + 1;
            let dt = NaiveDate::from_ymd_opt(year, month, 1u32).unwrap();
            assert_eq!(roll1.adjust(dt).day(), mid);
            assert_eq!(roll2.adjust(dt).day(), end);
        }
    }

    #[test]
    fn test_eom() {
        let year = 2026;
        let cases = [
            31u32, 28u32, 31u32, 30u32, 31u32, 30u32, 31u32, 31u32, 30u32, 31u32,
            30u32, 31u32,
        ];

        let roll = RollConvention::Eom;

        for (i, end) in cases.into_iter().enumerate() {
            let month = (i as u32) + 1;
            let dt = NaiveDate::from_ymd_opt(year, month, 1u32).unwrap();
            assert_eq!(roll.adjust(dt).day(), end);
        }
    }

    #[test]
    fn test_bom() {
        let year = 2026;

        let roll = RollConvention::Bom;

        for i in 1..12 {
            let month = i as u32;
            let dt = NaiveDate::from_ymd_opt(year, month, 15u32).unwrap();
            assert_eq!(roll.adjust(dt).day(), 1);
        }
    }
}
