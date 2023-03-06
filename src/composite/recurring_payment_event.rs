use crate::calendar::calendar_slice::CalendarSlice;
use crate::calendar::month::Month;
use crate::calendar::year_month::YearMonth as YM;
use crate::composite::payment_event::PaymentEvent;
use crate::composite::payment_event::PaymentEventBinStore;
use crate::schema::recurrance::Every;
use crate::traits::csv_store::CsvStore;
use chrono::Datelike;
use chrono::{Days, Months, NaiveDate};
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;

use super::payment_event::RecurrenceState;

#[derive(Deserialize, Serialize, Debug)]
pub struct RecurringPaymentEvent {
    pub id: Option<usize>,
    pub event_type: String,
    pub name: String,
    pub account_name: String,
    pub amount: Decimal,
    pub start: NaiveDate,
    pub end: NaiveDate, // TODO: turn into Option<NaiveDate>
    pub recurrence: Every,
}

pub type RecurringPaymentEventBinResult = Result<(), Box<dyn Error>>;
pub type RecurringPaymentEventFetchResult = Result<Vec<RecurringPaymentEvent>, Box<dyn Error>>;

impl RecurringPaymentEvent {
    pub fn fetch_events(path: String) -> RecurringPaymentEventFetchResult {
        let data: String = fs::read_to_string(path)?.parse()?;
        let recc_payment_events: Vec<RecurringPaymentEvent> = serde_json::from_str(&data)?;
        Ok(recc_payment_events)
    }

    pub fn fetch_and_bin_recurring_events(
        path: String,
        cal_slice: &CalendarSlice,
        bin_store: &mut PaymentEventBinStore,
    ) -> RecurringPaymentEventBinResult {
        let recc_payment_events = RecurringPaymentEvent::fetch_events(path)?;
        for recc_payment_event in recc_payment_events.into_iter() {
            let payment_events = recc_payment_event.payment_events(cal_slice);
            for payment_event in payment_events.into_iter() {
                let ym = YM::new(
                    payment_event.completed_at.year(),
                    Month::key_from_id(payment_event.completed_at.month()),
                );
                // YearMonth impl Eq, PartialEq, PartialOrd, Ord
                if cal_slice.start <= ym && ym <= cal_slice.end {
                    let store = bin_store.entry(ym).or_default();
                    PaymentEvent::save_to_store(payment_event, store);
                }
            }
        }
        Ok(())
    }

    pub fn payment_events(&self, cal_slice: &CalendarSlice) -> Vec<PaymentEvent> {
        let mut payment_events: Vec<PaymentEvent> = self
            .payment_dates(cal_slice)
            .into_iter()
            .map(|date| self.to_payment_event(&date))
            .collect();
        payment_events[0].recurrence_state = RecurrenceState::First;
        if let Some(payment_event) = payment_events.last_mut() {
            payment_event.recurrence_state = RecurrenceState::Last;
        }
        // println!("{payment_events:#?}");
        payment_events
    }

    pub fn to_payment_event(&self, date: &NaiveDate) -> PaymentEvent {
        PaymentEvent {
            id: None,
            event_type: self.event_type.clone(),
            name: self.name.clone(),
            account_name: self.account_name.clone(),
            amount: self.amount,
            completed_at: date.and_hms_opt(12, 0, 0).unwrap(), // TODO: consider how to handle time
            recurrence_state: RecurrenceState::Active,
        }
    }

