use std::ops::{Add, Sub};

use chrono::NaiveDate;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Period {
    Months(u32),
}

impl Period {
    /// Get the next date after ``dt`` corresponding to the period.
    pub fn next(&self, dt: NaiveDate) -> Option<NaiveDate> {
        match *self {
            Self::Months(m) => dt.checked_add_months(chrono::Months::new(m)),
        }
    }

    /// Get the previous date before ``dt`` corresponding to the period.
    pub fn prev(&self, dt: NaiveDate) -> Option<NaiveDate> {
        match *self {
            Self::Months(m) => dt.checked_sub_months(chrono::Months::new(m)),
        }
    }
}

// TODO: how to handle potential errors here? Unwrap seems like the best bet...?

impl Add<NaiveDate> for Period {
    type Output = NaiveDate;

    fn add(self, rhs: NaiveDate) -> Self::Output {
        self.next(rhs).unwrap()
    }
}

impl Add<Period> for NaiveDate {
    type Output = NaiveDate;

    fn add(self, rhs: Period) -> Self::Output {
        rhs.next(self).unwrap()
    }
}

impl Sub<Period> for NaiveDate {
    type Output = NaiveDate;

    fn sub(self, rhs: Period) -> Self::Output {
        rhs.prev(self).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use super::Period;

    #[test]
    fn test_add_months() {
        let period = Period::Months(6);

        // ! this is almost more of a test of chrono but worth having for audit reasons

        let dt1 = NaiveDate::parse_from_str("2025-08-31", "%Y-%m-%d").unwrap();
        let rslt1 = NaiveDate::parse_from_str("2026-02-28", "%Y-%m-%d").unwrap();
        assert_eq!(dt1 + period, rslt1);

        let dt2 = NaiveDate::parse_from_str("2027-08-31", "%Y-%m-%d").unwrap();
        let rslt2 = NaiveDate::parse_from_str("2028-02-29", "%Y-%m-%d").unwrap();
        assert_eq!(dt2 + period, rslt2);
    }

    #[test]
    fn test_add_months_assoc() {
        let period = Period::Months(6);

        // ! this is almost more of a test of chrono but worth having for audit reasons

        let dt1 = NaiveDate::parse_from_str("2025-08-31", "%Y-%m-%d").unwrap();
        let rslt1 = NaiveDate::parse_from_str("2026-02-28", "%Y-%m-%d").unwrap();
        assert_eq!(period + dt1, rslt1);

        let dt2 = NaiveDate::parse_from_str("2027-08-31", "%Y-%m-%d").unwrap();
        let rslt2 = NaiveDate::parse_from_str("2028-02-29", "%Y-%m-%d").unwrap();
        assert_eq!(period + dt2, rslt2);
    }

    #[test]
    fn test_sub_months() {
        let period = Period::Months(6);

        // ! this is almost more of a test of chrono but worth having for audit reasons

        let dt1 = NaiveDate::parse_from_str("2026-02-28", "%Y-%m-%d").unwrap();
        let rslt1 = NaiveDate::parse_from_str("2025-08-28", "%Y-%m-%d").unwrap();
        assert_eq!(dt1 - period, rslt1);

        let dt2 = NaiveDate::parse_from_str("2028-02-29", "%Y-%m-%d").unwrap();
        let rslt2 = NaiveDate::parse_from_str("2027-08-29", "%Y-%m-%d").unwrap();
        assert_eq!(dt2 - period, rslt2);
    }
}
