# MonthlyBudget

## intake budgetary CSV/JSON -> manipulate data in memory -> export report CSV/JSON
#### serde -> BTreeMap -> serde

## to run
- `cargo run -- --startym YYYY-MM --endym YYYY-MM`
- or `cargo run -- -s YYYY-MM -e YYYY-MM`
- can pass in `--transactions -t` folder for different payment events
  - add `{my_payment_events}.json` to folder `/data/json/{my_payment_events_directory}/`
  - by default uses payment events in `/data/json/example_1/payment_events.json`

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
