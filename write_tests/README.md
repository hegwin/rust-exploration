# Write and run tests

Execute `$ cargo test` to run tests.

Function blocks after `#[test]` will be regarded as testing codes.

Codes after `#[cfg(test)]` will not be compiled.

Tests after `#[should_panic]` will pass if the assertion fails.
