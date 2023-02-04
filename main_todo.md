#### Main TODO/Thoughts

- instead of having `CsvRecord.id` be an `Option<usize>`, implement shadow structs
    - e.g. `PaymentPartial`, `PaymentTemp`, `PaymentShadow`, or whatever name indicates
      that this is a temporary representation of the record and it is either not yet saved
      or not guaranteed to be in sync with store.
    - This is the conceptual distinction between the cases `id = None` and `id = Some(usize)`
      the latter being "tied" to the `store` by an `id`.
    - I found myself beginning to implement a pattern in which those `CsvRecord` methods,
      which applied only when `self.id = Some(usize)` would call a prelude method
      `self.require_be_in_store()` that would simply check for the presence of an `id`.
    - But that seemed like a bad pattern.
    - For now, we'll accept a fair amount of matching on `id` and some leaning on `.unwrap()`
    - However, this does underline the positives of the
      `HashMap<id: usize, record: T>` datastructure when iterating through the elements

          for (id, record) in store.csv_records.iter() {
            if *id == my_matching_id {
              // do something with record
              // ***if we handle data correctly***
              // record.id.unwrap() should never panic here
            }
          }

    - further I go, the less I know which way is best. we'll find out

    - Answer: BTreeMap
      - .key() is automatically sorted
      - maintains identical .entry() api

- wrap `main_store` in `RefCell`
  - so far not necessary but it would be fun to have an excuse to use `RefCell`
  - long term: multi-thread complex computations wrapping `main_store` in a `Mutex` (and likely casting other data structures as their thread-safe equivalents)

- add SQL
  - likely postgres
  - pick a rust engine and/or debug sqlx install
  - provide db-init from csv directory

- generalize to accept JSON
  - through WebAssembly, you could basically provide a custom local storage compiled from Rust
  - the front-end's backend
  - if there were ever a situation where you need to
    - pull a large amount of data once
    - operate on that data
    - save results to server
    - this would allow you to pull it through the JS `.fetch` api and then pass the data to rust, provide a JS front-end over a core Rust engine
  - Ideally, you could build a stand-alone app (using something like https://github.com/tauri-apps/tauri) and a fully in-browser version from the same JS+Rust


- handle account transfers

- impliment csv_index
  - provide index data stores
  - `BTreeMap<my_searchable_column: T, record_id: usize>`

- understand why serde doesn't like deserializing into `&str`
  - you may notice that structs use `String` for name values: this is why
  - maybe because it would just be deserializing into a string and then returning a slice of the whole string and by nature serde cannot know the size of the strings it will deserialize at compile time

- originally, I was thinking that this would start as a Rust repo and transform into a TS/RustWasm repo
  - but it looks like it maybe makes more sense to fork
    - either way, this repo will continue as a CSV/JSON command-line tool
    - and then it will be forked/copied and adapted to exist in a web environment using RustWasm
  - in the RustWasm environment, we won't be storing payment events in JS, we'll be storing them in Rust
  - so really we want to store events in month bins in Rust and expose an "add payment_event(year: number, month: string)" function to JS
  - storage is `PaymentEventMonthBins = BTreeMap<YearMonth, PaymentEventStore>`
  - the ux flow is:
    - User adds an event through the UI
    - Event is stored in `PaymentEventMonthBins`
    - User hits `RunSim`
      - this function accesses the data and performs the mutations to store

- waiting to see a good use of TupleStructs
  - it's happened 2-3 times now that I've implimented a tuple struct, built things around/with it, and then went on to turn it into a regular struct
  - nice to be able to instantiate things with just `TupleStruct(val1, val2, val3)`
  - but not worth the readability cost of trading `.field_name` for `.0`
  - especially when all you need is `::new()`
  - maybe they take up less memory?
