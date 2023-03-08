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
- `cargo run -- -s {"YYYY-MM"} -e {"YYYY-MM"} -p {"/path/to/directory"} -x {"t/f"}`
  - `-s, -start_yyyy_mm` starting month
  - `-e, -end_yyyy_mm` ending month
  - `-p, -path` (Optional. Default: `data/`) path to directory containing `data/`
  - `-x, -x_test` (Optional. Default: `f`) run test from `main()`, name chosen to avoid collision
### /schema
Core `Structs`
  - `Account`
  - `Account Balance`
  - `Amount`
  - `Expense`
  - `Income`
  - `Payment`
  - `PaymentReceived`
  - `Tag` (yet to be implemented)

### /traits
Shared logic for deserializing and serializing records

  - `CSVRecord`
  - `CSVStore`
  
### /storage
Central memory
  - `Store`

### /composite
Datastructures comprised of core `Structs`
  - `AccountSummary`
  - `PaymentComposite` (`.create_payment()`)
  - `PaymentDisplay`
  - `PaymentEvent`
  - `PaymentReceivedComposite` (`.create_payment_received()`)
  - `PaymentSummary`

### /calendar
`Structs` for time-binning data
  - `CalendarSlice`
  - `Day`
  - `Month`
  
### /programs
Scripts to model transactions (`payment_events`) taking place over time
  - `CalendarSliceModel`
  - `MonthModel`
  
### /test
Initiate `Store` in unit tests
  - `Spec`
  
### /error
Basic error logging
  - `ErrorHandler`


================================
================================

##### first Rust repo
##### Nathan S
