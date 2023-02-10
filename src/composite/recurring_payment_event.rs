use crate::schema::recurrance::Every;
use chrono::{Days, Months, NaiveDate};
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct RecurringPaymentEvent {
    pub id: Option<usize>,
    pub event_type: String,
    pub name: String,
    pub account_name: String,
    pub amount: Decimal,
    pub start: NaiveDate,
    pub end: NaiveDate,
    pub recurrence: Every,
}

impl RecurringPaymentEvent {
    #[allow(unused)]
    pub fn payment_dates(&self) -> Vec<NaiveDate> {
        let mut payment_dates: Vec<NaiveDate> = vec![self.start];

        if self.start == self.end {
            return payment_dates;
        }

        let mut curr_date = self.next_payment_date(self.start);
        while curr_date <= self.end {
            payment_dates.push(curr_date);
            let next_date = self.next_payment_date(curr_date);
            curr_date = next_date;
        }

        payment_dates
    }

    #[allow(unused)]
    pub fn next_payment_date(&self, last_payment_date: NaiveDate) -> NaiveDate {
        match self.recurrence {
            Every::Days(n) => last_payment_date
                .checked_add_days(Days::new(n))
                .expect("Error Computing Next Payment Event Date by Day"),
            Every::Weeks(n) => last_payment_date
                .checked_add_days(Days::new(7 * n))
                .expect("Error Computing Next Payment Event Date by Week"),
            Every::Months(n) => last_payment_date
                .checked_add_months(Months::new(n))
                .expect("Error Computing Next Payment Event Date by Month"),
            Every::Years(n) => last_payment_date
                .checked_add_months(Months::new(12 * n))
                .expect("Error Computing Next Payment Event Date by Year"),
        }
    }
}

#[cfg(test)]
mod payment_composite_spec {
    use super::*;
    use chrono::NaiveDate;

