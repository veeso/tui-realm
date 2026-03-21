# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

tui-realm (`tuirealm` crate) is a framework for [ratatui](https://github.com/ratatui/ratatui) that provides a React/Elm-inspired component system for building terminal UIs with properties, state management, messages, and event-driven architecture.

## Commands

```bash
# Build (default features: derive + crossterm)
cargo build

# Build with all features
cargo build --all-features

# Run all tests
cargo test --all-features

# Run a single test
cargo test --all-features <test_name>

# Lint
cargo clippy --all-targets --all-features -- -Dwarnings
cargo clippy --all-targets -- -Dwarnings

# Format (always use nightly)
cargo +nightly fmt --all

# Format check (always use nightly)
cargo +nightly fmt --all -- --check

# Run example
cargo run --example demo --features crossterm
```

MSRV: 1.86. Edition: 2024.

## Feature Flags

- `derive` (default) — `#[derive(MockComponent)]` proc macro
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
  → forwards events to focused component (Component::on)
  → forwards to subscribed components (based on EventClause + SubClause)
  → returns Vec<Msg>

update(msg) → user processes messages, mutates model

view() → Terminal::draw → each mounted component renders via MockComponent::view
```

### Key Traits

- **`MockComponent`** — rendering + state + properties + command execution. Methods: `view()`, `query()`, `attr()`, `state()`, `perform(Cmd) -> CmdResult`.
- **`Component<Msg, UserEvent>`** — extends MockComponent with `on(&Event<UserEvent>) -> Option<Msg>` for event-to-message mapping.
- **`Poll<UserEvent>`** — synchronous event source port.
- **`PollAsync<UserEvent>`** — async event source port (requires `async-ports` feature).

### Key Types

- **`Application<Id, Msg, UserEvent>`** — main entry point; owns `View` + `EventListener`. `Id` must be `Eq + PartialEq + Clone + Hash`.
- **`View`** — component container managing mount/unmount, focus stack, property injection.
- **`EventListener`** — background thread polling ports for events.
- **`Props`** / **`Attribute`** / **`AttrValue`** — property system with 40+ predefined attributes.
- **`State`** / **`StateValue`** — component state (Single, Pair, Vec, Map, Linked, Any, None).
- **`Cmd`** / **`CmdResult`** — commands sent to MockComponent::perform and their results.
- **`Sub`** / **`EventClause`** / **`SubClause`** — subscription system for non-focused event routing.

### Module Layout

- `src/core/` — Application, View, Component/MockComponent traits, Props, State, Subscriptions, Events, Commands
- `src/listener/` — EventListener, ports (sync/async), worker thread, builder
- `src/terminal.rs` — Terminal adapters and backend-specific input listeners
- `src/mock/` — Mock types for testing (test-only)
- `src/utils/` — Utility types (Email, PhoneNumber, etc.)
- `src/macros.rs` — `subclause_and!`, `subclause_or!`, `subclause_and_not!` helper macros

## Code Style

- rustfmt: `group_imports = "StdExternalCrate"`, `imports_granularity = "Module"`
