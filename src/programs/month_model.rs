use crate::calendar::day::{Day, DayStore};
use crate::calendar::month::Month;
use crate::calendar::month_key::MonthKey as MK;
use crate::calendar::year_month::YearMonth as YM;
use crate::composite::account_summary::{AccountSummary, AccountSummaryStore};
use crate::composite::payment_composite::PaymentCompositeStore;
use crate::composite::payment_display::{PaymentDisplay, PaymentDisplayStore};
use crate::composite::payment_event::PaymentEventStore;
use crate::composite::payment_received_composite::PaymentReceivedCompositeStore;
use crate::composite::payment_summary::PaymentSummary;
use crate::composite::payment_summary::PaymentSummaryStore;
use crate::schema::expense::{Expense, ExpenseStore};
use crate::schema::income::{Income, IncomeStore};
use crate::storage::store::Store;
use crate::traits::csv_store::CsvStore;
use chrono::NaiveDate;
use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::error::Error;

pub struct MonthModel {
    year: i32,
    key: MK,
    month: Month,
    output_results: bool,
    #[allow(unused)]
    path_in: String,
    path_out: String,
}

impl MonthModel {
    pub fn new(
        year_month: YM,
        output_results: bool,
        path_in: Option<String>,
        path_out: Option<String>,
    ) -> MonthModel {
        let data_in = match path_in {
            None => "data/init/".to_string(),
            Some(path) => path,
        };

        let data_out = match path_out {
            None => "data/reports/".to_string(),
            Some(path) => path,
        };

        MonthModel {
            key: year_month.month,
            year: year_month.year,
            month: Month::new(year_month),
            output_results,
            path_in: data_in,
            path_out: data_out,
        }
    }

    // Model Payments and PaymentsReceived occuring at specific times throughout the specified month
    pub fn run(
        &mut self,
        payment_events: &PaymentEventStore,
        store_ext: Option<&mut Store>,
    ) -> Result<(), Box<dyn Error>> {
        let mut self_store = Store::new();
        let store = match store_ext {
            Some(passed_in) => passed_in,
            None => {
                self_store.init(Some(self.path_in.clone()))?;
                &mut self_store
            }
        };

        // mark all Expense/Income as inactive to begin month
        // will be marked as active when payment event recorded
        Expense::mark_all_inactive(&mut store.expenses);
        Income::mark_all_inactive(&mut store.incomes);

        self.month = Month {
            key: self.key,
            days: MonthModel::construct_days(self.year, self.key),
            year: self.year,
        };

        self.record_payment_events_in_month(payment_events);

        // iterate through the days and execute payments in order
        // each payment event mutates store
        for (_id, day) in self.month.days.iter_mut() {
            // iter sorted by key thx to btree_map
            day.execute_payments_in_order(store)?;
        }

        if self.output_results {
            let account_summary_store = self.account_summary_by_id(2);
            AccountSummary::write_to_csv(
                &account_summary_store,
                self.format_path("account_2_summary").as_str(),
            )?;

            let expense_summary = self.construct_payment_summary(&mut store.expenses);
            PaymentSummary::write_to_csv(
                &expense_summary,
                self.format_path("expense_summary").as_str(),
            )?;

            let income_summary = self.construct_payment_received_summary(&mut store.incomes);
            PaymentSummary::write_to_csv(
                &income_summary, 
                self.format_path("income_summary").as_str(),
            )?;

            let all_payment_disp_store: PaymentDisplayStore = self.month.all_payments_display();
            PaymentDisplay::write_to_csv(
                &all_payment_disp_store,
                self.format_path("all_payments").as_str(),
            )?;

            let all_payment_rec_disp_store: PaymentDisplayStore =
                self.month.all_payments_received_display();
            PaymentDisplay::write_to_csv(
                &all_payment_rec_disp_store,
                self.format_path("all_payments_received").as_str(),
            )?;

            store.write_to_csv(Some(self.path_out.clone()))?;
        }

        Ok(())
    }

    pub fn account_summary_by_id(&mut self, account_id: usize) -> AccountSummaryStore {
        // TODO  
        let mut account_summary_store = AccountSummaryStore::new();
        for (_id, day) in self.month.days.iter() {
            for (id, _completed_at, event_type) in day.payment_event_ids_chrono().iter() {
                match *event_type {
                    "payment" => {
                        let ec = day.payments.get(id).unwrap();
                        if ec.account_id.unwrap() == account_id {
                            AccountSummary::save_to_store(
                                AccountSummary {
                                   id: None,
                                   name: ec.account_name.clone(),
                                   balance: ec.ending_balance.unwrap(),
                                   reported_at: ec.payment_completed_at,
                                },
                                &mut account_summary_store
                            );
                        }
                    },
                    "payment_received" => {
                        let ec = day.payments_received.get(id).unwrap();
                        if ec.account_id.unwrap() == account_id {
                            AccountSummary::save_to_store(
                                AccountSummary {
                                    id: None,
                                    name: ec.account_name.clone(),
                                    balance: ec.ending_balance.unwrap(),
                                    reported_at: ec.payment_received_completed_at,
                                },
                                &mut account_summary_store
                            );
                        }
                    },
                    _ => (),
                };
            }
        }
        account_summary_store
    }

