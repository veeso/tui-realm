# Formats the code
[group('code_check')]
fmt args="":
  cargo fmt --all {{args}}


# Formats the code using the nightly version of rustfmt
[group('code_check')]
fmt_nightly args="":
  cargo +nightly fmt --all {{args}}


# Runs clippy with all features and all targets (matches CI)
[group('code_check')]
clippy args="":
  cargo clippy --workspace --all-targets --all-features {{args}}


# Runs clippy with default features only (matches CI)
[group('code_check')]
clippy_default args="":
  cargo clippy --workspace --all-targets {{args}}


# Runs fmt and clippy on the code. Fails if any of the checks fail
[group('code_check')]
check_code:
  just fmt_nightly "--check"
  just clippy "-- -D warnings"
  just clippy_default "-- -D warnings"