    // TODO: move to spec::factory MB-10
    // https://github.com/users/nathanielBellamy/projects/1/views/1?pane=issue&itemId=20152028
    fn reccurring_payment_event(
        start: NaiveDate,
        end: NaiveDate,
        recurrence: Every,
    ) -> RecurringPaymentEvent {
        RecurringPaymentEvent {
            id: None,
            event_type: "payment".to_string(),
            name: "dog food".to_string(),
            account_name: "piggybank".to_string(),
            amount: Decimal::new(50, 0),
            start: start,
            end: end,
            recurrence: recurrence,
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn payment_dates__returns_vector_of_naivedates_according_to_recurrence() {
        // start end chosen so that both are payment_event_dates
        let start = NaiveDate::from_ymd_opt(2023, 02, 10).unwrap();
        let end = NaiveDate::from_ymd_opt(2023, 06, 16).unwrap();
        let reccurring_payment_event = reccurring_payment_event(start, end, Every::Weeks(2));
        let payment_dates: Vec<NaiveDate> = reccurring_payment_event.payment_dates();

        assert_eq!(payment_dates.len(), 10);
        assert_eq!(payment_dates[0], reccurring_payment_event.start);
        assert_eq!(
            payment_dates[1],
            NaiveDate::from_ymd_opt(2023, 02, 24).unwrap()
        );
        assert_eq!(
            payment_dates[2],
            NaiveDate::from_ymd_opt(2023, 03, 10).unwrap()
        );
        assert_eq!(
            payment_dates[3],
            NaiveDate::from_ymd_opt(2023, 03, 24).unwrap()
        );
        assert_eq!(
            payment_dates[4],
            NaiveDate::from_ymd_opt(2023, 04, 7).unwrap()
        );
        assert_eq!(
            payment_dates[5],
            NaiveDate::from_ymd_opt(2023, 04, 21).unwrap()
        );
        assert_eq!(
            payment_dates[6],
            NaiveDate::from_ymd_opt(2023, 05, 5).unwrap()
        );
        assert_eq!(
            payment_dates[7],
            NaiveDate::from_ymd_opt(2023, 05, 19).unwrap()
        );
        assert_eq!(
            payment_dates[8],
            NaiveDate::from_ymd_opt(2023, 06, 02).unwrap()
        );
        assert_eq!(payment_dates[9], reccurring_payment_event.end);
    }

    #[test]
    #[allow(non_snake_case)]
    fn payment_dates__does_not_include_events_after_end() {
        // end chosen to be day before next_payment_event
        let start = NaiveDate::from_ymd_opt(2023, 02, 10).unwrap();
        let end = NaiveDate::from_ymd_opt(2023, 06, 15).unwrap();
        let reccurring_payment_event = reccurring_payment_event(start, end, Every::Weeks(2));
        let payment_dates: Vec<NaiveDate> = reccurring_payment_event.payment_dates();

        assert_eq!(payment_dates.len(), 9);
        assert_eq!(payment_dates[0], reccurring_payment_event.start);
        assert_eq!(
            payment_dates[8],
            NaiveDate::from_ymd_opt(2023, 06, 02).unwrap()
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn next_payment_date__handles_day_recurrence() {
        let start = NaiveDate::from_ymd_opt(2023, 02, 10).unwrap();
        let end = NaiveDate::from_ymd_opt(2023, 06, 10).unwrap();
        let recurring_payment_event = reccurring_payment_event(start, end, Every::Days(5));

        let mut next_payment_date = recurring_payment_event.next_payment_date(start);
        assert_eq!(
            next_payment_date,
            NaiveDate::from_ymd_opt(2023, 02, 15).unwrap()
        );

        next_payment_date = recurring_payment_event.next_payment_date(next_payment_date);
        assert_eq!(
            next_payment_date,
            NaiveDate::from_ymd_opt(2023, 02, 20).unwrap()
        );

        next_payment_date = recurring_payment_event.next_payment_date(next_payment_date);
        assert_eq!(
            next_payment_date,
            NaiveDate::from_ymd_opt(2023, 02, 25).unwrap()
        );

        next_payment_date = recurring_payment_event.next_payment_date(next_payment_date);
        assert_eq!(
            next_payment_date,
            NaiveDate::from_ymd_opt(2023, 03, 02).unwrap()
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn next_payment_date__handles_week_recurrence() {
        let start = NaiveDate::from_ymd_opt(2023, 02, 10).unwrap();
        let end = NaiveDate::from_ymd_opt(2023, 06, 10).unwrap();
        let recurring_payment_event = reccurring_payment_event(start, end, Every::Weeks(2));

        let mut next_payment_date = recurring_payment_event.next_payment_date(start);
        assert_eq!(
            next_payment_date,
            NaiveDate::from_ymd_opt(2023, 02, 24).unwrap()
        );

        next_payment_date = recurring_payment_event.next_payment_date(next_payment_date);
        assert_eq!(
            next_payment_date,
            NaiveDate::from_ymd_opt(2023, 03, 10).unwrap()
        );

        next_payment_date = recurring_payment_event.next_payment_date(next_payment_date);
        assert_eq!(
            next_payment_date,
            NaiveDate::from_ymd_opt(2023, 03, 24).unwrap()
        );

        next_payment_date = recurring_payment_event.next_payment_date(next_payment_date);
        assert_eq!(
            next_payment_date,
            NaiveDate::from_ymd_opt(2023, 04, 07).unwrap()
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn next_payment_date__handles_month_recurrence() {
        let start = NaiveDate::from_ymd_opt(2023, 02, 10).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 02, 10).unwrap();
        let recurring_payment_event = reccurring_payment_event(start, end, Every::Months(3));

        let mut next_payment_date = recurring_payment_event.next_payment_date(start);
        assert_eq!(
            next_payment_date,
            NaiveDate::from_ymd_opt(2023, 05, 10).unwrap()
        );

        next_payment_date = recurring_payment_event.next_payment_date(next_payment_date);
        assert_eq!(
            next_payment_date,
            NaiveDate::from_ymd_opt(2023, 08, 10).unwrap()
        );

        next_payment_date = recurring_payment_event.next_payment_date(next_payment_date);
        assert_eq!(
            next_payment_date,
            NaiveDate::from_ymd_opt(2023, 11, 10).unwrap()
        );

        next_payment_date = recurring_payment_event.next_payment_date(next_payment_date);
        assert_eq!(
            next_payment_date,
            NaiveDate::from_ymd_opt(2024, 02, 10).unwrap()
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn next_payment_date__handles_year_recurrence() {
        let start = NaiveDate::from_ymd_opt(2023, 02, 10).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 02, 10).unwrap();
        let recurring_payment_event = reccurring_payment_event(start, end, Every::Years(3));

        let mut next_payment_date = recurring_payment_event.next_payment_date(start);
        assert_eq!(
            next_payment_date,
            NaiveDate::from_ymd_opt(2026, 02, 10).unwrap()
        );

        next_payment_date = recurring_payment_event.next_payment_date(next_payment_date);
        assert_eq!(
            next_payment_date,
            NaiveDate::from_ymd_opt(2029, 02, 10).unwrap()
        );

        next_payment_date = recurring_payment_event.next_payment_date(next_payment_date);
        assert_eq!(
            next_payment_date,
            NaiveDate::from_ymd_opt(2032, 02, 10).unwrap()
        );

        next_payment_date = recurring_payment_event.next_payment_date(next_payment_date);
        assert_eq!(
            next_payment_date,
            NaiveDate::from_ymd_opt(2035, 02, 10).unwrap()
        );
    }
}
