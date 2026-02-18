use std::ops::{Add, Sub};

use bizdate::{BusdayConvention, BusinessCalendar};
use chrono::{Days, Months, NaiveDate};

#[derive(Clone, Copy)]
pub enum Period<'a> {
    BusDays(u32, &'a BusinessCalendar),
    Days(u32),
    Months(u32),
    Weeks(u32),
    Years(u32),
}

impl<'a> Period<'a> {
    /// Get the next date after ``dt`` corresponding to the period.
    pub fn next(&self, dt: NaiveDate) -> Option<NaiveDate> {
        match *self {
            Self::BusDays(days, cal) => {
                Some(cal.add_busdays(dt, days, BusdayConvention::Following))
            }
            Self::Days(d) => dt.checked_add_days(Days::new(d as u64)),
            Self::Months(m) => dt.checked_add_months(Months::new(m)),
            Self::Weeks(d) => dt.checked_add_days(Days::new((d * 7) as u64)),
            Self::Years(m) => dt.checked_add_months(Months::new(m * 12)),
        }
    }

    /// Get the previous date before ``dt`` corresponding to the period.
    pub fn prev(&self, dt: NaiveDate) -> Option<NaiveDate> {
        match *self {
            Self::BusDays(days, cal) => {
                Some(cal.sub_busdays(dt, days, BusdayConvention::Following))
            }
            Self::Days(d) => dt.checked_sub_days(Days::new(d as u64)),
            Self::Months(m) => dt.checked_sub_months(Months::new(m)),
            Self::Weeks(d) => dt.checked_sub_days(Days::new((d * 7) as u64)),
            Self::Years(m) => dt.checked_sub_months(Months::new(m * 12)),
        }
    }

    pub fn is_days(&self) -> bool {
        matches!(self, Self::Days(_))
    }
}

// TODO: how to handle potential errors here? Unwrap seems like the best bet...?

impl<'a> Add<NaiveDate> for Period<'a> {
    type Output = NaiveDate;

    fn add(self, rhs: NaiveDate) -> Self::Output {
        self.next(rhs).unwrap()
    }
}

impl<'a> Add<Period<'a>> for NaiveDate {
    type Output = NaiveDate;

    fn add(self, rhs: Period) -> Self::Output {
        rhs.next(self).unwrap()
    }
}

impl<'a> Sub<Period<'a>> for NaiveDate {
    type Output = NaiveDate;

    fn sub(self, rhs: Period) -> Self::Output {
        rhs.prev(self).unwrap()
    }
}

// TODO: need to implement a BusinessCalendarCache in order to look up calendars from string
// impl<'a> FromStr for Period<'a> {
//     type Err = SchedulingError;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let re = Regex::new(r"^(?<count>\d+)(?<interval>[BDMWY])$").unwrap();
//         let Some(caps) = re.captures(s) else {
//             let msg = format!("Invalid Period format string {}", s);
//             return Err(SchedulingError::ParseErr(msg));
//         };

//         let count = caps["count"].parse::<u32>().unwrap();

//         match &caps["interval"] {
//             "D" => Ok(Self::Days(count)),
//             "M" => Ok(Self::Months(count)),
//             "W" => Ok(Self::Weeks(count)),
//             "Y" => Ok(Self::Years(count)),
//             &_ => Err(SchedulingError::ParseErr(format!(
//                 "Unable to parse Period {}",
//                 s
//             ))),
//         }
//     }
// }

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
