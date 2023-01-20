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
    - But that seemed like a bad pattern for many reasons.
    - For now, we'll accept a fair amount of matching on `id` and some leaning on `.unwrap()`
    - However, this does underline the positives of the
      `HashMap<id: usize, record: T>` datastructure:

      iterating through the elements, we don't need to unwrap the already guarenteed to
          exist `id`:

          for (id, record) in store.csv_records.iter() {
            if *id == my_matching_id {
              // do something with record
              // ***if we handle data correctly***
              // record.id.unwrap() should never panic here
            }
          }


