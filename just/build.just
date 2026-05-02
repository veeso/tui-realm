# Cleans the build artifacts
[group('build')]
[confirm("Are you sure you want to clean the build artifacts?")]
clean:
  cargo clean


# Builds the entire workspace with all features
[group('build')]
build_all:
  cargo build --workspace --all-features


# Builds all examples
[group('build')]
build_examples:
  cargo build --all-targets --examples


# Builds a single crate
[group('build')]
build crate:
  cargo build -p {{crate}} --all-features
