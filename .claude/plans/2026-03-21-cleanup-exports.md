# Cleanup Exports (#168) Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Remove root-level re-exports from all crates; expose types only through their defining modules.

**Architecture:** Make `core`'s submodules (currently private) public and re-export them as top-level modules in `tuirealm` lib.rs. Remove all `pub use` of individual types from lib.rs. For stdlib, make `components` module public. For extra crates, make internal modules public. Update all downstream imports.

**Tech Stack:** Rust, cargo workspace

---

## Current State Summary

**tuirealm core** (lib.rs): `mod core;` is private. Types like `Application`, `Event`, `Props`, etc. are re-exported at crate root via `pub use self::core::...`. Some modules (`application`, `event`, `props`, `command`) are also re-exported as modules. Private submodules `component`, `state`, `view` only have types re-exported at root.

**stdlib** (lib.rs): `mod components;` is private. Everything re-exported at root via `pub use components::{props, *};`.

**Extra crates**: Flat exports in lib.rs.

## Target Module Paths

### tuirealm

| Type | Old path | New path |
|------|----------|----------|
| `Application` | `tuirealm::Application` | `tuirealm::application::Application` |
| `ApplicationError` | `tuirealm::ApplicationError` | `tuirealm::application::ApplicationError` |
| `PollStrategy` | `tuirealm::PollStrategy` | `tuirealm::application::PollStrategy` |
| `Event` | `tuirealm::Event` | `tuirealm::event::Event` |
| `NoUserEvent` | `tuirealm::NoUserEvent` | `tuirealm::event::NoUserEvent` |
| `AttrValue` | `tuirealm::AttrValue` | `tuirealm::props::AttrValue` |
| `Attribute` | `tuirealm::Attribute` | `tuirealm::props::Attribute` |
| `Props` | `tuirealm::Props` | `tuirealm::props::Props` |
| `Sub` | `tuirealm::Sub` | `tuirealm::subscription::Sub` |
| `SubClause` | `tuirealm::SubClause` | `tuirealm::subscription::SubClause` |
| `SubEventClause` | `tuirealm::SubEventClause` | `tuirealm::subscription::EventClause` |
| `Component` | `tuirealm::Component` | `tuirealm::component::Component` |
| `MockComponent` | `tuirealm::MockComponent` | `tuirealm::component::MockComponent` |
| `State` | `tuirealm::State` | `tuirealm::state::State` |
| `StateValue` | `tuirealm::StateValue` | `tuirealm::state::StateValue` |
| `ViewError` | `tuirealm::ViewError` | `tuirealm::view::ViewError` |
| `View` | `tuirealm::core::View` | `tuirealm::view::View` |
| `Injector` | `tuirealm::Injector` | `tuirealm::injector::Injector` |
| `Frame` | `tuirealm::Frame` | `tuirealm::ratatui::Frame` |
| `EventListenerCfg` | `tuirealm::EventListenerCfg` | `tuirealm::listener::EventListenerCfg` |
| `ListenerError` | `tuirealm::ListenerError` | `tuirealm::listener::ListenerError` |

**Kept at root** (special cases):
- `#[cfg(feature = "async-ports")] pub use async_trait::async_trait;` — needed by users implementing `PollAsync`
- `#[cfg(feature = "derive")] pub use tuirealm_derive::*;` — proc macro must be at root scope

### tui-realm-stdlib

| Type | Old path | New path |
|------|----------|----------|
| `Input` | `tui_realm_stdlib::Input` | `tui_realm_stdlib::components::Input` |
| `Label` | `tui_realm_stdlib::Label` | `tui_realm_stdlib::components::Label` |
| (all components) | `tui_realm_stdlib::<Name>` | `tui_realm_stdlib::components::<Name>` |
| `INPUT_PLACEHOLDER` etc. | `tui_realm_stdlib::props::*` | `tui_realm_stdlib::components::props::*` |
| `InputStates` etc. | `tui_realm_stdlib::InputStates` | `tui_realm_stdlib::components::states::InputStates` |

`prop_ext` and `utils` modules remain unchanged (already module-level).

### tui-realm-treeview

