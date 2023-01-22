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
