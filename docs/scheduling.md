# Scheduling

This document outlines the general logic/considerations of generating schedules for
fixed income instruments.

There are different types of schedules one encounters with fixed income produtcts;

* accrual/pay schedules
* reset schedules
* amortization schedules
* call/put schedules

This document will attempt to outline them all.

## Periods

Periods are the building blocks of Schedules--schedules consist of several contiguous
periods of time.

Periods have a start and an end date which are not business calendar-adjusted by
default. Each of these has an adjusted counterpart where the corresponding date is
adjusted to valid business day based on an appropriate rule. If the start or end date
are already business days then the unadjusted date equals the adjusted date.

### Stubs

Stubs are irregular periods--fixed income product schedules have rules that define
key schedule dates (e.g. accrual period start/end). In the majority of cases, these
rules are fixed periods and/or frequencies per year, e.g., semi-annual (2x a year,
every 6 months), quarterly (4x/year or every 3 months), weekly, daily, etc.

Stubs are periods where the period between the start and end do not align to these rules.
For example, both legs of a USD SOFR swap may pay annually but the first payment occurs
5 months after the effective date--this is a _short stub_.

Stubs can be be long (end is after the date implied by the regular periodicity) or
short ()