| Type | Old path | New path |
|------|----------|----------|
| `TreeState` | `tui_realm_treeview::TreeState` | `tui_realm_treeview::tree_state::TreeState` |
| `TreeWidget` | `tui_realm_treeview::TreeWidget` | `tui_realm_treeview::widget::TreeWidget` |
| `Node`, `Tree` type aliases | `tui_realm_treeview::Node` | remain at root (type aliases, not re-exports) |
| `TreeView` struct | `tui_realm_treeview::TreeView` | remain at root (defined in lib.rs) |
| `NodeValue` trait | `tui_realm_treeview::NodeValue` | remain at root (defined in lib.rs) |
| Constants | `tui_realm_treeview::TREE_*` | remain at root (defined in lib.rs) |

### tui-realm-textarea

No structural changes — everything is defined directly in lib.rs (not re-exported from submodules). Only update tuirealm imports.

---

## Task 1: Update tuirealm core module structure

**Files:**
- Modify: `crates/tuirealm/src/lib.rs`
- Modify: `crates/tuirealm/src/core/mod.rs`

- [ ] **Step 1: Make private core submodules public**

In `crates/tuirealm/src/core/mod.rs`, change:

```rust
pub mod application;
pub mod command;
pub mod component;  // was: mod component;
pub mod event;
pub mod injector;
pub mod props;
pub mod state;      // was: mod state;
pub mod subscription;
pub mod view;       // was: mod view;

// -- internal
pub(crate) use subscription::Subscription;
pub(crate) use view::WrappedComponent;
```

