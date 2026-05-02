# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

tui-realm is a monorepo workspace containing the following crates:

| Crate | Path | Description |
|-------|------|-------------|
| `tuirealm` | `crates/tuirealm/` | Core framework — React/Elm-inspired component system for ratatui |
| `tuirealm_derive` | `crates/tuirealm-derive/` | `#[derive(Component)]` proc macro |
| `tui-realm-stdlib` | `crates/tuirealm-stdlib/` | Standard components library (input, list, table, etc.) |
| `tui-realm-textarea` | `crates/tuirealm-textarea/` | Textarea component (based on tui-textarea-2) |
| `tui-realm-treeview` | `crates/tuirealm-treeview/` | Treeview component (based on orange-trees) |

## Commands

**IMPORTANT:** Always use the `just` recipes for build, lint, format, and test checks. Do not invoke `cargo build`, `cargo test`, `cargo clippy`, or `cargo fmt` directly — the recipes encode the exact flags used by CI, and consistency between local and CI is required.

```bash
# List all recipes
just

# Build
just build_all                # workspace, all features
just build_examples           # all examples
just build <crate>            # single crate, all features

# Test
just test_all                 # workspace, all features
just test <crate> [name]      # single crate
just coverage                 # lcov coverage (Linux/macOS)
just coverage_no_termion      # lcov coverage (Windows; no termion)

# Lint / format
just fmt_nightly              # nightly rustfmt (apply)
just fmt_nightly "--check"    # nightly rustfmt (check)
just clippy "-- -Dwarnings"            # clippy, all features + targets
just clippy_default "-- -Dwarnings"    # clippy, default features
just check_code               # fmt_nightly --check + clippy (both feature sets)

# Publish (in dependency order, with retry on registry lag)
just publish_all
```

For one-off cases not covered by a recipe (e.g. running a specific example), `cargo` is acceptable:

```bash
cargo run -p tuirealm --example demo --features crossterm
cargo run -p tui-realm-stdlib --example input
```

MSRV: 1.88. Edition: 2024.

## Feature Flags (tuirealm core)

- `derive` (default) — `#[derive(Component)]` proc macro
- `crossterm` (default) — crossterm terminal backend
- `async-ports` — async event ports via tokio
- `serialize` — serde support for key events
- `termion` — termion backend (Unix-only)
- `termwiz` — termwiz backend

## Architecture

### Core Loop (Elm-style)

```
Application::tick(PollStrategy)
  → polls EventListener for events
  → forwards events to focused component (AppComponent::on)
  → forwards to subscribed components (based on EventClause + SubClause)
  → returns Vec<Msg>

update(msg) → user processes messages, mutates model

view() → Terminal::draw → each mounted component renders via Component::view
```

### Key Traits

- **`Component`** — rendering + state + properties + command execution. Methods: `view()`, `query()`, `attr()`, `state()`, `perform(Cmd) -> CmdResult`.
- **`AppComponent<Msg, UserEvent>`** — extends Component with `on(&Event<UserEvent>) -> Option<Msg>` for event-to-message mapping.
- **`Poll<UserEvent>`** — synchronous event source port.
- **`PollAsync<UserEvent>`** — async event source port (requires `async-ports` feature).

### Key Types

- **`Application<Id, Msg, UserEvent>`** — main entry point; owns `View` + `EventListener`. `Id` must be `Eq + PartialEq + Clone + Hash`.
- **`View`** — component container managing mount/unmount, focus stack, property injection.
- **`EventListener`** — background thread polling ports for events.
- **`Props`** / **`Attribute`** / **`AttrValue`** — property system with 40+ predefined attributes.
- **`State`** / **`StateValue`** — component state (Single, Pair, Vec, Map, Linked, Any, None).
- **`Cmd`** / **`CmdResult`** — commands sent to Component::perform and their results.
- **`Sub`** / **`EventClause`** / **`SubClause`** — subscription system for non-focused event routing.

### Module Layout

- `crates/tuirealm/src/core/` — Application, View, Component/AppComponent traits, Props, State, Subscriptions, Events, Commands
- `crates/tuirealm/src/listener/` — EventListener, ports (sync/async), worker thread, builder
- `crates/tuirealm/src/terminal.rs` — Terminal adapters and backend-specific input listeners
- `crates/tuirealm/src/mock/` — Mock types for testing (test-only)
- `crates/tuirealm/src/utils/` — Utility types (Email, PhoneNumber, etc.)
- `crates/tuirealm/src/macros.rs` — `subclause_and!`, `subclause_or!`, `subclause_and_not!` helper macros

## Code Style

- rustfmt: `group_imports = "StdExternalCrate"`, `imports_granularity = "Module"`
