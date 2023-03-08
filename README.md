# MonthlyBudget

## intake budgetary CSV/JSON -> manipulate data in memory -> export report CSV/JSON
#### CSV --serde--> BTreeMap --serde--> CSV

## data setup
- `data/`
  - `events` => user-entered payment events
    - `one_off.json`
    - `recurring.json`
  - `init/` => provide initial values
    - `account_balances.csv`
    - `accounts.csv`
    - `amounts.csv`
    - `expenses.csv`
    - `incomes.csv`
    - `payments.csv`
    - `payments_received.csv`
  - `reports/` => output

## to run
- `cargo run -- -s {"YYYY-MM"} -e {"YYYY-MM"} -p {"/path/to/directory/data"} -x {"t/f"}`
  - `-s, -start_yyyy_mm` starting month
  - `-e, -end_yyyy_mm` ending month
  - `-p, -path` 
    - path to `data` directory
    - Optional. Default: `data/` 
  - `-x, -x_test` 
    - run test from `main()`
    - name chosen to avoid collision
    - Optional. Default: `f` 

## file structure
- `data/` (see above)
- `src/`
  - `app/`
    - `cli.rs`
  - `calendar/`
    - `calendar_slice.rs`
    - `day.rs`
    - `month.rs`
    - `month_key.rs`
    - `year_month.rs`
  - `composite/`
    - `account_summary.rs`
    - `payment_composite.rs`
    - `payment_display.rs`
    - `payment_event.rs`
    - `payment_received_composite.rs`
    - `payment_summary.rs`
    - `recurring_payment_event.rs`
  - `error/`
    - `error_handler.rs`
    - `error_log`
  - `programs/`
    - `calendar_slice_model.rs`
    - `month_model.rs`
  - `schema/`
    - `account.rs`
    - `account_balance.rs`
    - `amount.rs`
    - `expense.rs`
    - `income.rs`
    - `payment.rs`
    - `payment_received.rs`
  - `storage/`
    - `store.rs`
  - `test/`
    - `data/`
      - `events`
      - `init`
      - `reports`
    - `end_to_end/`
      - `calendar_slice_model`
        - `data`
          - `events`
          - `init`
          - `reports`
        - `csm_test.rs`
    - `spec.rs`
  - `traits/`
    - `csv_record.rs`
    - `csv_store.rs`
    - `file_io.rs`
  

================================
================================

##### Nathan S
This code is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).

