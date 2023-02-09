# MonthlyBudget
## [Github Project](https://github.com/users/nathanielBellamy/projects/1/views/1)

## intake budgetary CSV/JSON -> manipulate data in memory -> export report CSV/JSON
#### serde -> BTreeMap -> serde

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
