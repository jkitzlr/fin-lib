use bizdate::{BusdayConvention, BusinessCalendar};
use chrono::NaiveDate;

use crate::period::Period;
use crate::roll_conv::{Roll, RollConvention};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SchedulePeriod {
    pub(crate) start: NaiveDate,
    pub(crate) end: NaiveDate,
    pub(crate) start_adj: NaiveDate,
    pub(crate) end_adj: NaiveDate,
}

impl SchedulePeriod {
    pub fn new(
        start: NaiveDate,
        end: NaiveDate,
        start_adj: NaiveDate,
        end_adj: NaiveDate,
    ) -> Self {
        Self {
            start,
            end,
            start_adj,
            end_adj,
        }
    }

    pub fn new_adjust(
        start: NaiveDate,
        end: NaiveDate,
        buscal: &BusinessCalendar,
        conv: BusdayConvention,
    ) -> Self {
        let start_adj = buscal.adjust(start, conv);
        let end_adj = buscal.adjust(end, conv);
        Self {
            start,
            end,
            start_adj,
            end_adj,
        }
    }

    pub fn new_relative(
        start: NaiveDate,
        roll_conv: RollConvention,
        period: Period,
        buscal: &BusinessCalendar,
        conv: BusdayConvention,
    ) -> Self {
        let end = roll_conv.next(start, period);
        let start_adj = buscal.adjust(start, conv);
        let end_adj = buscal.adjust(end, conv);
        Self {
            start,
            end,
            start_adj,
            end_adj,
        }
    }

    /// Check if ``dt`` falls within the period, i.e. $dt \in [start, end)$
    pub fn contains(&self, dt: NaiveDate) -> bool {
        dt >= self.start && dt < self.end
    }

    /// Check if ``dt`` falls within the adjusted period, i.e. $dt \in [astart, aend)$
    pub fn contains_adj(&self, dt: NaiveDate) -> bool {
        dt >= self.start_adj && dt < self.end_adj
    }

    /// Check whether the period is a long stub
    pub fn is_long_stub(&self, roll_conv: RollConvention, period: Period) -> bool {
        self.end > roll_conv.next(self.start, period)
    }

    /// Check whether the period is a short stub
    pub fn is_short_stub(&self, roll_conv: RollConvention, period: Period) -> bool {
        self.end < roll_conv.next(self.start, period)
    }

    pub fn is_stub(&self, roll_conv: RollConvention, period: Period) -> bool {
        !self.is_regular(roll_conv, period)
    }

    pub fn is_regular(&self, roll_conv: RollConvention, period: Period) -> bool {
        roll_conv.prev(self.end, period) == self.start
            && roll_conv.next(self.start, period) == self.end
    }
}

#[cfg(test)]
mod tests {
    use bizdate::{BusdayConvention, BusinessCalendar};
    use chrono::NaiveDate;

    use crate::{Period, RollConvention};

    use super::SchedulePeriod;

    static REG: &SchedulePeriod = &SchedulePeriod {
        start: NaiveDate::from_ymd_opt(2026, 2, 28).unwrap(),
        end: NaiveDate::from_ymd_opt(2026, 8, 31).unwrap(),
        start_adj: NaiveDate::from_ymd_opt(2026, 2, 27).unwrap(),
        end_adj: NaiveDate::from_ymd_opt(2026, 8, 31).unwrap(),
    };

    static SHORT: &SchedulePeriod = &SchedulePeriod {
        start: NaiveDate::from_ymd_opt(2026, 2, 28).unwrap(),
        end: NaiveDate::from_ymd_opt(2026, 6, 30).unwrap(),
        start_adj: NaiveDate::from_ymd_opt(2026, 2, 27).unwrap(),
        end_adj: NaiveDate::from_ymd_opt(2026, 6, 30).unwrap(),
    };

    static LONG: &SchedulePeriod = &SchedulePeriod {
        start: NaiveDate::from_ymd_opt(2026, 2, 28).unwrap(),
        end: NaiveDate::from_ymd_opt(2026, 10, 30).unwrap(),
        start_adj: NaiveDate::from_ymd_opt(2026, 2, 27).unwrap(),
        end_adj: NaiveDate::from_ymd_opt(2026, 10, 30).unwrap(),
    };

    #[test]
    fn test_reg_period() {
        let roll_conv = RollConvention::Eom;
        let period = Period::Months(6);
        assert!(REG.is_regular(roll_conv, period));
        assert!(!REG.is_stub(roll_conv, period));
        assert!(!REG.is_long_stub(roll_conv, period));
        assert!(!REG.is_short_stub(roll_conv, period));
    }

    #[test]
    fn test_short_period() {
        let roll_conv = RollConvention::Eom;
        let period = Period::Months(6);
        assert!(!SHORT.is_regular(roll_conv, period));
        assert!(SHORT.is_stub(roll_conv, period));
        assert!(!SHORT.is_long_stub(roll_conv, period));
        assert!(SHORT.is_short_stub(roll_conv, period));
    }

    #[test]
    fn test_long_period() {
        let roll_conv = RollConvention::Eom;
        let period = Period::Months(6);
        assert!(!LONG.is_regular(roll_conv, period));
        assert!(LONG.is_stub(roll_conv, period));
        assert!(LONG.is_long_stub(roll_conv, period));
        assert!(!LONG.is_short_stub(roll_conv, period));
    }

    #[test]
    fn test_new_adjust() {
        let cal = BusinessCalendar::new(None::<Vec<NaiveDate>>, "1111100");

        let start = NaiveDate::from_ymd_opt(2026, 2, 28).unwrap();
        let end = NaiveDate::from_ymd_opt(2026, 8, 31).unwrap();
        let rslt = SchedulePeriod::new_adjust(
            start,
            end,
            &cal,
            BusdayConvention::ModifiedFollowing,
        );
        assert_eq!(&rslt, REG);
    }

    #[test]
    fn test_new_relative() {
        let cal = BusinessCalendar::new(None::<Vec<NaiveDate>>, "1111100");
        let roll_conv = RollConvention::Eom;
        let period = Period::Months(6);

        let start = NaiveDate::from_ymd_opt(2026, 2, 28).unwrap();

        let rslt = SchedulePeriod::new_relative(
            start,
            roll_conv,
            period,
            &cal,
            BusdayConvention::ModifiedFollowing,
        );
        assert_eq!(&rslt, REG);
    }
}
