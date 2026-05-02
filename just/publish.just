# Publish all crates in dependency order
[group('publish')]
publish_all: publish_tuirealm_derive publish_tuirealm publish_tuirealm_stdlib publish_tuirealm_textarea publish_tuirealm_treeview

[group('publish')]
publish_tuirealm_derive: (_publish "tuirealm_derive")

[group('publish')]
publish_tuirealm: (_publish "tuirealm")

[group('publish')]
publish_tuirealm_stdlib: (_publish "tui-realm-stdlib")

[group('publish')]
publish_tuirealm_textarea: (_publish "tui-realm-textarea")

[group('publish')]
publish_tuirealm_treeview: (_publish "tui-realm-treeview")

# Publish a crate with retry logic to handle registry lag
[private]
_publish crate:
  #!/usr/bin/env bash
  set -euo pipefail
  max_retries=5
  delay=15
  for attempt in $(seq 1 $max_retries); do
    echo "Publishing {{crate}} (attempt $attempt/$max_retries)..."
    if cargo publish -p {{crate}}; then
      echo "Successfully published {{crate}}"
      exit 0
    fi
    if [ "$attempt" -lt "$max_retries" ]; then
      echo "Publish failed, retrying in ${delay}s..."
      sleep "$delay"
    fi
  done
  echo "Failed to publish {{crate}} after $max_retries attempts"
  exit 1
