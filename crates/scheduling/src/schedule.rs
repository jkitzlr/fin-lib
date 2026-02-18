use bizdate::{BusdayConvention, BusinessCalendar};
use chrono::{Datelike, NaiveDate};

use crate::{Period, Roll, RollConvention, SchedulePeriod};

#[derive(Clone, Debug, PartialEq)]
pub struct Schedule {
    periods: Vec<SchedulePeriod>,
    // period: Period<'a>,
    // roll_conv: RollConvention,
}

impl Schedule {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        effective: NaiveDate,
        front_stub: Option<NaiveDate>,
        back_stub: Option<NaiveDate>,
        termination: NaiveDate,
        pmt_cal: &BusinessCalendar,
        busday_conv: BusdayConvention,
        period: Period,
        bom: bool,
        eom: bool,
    ) -> Self {
        let mut periods = Vec::new();

        // * handle front stub
        if let Some(end) = front_stub {
            periods.push(SchedulePeriod::new_adjust(
                effective,
                end,
                pmt_cal,
                busday_conv,
            ));
        }

        let start = front_stub.unwrap_or(effective);
        let end = back_stub.unwrap_or(termination);

        let roll_conv = if period.is_days() {
            RollConvention::None
        } else if bom {
            RollConvention::Bom
        } else if eom {
            RollConvention::Eom
        } else {
            RollConvention::DayOfMonth(start.day())
        };

        let mut tmp = start;
        while tmp < end {
            let start = tmp;
            let end = roll_conv.next(start, period);

            let period = SchedulePeriod::new_adjust(start, end, pmt_cal, busday_conv);
            periods.push(period);

            tmp = end;
        }

        // handle back stub--end of schedule thus far ends at back stub date
        if let Some(start) = back_stub {
            periods.push(SchedulePeriod::new_adjust(
                start,
                termination,
                pmt_cal,
                busday_conv,
            ));
        }

        Self {
            periods,
            // period,
            // roll_conv,
        }
    }
}

#[cfg(test)]
mod tests {
    use bizdate::{BusdayConvention, BusinessCalendar};
    use chrono::NaiveDate;

    use crate::{Period, SchedulePeriod};

    use super::Schedule;

    #[test]
    fn test_regular() {
        let cal = BusinessCalendar::new(None::<Vec<NaiveDate>>, "1111100");
        let effective = NaiveDate::from_ymd_opt(2026, 2, 15).unwrap();
        let termination = NaiveDate::from_ymd_opt(2028, 2, 15).unwrap();
        let period = Period::Months(6);
        let busday_conv = BusdayConvention::Following;
        let sch = Schedule::new(
            effective,
            None,
            None,
            termination,
            &cal,
            busday_conv,
            period,
            false,
            false,
        );
        let periods = vec![
            SchedulePeriod::new_adjust(
                NaiveDate::from_ymd_opt(2026, 2, 15).unwrap(),
                NaiveDate::from_ymd_opt(2026, 8, 15).unwrap(),
                &cal,
                busday_conv,
            ),
            SchedulePeriod::new_adjust(
                NaiveDate::from_ymd_opt(2026, 8, 15).unwrap(),
                NaiveDate::from_ymd_opt(2027, 2, 15).unwrap(),
                &cal,
                busday_conv,
            ),
            SchedulePeriod::new_adjust(
                NaiveDate::from_ymd_opt(2027, 2, 15).unwrap(),
                NaiveDate::from_ymd_opt(2027, 8, 15).unwrap(),
                &cal,
                busday_conv,
            ),
            SchedulePeriod::new_adjust(
                NaiveDate::from_ymd_opt(2027, 8, 15).unwrap(),
                NaiveDate::from_ymd_opt(2028, 2, 15).unwrap(),
                &cal,
                busday_conv,
            ),
        ];
        assert_eq!(Schedule { periods }, sch)
    }

    #[test]
    fn test_long_front() {
        let cal = BusinessCalendar::new(None::<Vec<NaiveDate>>, "1111100");
        let effective = NaiveDate::from_ymd_opt(2025, 12, 15).unwrap();
        let front_stub = NaiveDate::from_ymd_opt(2026, 8, 15).unwrap();
        let termination = NaiveDate::from_ymd_opt(2028, 2, 15).unwrap();
        let period = Period::Months(6);
        let busday_conv = BusdayConvention::Following;
        let sch = Schedule::new(
            effective,
            Some(front_stub),
            None,
            termination,
            &cal,
            busday_conv,
            period,
            false,
            false,
        );
        let periods = vec![
            SchedulePeriod::new_adjust(
                NaiveDate::from_ymd_opt(2025, 12, 15).unwrap(),
                NaiveDate::from_ymd_opt(2026, 8, 15).unwrap(),
                &cal,
                busday_conv,
            ),
            SchedulePeriod::new_adjust(
                NaiveDate::from_ymd_opt(2026, 8, 15).unwrap(),
                NaiveDate::from_ymd_opt(2027, 2, 15).unwrap(),
                &cal,
                busday_conv,
            ),
            SchedulePeriod::new_adjust(
                NaiveDate::from_ymd_opt(2027, 2, 15).unwrap(),
                NaiveDate::from_ymd_opt(2027, 8, 15).unwrap(),
                &cal,
                busday_conv,
            ),
            SchedulePeriod::new_adjust(
                NaiveDate::from_ymd_opt(2027, 8, 15).unwrap(),
                NaiveDate::from_ymd_opt(2028, 2, 15).unwrap(),
                &cal,
                busday_conv,
            ),
        ];
        assert_eq!(Schedule { periods }, sch)
    }

