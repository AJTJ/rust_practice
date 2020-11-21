https://doc.rust-lang.org/book/ch11-03-test-organization.html

## files

- `adder`

# Tests

`measured` is for benchmarks
`doc-tests` is for documentation tests

## Test Options

`cargo test -- --test-threads=1`

- Set the number of test threads
  - Setting it to 1 tells it to use no parallelism
- `--show-output`
  - this will show the printed values for passing tests
  - normally we will only see the value of failed tests
- `cargo test NAMEOFTEST`
  - this will ONLY run the named test
- `cargo test PARTOFWORD`
  - if multiple tests contain part of that word they will all be run
  - thus creating the option to run similar tests
  - this can also filter by the module containing tests
- `cargo test -- --ignored`
  - This will run only the `#[ignore]` tests.

# Integration tests

- build a `tests` directory beside the `src` directory
- not necessary to annotate with `#[cfg(test)]` since they are separate files and are compiled separately

# Notes

- if we call `println!` in a test and the test passes, we won't see `println!` output in the terminal, we only see the line that indicates the test passed.
