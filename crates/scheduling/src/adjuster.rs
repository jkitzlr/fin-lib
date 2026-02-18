use bizdate::{BusdayConvention, BusinessCalendar};
use chrono::NaiveDate;

pub struct Adjuster<'t> {
    buscal: &'t BusinessCalendar,
    conv: BusdayConvention,
}

impl<'t> Adjuster<'t> {
    pub fn new(buscal: &'t BusinessCalendar, conv: BusdayConvention) -> Self {
        Self { buscal, conv }
    }

    pub fn adjust(&self, dt: NaiveDate) -> NaiveDate {
        self.buscal.adjust(dt, self.conv)
    }
}