    #[test]
    fn test_short_front() {
        let cal = BusinessCalendar::new(None::<Vec<NaiveDate>>, "1111100");
        let effective = NaiveDate::from_ymd_opt(2026, 4, 30).unwrap();
        let front_stub = NaiveDate::from_ymd_opt(2026, 8, 15).unwrap();
        let termination = NaiveDate::from_ymd_opt(2028, 2, 15).unwrap();
        let period = Period::Months(6);
        let busday_conv = BusdayConvention::Following;
        let sch = Schedule::new(
            effective,
            Some(front_stub),
            None,
            termination,
            &cal,
            busday_conv,
            period,
            false,
            false,
        );
        let periods = vec![
            SchedulePeriod::new_adjust(
                NaiveDate::from_ymd_opt(2026, 4, 30).unwrap(),
                NaiveDate::from_ymd_opt(2026, 8, 15).unwrap(),
                &cal,
                busday_conv,
            ),
            SchedulePeriod::new_adjust(
                NaiveDate::from_ymd_opt(2026, 8, 15).unwrap(),
                NaiveDate::from_ymd_opt(2027, 2, 15).unwrap(),
                &cal,
                busday_conv,
            ),
            SchedulePeriod::new_adjust(
                NaiveDate::from_ymd_opt(2027, 2, 15).unwrap(),
                NaiveDate::from_ymd_opt(2027, 8, 15).unwrap(),
                &cal,
                busday_conv,
            ),
            SchedulePeriod::new_adjust(
                NaiveDate::from_ymd_opt(2027, 8, 15).unwrap(),
                NaiveDate::from_ymd_opt(2028, 2, 15).unwrap(),
                &cal,
                busday_conv,
            ),
        ];
        assert_eq!(Schedule { periods }, sch)
    }

    #[test]
    fn test_long_back() {
        let cal = BusinessCalendar::new(None::<Vec<NaiveDate>>, "1111100");
        let effective = NaiveDate::from_ymd_opt(2026, 2, 15).unwrap();
        let back_stub = NaiveDate::from_ymd_opt(2027, 8, 15).unwrap();
        let termination = NaiveDate::from_ymd_opt(2028, 6, 15).unwrap();
        let period = Period::Months(6);
        let busday_conv = BusdayConvention::Following;
        let sch = Schedule::new(
            effective,
            None,
            Some(back_stub),
            termination,
            &cal,
            busday_conv,
            period,
            false,
            false,
        );
        let periods = vec![
            SchedulePeriod::new_adjust(
                NaiveDate::from_ymd_opt(2026, 2, 15).unwrap(),
                NaiveDate::from_ymd_opt(2026, 8, 15).unwrap(),
                &cal,
                busday_conv,
            ),
            SchedulePeriod::new_adjust(
                NaiveDate::from_ymd_opt(2026, 8, 15).unwrap(),
                NaiveDate::from_ymd_opt(2027, 2, 15).unwrap(),
                &cal,
                busday_conv,
            ),
            SchedulePeriod::new_adjust(
                NaiveDate::from_ymd_opt(2027, 2, 15).unwrap(),
                NaiveDate::from_ymd_opt(2027, 8, 15).unwrap(),
                &cal,
                busday_conv,
            ),
            SchedulePeriod::new_adjust(
                NaiveDate::from_ymd_opt(2027, 8, 15).unwrap(),
                NaiveDate::from_ymd_opt(2028, 6, 15).unwrap(),
                &cal,
                busday_conv,
            ),
        ];
        assert_eq!(Schedule { periods }, sch)
    }

    #[test]
    fn test_short_back() {
        let cal = BusinessCalendar::new(None::<Vec<NaiveDate>>, "1111100");
        let effective = NaiveDate::from_ymd_opt(2026, 2, 15).unwrap();
        let back_stub = NaiveDate::from_ymd_opt(2028, 2, 15).unwrap();
        let termination = NaiveDate::from_ymd_opt(2028, 6, 15).unwrap();
        let period = Period::Months(6);
        let busday_conv = BusdayConvention::Following;
        let sch = Schedule::new(
            effective,
            None,
            Some(back_stub),
            termination,
            &cal,
            busday_conv,
            period,
            false,
            false,
        );
        let periods = vec![
            SchedulePeriod::new_adjust(
                NaiveDate::from_ymd_opt(2026, 2, 15).unwrap(),
                NaiveDate::from_ymd_opt(2026, 8, 15).unwrap(),
                &cal,
                busday_conv,
            ),
            SchedulePeriod::new_adjust(
                NaiveDate::from_ymd_opt(2026, 8, 15).unwrap(),
                NaiveDate::from_ymd_opt(2027, 2, 15).unwrap(),
                &cal,
                busday_conv,
            ),
            SchedulePeriod::new_adjust(
                NaiveDate::from_ymd_opt(2027, 2, 15).unwrap(),
                NaiveDate::from_ymd_opt(2027, 8, 15).unwrap(),
                &cal,
                busday_conv,
            ),
            SchedulePeriod::new_adjust(
                NaiveDate::from_ymd_opt(2027, 8, 15).unwrap(),
                NaiveDate::from_ymd_opt(2028, 2, 15).unwrap(),
                &cal,
                busday_conv,
            ),
            SchedulePeriod::new_adjust(
                NaiveDate::from_ymd_opt(2028, 2, 15).unwrap(),
                NaiveDate::from_ymd_opt(2028, 6, 15).unwrap(),
                &cal,
                busday_conv,
            ),
        ];
        assert_eq!(Schedule { periods }, sch)
    }
}