    pub fn payment_dates(&self, cal_slice: &CalendarSlice) -> Vec<NaiveDate> {
        let mut payment_dates: Vec<NaiveDate> = vec![self.start];

        if self.start == self.end {
            return payment_dates;
        }

        let mut curr_date = self.next_payment_date(self.start);
        while curr_date <= self.end && curr_date < cal_slice.end.start_of_next_month() {
            payment_dates.push(curr_date);
            let next_date = self.next_payment_date(curr_date);
            curr_date = next_date;
        }

        payment_dates
    }

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
    use crate::calendar::month_key::MonthKey as MK;
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
            start,
            end,
            recurrence,
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn payment_events__returns_vector_of_payment_events_according_to_recurrence() {
        // start end chosen so that both are payment_event_dates
        let start = NaiveDate::from_ymd_opt(2023, 02, 10).unwrap();
        let end = NaiveDate::from_ymd_opt(2023, 06, 16).unwrap();
        let cal_slice = CalendarSlice::new(YM::new(2023, MK::Feb), YM::new(2023, MK::Jun)).unwrap();
        let reccurring_payment_event = reccurring_payment_event(start, end, Every::Weeks(2));
        let payment_events: Vec<PaymentEvent> = reccurring_payment_event.payment_events(&cal_slice);

        assert_eq!(payment_events.len(), 10);
        for payment_event in payment_events.iter() {
            assert_eq!(payment_event.event_type, "payment".to_string());
            assert_eq!(payment_event.name, "dog food".to_string());
            assert_eq!(payment_event.account_name, "piggybank".to_string());
            assert_eq!(payment_event.amount, Decimal::new(50, 0));
        }
        assert_eq!(
            payment_events[0].completed_at,
            NaiveDate::from_ymd_opt(2023, 02, 10)
                .unwrap()
                .and_hms_opt(12, 0, 0)
                .unwrap()
        );
        assert_eq!(
            payment_events[1].completed_at,
            NaiveDate::from_ymd_opt(2023, 02, 24)
                .unwrap()
                .and_hms_opt(12, 0, 0)
                .unwrap()
        );
        assert_eq!(
            payment_events[2].completed_at,
            NaiveDate::from_ymd_opt(2023, 03, 10)
                .unwrap()
                .and_hms_opt(12, 0, 0)
                .unwrap()
        );
        assert_eq!(
            payment_events[3].completed_at,
            NaiveDate::from_ymd_opt(2023, 03, 24)
                .unwrap()
                .and_hms_opt(12, 0, 0)
                .unwrap()
        );
        assert_eq!(
            payment_events[4].completed_at,
            NaiveDate::from_ymd_opt(2023, 04, 07)
                .unwrap()
                .and_hms_opt(12, 0, 0)
                .unwrap()
        );
        assert_eq!(
            payment_events[5].completed_at,
            NaiveDate::from_ymd_opt(2023, 04, 21)
                .unwrap()
                .and_hms_opt(12, 0, 0)
                .unwrap()
        );
        assert_eq!(
            payment_events[6].completed_at,
            NaiveDate::from_ymd_opt(2023, 05, 05)
                .unwrap()
                .and_hms_opt(12, 0, 0)
                .unwrap()
        );
        assert_eq!(
            payment_events[7].completed_at,
            NaiveDate::from_ymd_opt(2023, 05, 19)
                .unwrap()
                .and_hms_opt(12, 0, 0)
                .unwrap()
        );
        assert_eq!(
            payment_events[8].completed_at,
            NaiveDate::from_ymd_opt(2023, 06, 02)
                .unwrap()
                .and_hms_opt(12, 0, 0)
                .unwrap()
        );
        assert_eq!(
            payment_events[9].completed_at,
            NaiveDate::from_ymd_opt(2023, 06, 16)
                .unwrap()
                .and_hms_opt(12, 0, 0)
                .unwrap()
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn payment_dates__returns_vector_of_naivedates_according_to_recurrence() {
        // start end chosen so that both are payment_event_dates
        let start = NaiveDate::from_ymd_opt(2023, 02, 10).unwrap();
        let end = NaiveDate::from_ymd_opt(2023, 06, 16).unwrap();
        let cal_slice = CalendarSlice::new(YM::new(2023, MK::Feb), YM::new(2023, MK::Jun)).unwrap();
        let reccurring_payment_event = reccurring_payment_event(start, end, Every::Weeks(2));
        let payment_dates: Vec<NaiveDate> = reccurring_payment_event.payment_dates(&cal_slice);

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
        let cal_slice = CalendarSlice::new(YM::new(2023, MK::Feb), YM::new(2023, MK::Jun)).unwrap();
        let payment_dates: Vec<NaiveDate> = reccurring_payment_event.payment_dates(&cal_slice);

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
