# account-tx-engine
Example cli application to process payments transactions

## Build

```
cargo build
```

## Run

```
cargo run -- <input_file.csv> > <output_file.csv>
```

## Test
```
cargo test
```

## NOTES

- Only deposit and withdrawals can be disputed ( is weird to disout a deposit but could be the case).
- Only `disputed` transactions can be resolved or chagebacked.
- Test are running in CI ( build / fmt / clippy ).


### List of improvements:
    - Add unit test and improve integration tests.
    - Accept a generic iterator, allowing not only read `cvs` files.
    - Add a transaction log to easy replay by client.