    pub fn format_path(&self, path: &'static str) -> String {
        format!(
            "{}{}_{}_{}.csv",
            self.path_out,
            self.month.year,
            self.month.display_number(),
            path
        )
    }

    pub fn construct_payment_summary(&self, store: &mut ExpenseStore) -> PaymentSummaryStore {
        let mut payment_summary_store = PaymentSummaryStore::new();
        for expense_id in self.month.expense_ids().iter() {
            if let Entry::Occupied(expense) = store.entry(*expense_id) {
                if expense.get().active {
                    payment_summary_store
                        .entry(*expense_id)
                        .or_insert(PaymentSummary {
                            id: Some(*expense_id),
                            name: Expense::name_by_id(*expense_id, store).to_string(),
                            total: Expense::month_total_by_id(*expense_id, &self.month),
                        });
                }
            }
        }
        payment_summary_store
    }

    pub fn construct_payment_received_summary(&self, store: &mut IncomeStore) -> PaymentSummaryStore {
        let mut payment_rec_summary_store = PaymentSummaryStore::new();
        for income_id in self.month.income_ids().iter() {
            if let Entry::Occupied(income) = store.entry(*income_id) {
                if income.get().active {
                    payment_rec_summary_store
                        .entry(*income_id)
                        .or_insert(PaymentSummary {
                            id: Some(*income_id),
                            name: Income::name_by_id(*income_id, store).to_string(),
                            total: Income::month_total_by_id(*income_id, &self.month),
                        });
                }
            }
        }
        payment_rec_summary_store
    }

    // TODO: leap years
    pub fn construct_days(year: i32, month: MK) -> DayStore {
        let length: u32 = Month::length(month);
        let month_id: u32 = Month::id(month);
        let mut days: DayStore = BTreeMap::new();

        for date in 1..length + 1 {
            let id = usize::try_from(date).unwrap();
            days.entry(id).or_insert(Day {
                id: Some(id),
                payments: PaymentCompositeStore::new(),
                payments_received: PaymentReceivedCompositeStore::new(),
                date: NaiveDate::from_ymd_opt(year, month_id, date).unwrap(),
            });
        }

        days
    }

    pub fn record_payment_events_in_month(&mut self, payment_events: &PaymentEventStore) {
        for (_id, payment_event) in payment_events.iter() {
            for (_id, day) in self.month.days.iter_mut() {
                // inner loop is O(1)
                if payment_event.completed_at.date() == day.date {
                    day.add_payment_event(payment_event);
                }
            }
        }
    }
}

#[cfg(test)]
mod month_model_spec {
    use super::*;
    // use crate::test::spec::Spec;
    // use chrono::NaiveDate;

    pub fn model() -> MonthModel {
        MonthModel::new(
            YM::new(2023, MK::Feb),
            false,
            Some("src/test/data/init".to_string()),
            None,
        )
    }

    //TODO: specs
    #[test]
    #[allow(non_snake_case)]
    fn record_payment_events_in_month__adds_payment_composites_to_days() {
        let _model = model();
    }

    #[test]
    #[allow(non_snake_case)]
    fn construct_days__returns_daystore_of_length_equal_to_days_in_month() {
        let jan_days = MonthModel::construct_days(2023, MK::Jan);
        assert_eq!(31, jan_days.len());
        let feb_days = MonthModel::construct_days(2023, MK::Feb);
        assert_eq!(28, feb_days.len());
        let mar_days = MonthModel::construct_days(2023, MK::Mar);
        assert_eq!(31, mar_days.len());
        let apr_days = MonthModel::construct_days(2023, MK::Apr);
        assert_eq!(30, apr_days.len());
        let may_days = MonthModel::construct_days(2023, MK::May);
        assert_eq!(31, may_days.len());
        let jun_days = MonthModel::construct_days(2023, MK::Jun);
        assert_eq!(30, jun_days.len());
        let jul_days = MonthModel::construct_days(2023, MK::Jul);
        assert_eq!(31, jul_days.len());
        let aug_days = MonthModel::construct_days(2023, MK::Aug);
        assert_eq!(31, aug_days.len());
        let sep_days = MonthModel::construct_days(2023, MK::Sep);
        assert_eq!(30, sep_days.len());
        let oct_days = MonthModel::construct_days(2023, MK::Oct);
        assert_eq!(31, oct_days.len());
        let nov_days = MonthModel::construct_days(2023, MK::Nov);
        assert_eq!(30, nov_days.len());
        let dec_days = MonthModel::construct_days(2023, MK::Dec);
        assert_eq!(31, dec_days.len());
    }
}