Remove the `pub use` re-exports for `Component`, `MockComponent`, `State`, `StateValue`, `View`, `ViewError` (they'll be accessible through their modules).

- [ ] **Step 2: Rewrite lib.rs exports**

Replace all root-level `pub use` with module-only re-exports:

```rust
mod core;
mod macros;

pub mod listener;
pub mod ratatui;
pub mod terminal;
pub mod utils;

#[cfg(test)]
pub mod mock;

// Re-export core submodules as top-level modules
pub use self::core::{
    application, command, component, event, injector, props, state, subscription, view,
};

// Feature re-exports (must stay at root)
#[cfg(feature = "async-ports")]
#[cfg_attr(docsrs, doc(cfg(feature = "async-ports")))]
pub use async_trait::async_trait;
#[cfg(feature = "derive")]
#[doc(hidden)]
pub use tuirealm_derive::*;
```

- [ ] **Step 3: Build tuirealm crate to verify module structure compiles**

Run: `cargo build -p tuirealm --all-features`
Expected: Compilation errors from internal code using old paths (we'll fix those next).

- [ ] **Step 4: Fix internal imports within tuirealm**

Update all `use crate::` imports within tuirealm that relied on root-level re-exports. Key files:

- `crates/tuirealm/src/macros.rs` — uses `crate::{Sub, SubClause, SubEventClause}`, change to `crate::subscription::{Sub, SubClause, EventClause}`
- `crates/tuirealm/src/core/application/mod.rs` — may use `crate::` paths
- `crates/tuirealm/src/core/view.rs` — may use `crate::` paths
- `crates/tuirealm/src/listener/` — may reference `crate::Event`, `crate::ListenerError`
- `crates/tuirealm/src/terminal.rs` — may reference `crate::` paths
- Any test modules

Approach: run `cargo build -p tuirealm --all-features` iteratively, fix each error.

- [ ] **Step 5: Verify tuirealm crate compiles and tests pass**

Run: `cargo build -p tuirealm --all-features && cargo test -p tuirealm --all-features`
Expected: PASS

- [ ] **Step 6: Commit**

```bash
git add crates/tuirealm/src/
git commit -m "refactor(tuirealm): remove root-level re-exports, expose modules only (#168)"
```

---

## Task 2: Update tuirealm examples

**Files:**
- Modify: all files in `crates/tuirealm/examples/`

- [ ] **Step 1: Update demo example imports**

Files: `crates/tuirealm/examples/demo/demo.rs`, `demo/app/model.rs`, `demo/components/mod.rs`, `demo/components/clock.rs`, `demo/components/counter.rs`, `demo/components/label.rs`

Replace root-level imports with module-level:
- `tuirealm::Application` → `tuirealm::application::Application`
- `tuirealm::{Component, MockComponent}` → `tuirealm::component::{Component, MockComponent}`
- `tuirealm::{State, StateValue}` → `tuirealm::state::{State, StateValue}`
- `tuirealm::{Event, NoUserEvent}` → `tuirealm::event::{Event, NoUserEvent}`
- `tuirealm::{AttrValue, Attribute}` → `tuirealm::props::{AttrValue, Attribute}`
- `tuirealm::{Sub, SubClause, SubEventClause}` → `tuirealm::subscription::{Sub, SubClause, EventClause}`
- `tuirealm::Frame` → `tuirealm::ratatui::Frame`
- `tuirealm::EventListenerCfg` → `tuirealm::listener::EventListenerCfg`

- [ ] **Step 2: Update user_events example imports**

Files: `crates/tuirealm/examples/user_events/user_events.rs`, `user_events/model.rs`, `user_events/components/label.rs`

Same pattern as step 1.

- [ ] **Step 3: Update standalone example imports**

Files: `crates/tuirealm/examples/arbitrary_data.rs`, `async_ports.rs`, `event_display.rs`, `inline_display.rs`

Same pattern.

- [ ] **Step 4: Verify examples compile**

Run: `cargo build -p tuirealm --all-features --examples`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add crates/tuirealm/examples/
git commit -m "refactor(tuirealm): update example imports to module-level paths (#168)"
```

---

## Task 3: Update tui-realm-stdlib

**Files:**
- Modify: `crates/tui-realm-stdlib/src/lib.rs`
- Modify: `crates/tui-realm-stdlib/src/prop_ext.rs`
- Modify: all files in `crates/tui-realm-stdlib/src/components/`

- [ ] **Step 1: Update stdlib lib.rs exports**

Change `crates/tui-realm-stdlib/src/lib.rs`:

```rust
pub mod components;
pub mod prop_ext;
pub mod utils;
```

Remove `pub use components::{props, *};`.

- [ ] **Step 2: Fix internal tuirealm imports in stdlib source**

Update all `use tuirealm::` imports in `crates/tui-realm-stdlib/src/` to use module-level paths:
- `tuirealm::{Frame, MockComponent, State, StateValue}` → split into `tuirealm::ratatui::Frame`, `tuirealm::component::MockComponent`, `tuirealm::state::{State, StateValue}`
- `tuirealm::{AttrValue, Attribute}` → `tuirealm::props::{AttrValue, Attribute}`
- etc.

Files affected: `prop_ext.rs`, `utils.rs`, and all component files in `components/`.

- [ ] **Step 3: Verify stdlib compiles and tests pass**

Run: `cargo build -p tui-realm-stdlib --all-features && cargo test -p tui-realm-stdlib --all-features`
Expected: PASS

- [ ] **Step 4: Commit**

```bash
git add crates/tui-realm-stdlib/src/
git commit -m "refactor(stdlib): make components module public, remove root re-exports (#168)"
```

---

## Task 4: Update tui-realm-stdlib examples

**Files:**
- Modify: all files in `crates/tui-realm-stdlib/examples/`

- [ ] **Step 1: Update all stdlib example imports**

For each example in `crates/tui-realm-stdlib/examples/`:
- `use tui_realm_stdlib::Input;` → `use tui_realm_stdlib::components::Input;`
- (same for all component imports)
- Update all `use tuirealm::` imports to module-level paths (same as Task 2)

- [ ] **Step 2: Verify stdlib examples compile**

Run: `cargo build -p tui-realm-stdlib --all-features --examples`
Expected: PASS

- [ ] **Step 3: Commit**

```bash
git add crates/tui-realm-stdlib/examples/
git commit -m "refactor(stdlib): update example imports to module-level paths (#168)"
```

---

## Task 5: Update tui-realm-treeview

**Files:**
- Modify: `crates/extra/tui-realm-treeview/src/lib.rs`
- Modify: `crates/extra/tui-realm-treeview/src/widget.rs`

- [ ] **Step 1: Make treeview internal modules public and remove re-exports**

In `crates/extra/tui-realm-treeview/src/lib.rs`:
- Change `mod tree_state;` → `pub mod tree_state;`
- Change `mod widget;` → `pub mod widget;`
- Remove `pub use tree_state::TreeState;` and `pub use widget::TreeWidget;`

- [ ] **Step 2: Fix tuirealm imports in treeview source**

Update all `use tuirealm::` imports to module-level paths.

- [ ] **Step 3: Update treeview example imports**

In `crates/extra/tui-realm-treeview/examples/demo.rs`:
- `use tui_realm_treeview::{..., TreeView}` → update `TreeState` and `TreeWidget` to module paths
- Update all `use tuirealm::` imports
- Update `use tui_realm_stdlib::` imports to `use tui_realm_stdlib::components::`

- [ ] **Step 4: Verify treeview compiles and tests pass**

Run: `cargo build -p tui-realm-treeview --all-features && cargo test -p tui-realm-treeview --all-features`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add crates/extra/tui-realm-treeview/
git commit -m "refactor(treeview): make modules public, remove root re-exports, update imports (#168)"
```

---

## Task 6: Update tui-realm-textarea

**Files:**
- Modify: `crates/extra/tui-realm-textarea/src/lib.rs`
- Modify: `crates/extra/tui-realm-textarea/src/fmt.rs`
- Modify: `crates/extra/tui-realm-textarea/examples/*.rs`

- [ ] **Step 1: Fix tuirealm imports in textarea source**

Update all `use tuirealm::` imports in textarea source to module-level paths.

- [ ] **Step 2: Update textarea example imports**

Update all `use tuirealm::` and `use tui_realm_stdlib::` imports in examples.

- [ ] **Step 3: Verify textarea compiles and tests pass**

Run: `cargo build -p tui-realm-textarea --all-features && cargo test -p tui-realm-textarea --all-features`
Expected: PASS

- [ ] **Step 4: Commit**

```bash
git add crates/extra/tui-realm-textarea/
git commit -m "refactor(textarea): update imports to module-level paths (#168)"
```

---

## Task 7: Full workspace verification

- [ ] **Step 1: Build entire workspace**

Run: `cargo build --workspace --all-features`
Expected: PASS

- [ ] **Step 2: Run all tests**

Run: `cargo test --workspace --all-features`
Expected: PASS

- [ ] **Step 3: Run clippy**

Run: `cargo clippy --workspace --all-targets --all-features -- -Dwarnings`
Expected: PASS

- [ ] **Step 4: Run fmt check**

Run: `cargo +nightly fmt --all -- --check`
Expected: PASS

---

## Task 8: Update migration guide

**Files:**
- Modify: `docs/en/migrating-4.0.md`

- [ ] **Step 1: Add export cleanup section to migration guide**

Add a new section to `docs/en/migrating-4.0.md` documenting:
- Root-level re-exports have been removed
- All types must be imported from their defining module
- `SubEventClause` alias removed, use `EventClause` from `subscription` module
- `Frame` is no longer re-exported at root, use `tuirealm::ratatui::Frame`
- Include a migration table showing old → new import paths
- For stdlib: components now imported from `tui_realm_stdlib::components::`
- For treeview: `TreeState` from `tree_state::`, `TreeWidget` from `widget::`

- [ ] **Step 2: Commit**

```bash
git add docs/en/migrating-4.0.md
git commit -m "docs: add export cleanup migration notes for 4.0 (#168)"
```

---

## Task 9: Update other documentation

**Files:**
- Modify: `docs/en/get-started.md`
- Modify: `docs/en/advanced.md`

- [ ] **Step 1: Update get-started.md imports**

Update all `use tuirealm::` and `use tui_realm_stdlib::` examples to use module-level paths.

- [ ] **Step 2: Update advanced.md imports**

Update all `use tuirealm::` examples to use module-level paths.

- [ ] **Step 3: Verify no other docs reference old import paths**

Run: `grep -r 'use tuirealm::' docs/` and check for any remaining old-style imports (except in migrating-legacy.md which documents historical API).

- [ ] **Step 4: Commit**

```bash
git add docs/
git commit -m "docs: update import examples to module-level paths (#168)"
```
