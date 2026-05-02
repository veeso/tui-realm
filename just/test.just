# Run all tests across the workspace with all features
[group('test')]
test_all test_name="":
  cargo test --workspace --all-features {{test_name}}


# Run tests for a single crate
[group('test')]
test crate test_name="":
  cargo test -p {{crate}} --all-features {{test_name}}


# Run tests with code coverage (lcov output)
[group('test')]
coverage:
  cargo llvm-cov --no-fail-fast --all-features --workspace --lcov --output-path lcov.info --no-cfg-coverage


# Run tests with reduced feature set (Windows-compatible — no termion)
[group('test')]
coverage_no_termion:
  cargo llvm-cov --no-fail-fast --features async-ports,serialize --workspace --lcov --output-path lcov.info --no-cfg-coverage
