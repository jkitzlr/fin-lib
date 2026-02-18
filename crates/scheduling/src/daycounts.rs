use bizdate::BusinessCalendar;
use chrono::{Datelike, NaiveDate};

pub enum Daycounts<'t> {
    Act360,
    Act365,
    Act365F,
    ActActIsda,
    // Act365L, // TODO: requires Schedule
    Bus252(&'t BusinessCalendar),
}

impl<'t> Daycounts<'t> {
    pub fn year_fraction(&self, start: NaiveDate, end: NaiveDate) -> f64 {
        match *self {
            Self::Act360 => act360(start, end),
            Self::Act365 => act365(start, end),
            Self::Act365F => act365f(start, end),
            Self::ActActIsda => act_act_isda(start, end),
            Self::Bus252(cal) => bus252(start, end, cal),
        }
    }
}

// impl FromStr for Daycounts {
//     type Err = SchedulingError;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s {
//             "ACT/360" => Ok(Self::Act360),
//             "ACT/365F" => Ok(Self::Act365F),
//         }
//     }
// }

fn act360(start: NaiveDate, end: NaiveDate) -> f64 {
    let days = (end - start).num_days() as f64;
    days / 360.0
}

fn act365f(start: NaiveDate, end: NaiveDate) -> f64 {
    let days = (end - start).num_days() as f64;
    days / 365.0
}

fn act_act_isda(start: NaiveDate, end: NaiveDate) -> f64 {
    if start.year() == end.year() {
        return act_act_isda_sub(start, end);
    }

    let mid = NaiveDate::from_ymd_opt(end.year(), 1, 1).unwrap();
    act_act_isda_sub(start, mid) + act_act_isda_sub(mid, end)
}

fn act_act_isda_sub(start: NaiveDate, end: NaiveDate) -> f64 {
    let denom = if start.leap_year() { 366.0 } else { 365.0 };
    let days = (end - start).num_days() as f64;
    days / denom
}

fn act365(start: NaiveDate, end: NaiveDate) -> f64 {
    let next_leap = next_leap_date(start);
    let days = (end - start).num_days() as f64;
    days / if next_leap < end { 366.0 } else { 365.0 }
}

fn bus252(start: NaiveDate, end: NaiveDate, cal: &BusinessCalendar) -> f64 {
    let days = cal.busday_count(start, end) as f64;
    days / 252.0
}

fn next_leap_date(dt: NaiveDate) -> NaiveDate {
    for i in 0..8 {
        let year = dt.year() + i;
        let tmp = NaiveDate::from_ymd_opt(year, 2, 29);
        if tmp.is_some_and(|d| d > dt) {
            return tmp.unwrap();
        }
    }

    // ! this should never evaluate but satisfy thecompiler
    dt
}

#[cfg(test)]
mod tests {
    use bizdate::BusinessCalendar;
    use chrono::NaiveDate;

    use super::Daycounts;

    static HOLIDAYS: &[NaiveDate] = &[
        NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
        NaiveDate::from_ymd_opt(2026, 1, 19).unwrap(),
        NaiveDate::from_ymd_opt(2026, 2, 16).unwrap(),
        NaiveDate::from_ymd_opt(2026, 5, 25).unwrap(),
        NaiveDate::from_ymd_opt(2026, 6, 19).unwrap(),
        NaiveDate::from_ymd_opt(2026, 7, 3).unwrap(),
        NaiveDate::from_ymd_opt(2026, 9, 7).unwrap(),
        NaiveDate::from_ymd_opt(2026, 10, 12).unwrap(),
        NaiveDate::from_ymd_opt(2026, 11, 11).unwrap(),
        NaiveDate::from_ymd_opt(2026, 11, 26).unwrap(),
        NaiveDate::from_ymd_opt(2026, 12, 25).unwrap(),
    ];

    fn get_cal() -> BusinessCalendar {
        BusinessCalendar::new(Some(HOLIDAYS.iter().cloned()), "1111100")
    }

    #[test]
    fn test_bus252() {
        let start = NaiveDate::parse_from_str("20260101", "%Y%m%d").unwrap();
        let end = NaiveDate::parse_from_str("20260630", "%Y%m%d").unwrap();
        let cal = get_cal();
        let dc = Daycounts::Bus252(&cal);
        assert_eq!(dc.year_fraction(start, end), 123.0 / 252.0)
    }
}
