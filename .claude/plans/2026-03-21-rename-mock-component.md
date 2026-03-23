# Rename MockComponent Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Rename `MockComponent` → `Component` and `Component` → `AppComponent` across the entire tui-realm workspace.

**Architecture:** Two-pass rename — first rename `Component` → `AppComponent` everywhere, then rename `MockComponent` → `Component` everywhere. This avoids name collisions. The derive macro `#[derive(MockComponent)]` becomes `#[derive(Component)]`.

**Tech Stack:** Rust, proc-macro (syn/quote), ratatui

**Spec:** `docs/superpowers/specs/2026-03-21-rename-mock-component-design.md`

---

### Task 1: Rename `Component` → `AppComponent` in core trait definition

**Files:**
- Modify: `crates/tuirealm/src/core/component.rs`

- [ ] **Step 1: Rename the trait and update doc comments**

In `crates/tuirealm/src/core/component.rs`, make these changes:

1. Rename `trait Component<Msg, UserEvent>: MockComponent + Any` to `trait AppComponent<Msg, UserEvent>: MockComponent + Any`
2. Update the doc comment above it (lines 46-56) to reference `AppComponent` instead of `Component`
3. Update the `dyn Component<Msg, UserEvent>` impl block (line 68) to `dyn AppComponent<Msg, UserEvent>`

- [ ] **Step 2: Update re-exports**

In `crates/tuirealm/src/core/mod.rs` (line 14):
```rust
pub use component::{AppComponent, MockComponent};
```

In `crates/tuirealm/src/lib.rs` (line 94):
```rust
pub use self::core::{AppComponent, MockComponent, State, StateValue, ViewError, command};
```

- [ ] **Step 3: Update `view.rs` imports and type alias**

In `crates/tuirealm/src/core/view.rs`:
- Line 10: change `Component` to `AppComponent` in the import
- Line 13: `type WrappedComponent<Msg, UserEvent> = Box<dyn AppComponent<Msg, UserEvent>>;`
- Lines 135, 140-143: update `get_component` / `get_component_mut` return types to `&dyn AppComponent` / `&mut dyn AppComponent`

- [ ] **Step 4: Update `application.rs` imports**

In `crates/tuirealm/src/core/application.rs`:
- Line 13: change `Component` to `AppComponent` in the import
- Update any method signatures that return `&dyn Component` / `&mut dyn Component` to use `AppComponent`

- [ ] **Step 5: Update `command.rs` doc comments**

In `crates/tuirealm/src/core/command.rs`:
- Line 1-2: update doc comment references from `Component` to `AppComponent` where it refers to the event-handling trait
- Line 9-12: same treatment

- [ ] **Step 6: Verify it compiles**

Run: `cargo build -p tuirealm --all-features`
Expected: Should fail because `mock/`, examples, and tests still reference old name — that's OK, we'll fix those next.

- [ ] **Step 7: Commit**

```bash
git add crates/tuirealm/src/core/ crates/tuirealm/src/lib.rs
git commit -m "refactor(tuirealm): rename Component trait to AppComponent"
```

---

### Task 2: Update core crate internals for `Component` → `AppComponent`

**Files:**
- Modify: `crates/tuirealm/src/mock/components.rs`
- Modify: `crates/tuirealm/src/core/view.rs` (test module)
- Modify: `crates/tuirealm/src/core/subscription.rs` (test module)

- [ ] **Step 1: Update mock components**

In `crates/tuirealm/src/mock/components.rs`:
- Line 8: change `Component` to `AppComponent` in the import
- Lines 98-126: change `impl Component<MockMsg, MockEvent> for MockFooInput` to `impl AppComponent<MockMsg, MockEvent> for MockFooInput`
- Lines 133-162: change `impl Component<MockMsg, MockEvent> for MockBarInput` to `impl AppComponent<MockMsg, MockEvent> for MockBarInput`

- [ ] **Step 2: Update view.rs test module**

In `crates/tuirealm/src/core/view.rs`, find the test module (around line 780). Update:
- Any `impl Component<..>` to `impl AppComponent<..>`
- Any imports of `Component` to `AppComponent`

- [ ] **Step 3: Verify core crate compiles and tests pass**

Note: `subscription.rs` tests only import `MockComponent` (not `Component`), so no changes are needed here in Pass 1. It will be handled in Task 4.

Run: `cargo test -p tuirealm --all-features --lib --tests`
Expected: PASS (use `--lib --tests` to skip examples, which haven't been updated yet)

- [ ] **Step 5: Commit**

```bash
git add crates/tuirealm/src/
git commit -m "refactor(tuirealm): update internal code for Component→AppComponent rename"
```

---

### Task 3: Rename `MockComponent` → `Component` in core trait definition

**Files:**
- Modify: `crates/tuirealm/src/core/component.rs`
- Modify: `crates/tuirealm/src/core/mod.rs`
- Modify: `crates/tuirealm/src/lib.rs`

- [ ] **Step 1: Rename the trait and update doc comments**

In `crates/tuirealm/src/core/component.rs`:
1. Rename `trait MockComponent` to `trait Component`
2. Update the doc comment above it (lines 11-24) — remove all "Mock" language, describe it as the base component trait
3. Update `trait AppComponent<Msg, UserEvent>: MockComponent + Any` to `trait AppComponent<Msg, UserEvent>: Component + Any`
4. Update the `AppComponent` doc comment to reference `Component` instead of `MockComponent`

- [ ] **Step 2: Update re-exports**

In `crates/tuirealm/src/core/mod.rs`:
```rust
pub use component::{AppComponent, Component};
```

In `crates/tuirealm/src/lib.rs` (line 94):
```rust
pub use self::core::{AppComponent, Component, State, StateValue, ViewError, command};
```

Also update the crate-level rustdoc in `lib.rs` (line 33) — change `#[derive(MockComponent)]` to `#[derive(Component)]` and update trait name references.

- [ ] **Step 3: Update `command.rs` doc comments**

In `crates/tuirealm/src/core/command.rs`:
- Replace remaining `MockComponent` references with `Component` in doc comments

- [ ] **Step 4: Verify it compiles (expect failures in dependents)**

Run: `cargo build -p tuirealm --all-features`
Expected: May fail in mock/tests — we fix those next.

- [ ] **Step 5: Commit**

```bash
git add crates/tuirealm/src/core/ crates/tuirealm/src/lib.rs
git commit -m "refactor(tuirealm): rename MockComponent trait to Component"
```

---

### Task 4: Update core crate internals for `MockComponent` → `Component`

**Files:**
- Modify: `crates/tuirealm/src/mock/components.rs`
- Modify: `crates/tuirealm/src/core/view.rs` (test module)
- Modify: `crates/tuirealm/src/core/subscription.rs` (test module)

Note: Line numbers may have shifted from earlier tasks. Match by content, not line number.

- [ ] **Step 1: Update mock components**

In `crates/tuirealm/src/mock/components.rs`:
- Update import: `MockComponent` → `Component`
- `impl MockComponent for MockInput` → `impl Component for MockInput`
- `#[derive(MockComponent)]` → `#[derive(Component)]` (on MockFooInput and MockBarInput)

Note: This will fail to compile because the derive macro hasn't been renamed yet. That's expected — we'll fix the derive macro in Task 5.

- [ ] **Step 2: Update view.rs test module**

Update any `#[derive(MockComponent)]` and `impl MockComponent` in the test module to use `Component`.

- [ ] **Step 3: Update subscription.rs test module**

In `crates/tuirealm/src/core/subscription.rs`, the test module imports `MockComponent` as a trait (e.g., `use crate::{MockComponent, ...}`). Update this import to `Component`.

- [ ] **Step 4: Commit (code won't compile yet — derive macro still uses old name)**

```bash
git add crates/tuirealm/src/
git commit -m "refactor(tuirealm): update internal code for MockComponent→Component rename"
```

---

### Task 5: Update derive macro

**Files:**
- Modify: `crates/tuirealm_derive/src/lib.rs`
- Modify: `crates/tuirealm_derive/Cargo.toml`

- [ ] **Step 1: Rename the proc macro**

In `crates/tuirealm_derive/src/lib.rs`:

1. Line 111: change `#[proc_macro_derive(MockComponent, attributes(component))]` to `#[proc_macro_derive(Component, attributes(component))]`
2. Rename the function from `mock_component` to `component` (or keep it — internal name doesn't matter)
3. Line 157: in generated code, change `use ::tuirealm::MockComponent` to `use ::tuirealm::Component`
4. Line 158: change `impl #generics MockComponent for #ident` to `impl #generics Component for #ident`
5. Line 184: change panic message from `"MockComponent must be derived by a Struct"` to `"Component must be derived by a Struct"`

- [ ] **Step 2: Update crate-level doc comments**

Update the entire module doc comment (lines 1-97) to replace all `MockComponent` references with `Component` and all `Component` references (where they mean the event-handling trait) with `AppComponent`.

- [ ] **Step 3: Update Cargo.toml description**

In `crates/tuirealm_derive/Cargo.toml`, update the `description` field from mentioning `MockComponent` to `Component`.

- [ ] **Step 4: Verify core crate compiles and tests pass**

Run: `cargo test -p tuirealm --all-features --lib --tests`
Expected: PASS (use `--lib --tests` to skip examples, which haven't been updated yet)

- [ ] **Step 5: Commit**

```bash
git add crates/tuirealm_derive/
git commit -m "refactor(tuirealm_derive): rename derive macro from MockComponent to Component"
```

---

### Task 6: Update tuirealm examples

**Files:**
- Modify: `crates/tuirealm/examples/arbitrary_data.rs`
- Modify: `crates/tuirealm/examples/demo/components/clock.rs`
- Modify: `crates/tuirealm/examples/demo/components/counter.rs`
- Modify: `crates/tuirealm/examples/demo/components/label.rs`
- Modify: `crates/tuirealm/examples/user_events/components/label.rs`
- Modify: `crates/tuirealm/examples/event_display.rs`
- Modify: `crates/tuirealm/examples/inline_display.rs`
- Modify: `crates/tuirealm/examples/async_ports.rs`

- [ ] **Step 1: Apply renames across all example files**

For each file, apply these mechanical replacements:
1. `MockComponent` (in imports and trait impls) → `Component`
2. `Component<` (in trait impls like `impl Component<Msg, ...>`) → `AppComponent<`
3. `#[derive(MockComponent)]` → `#[derive(Component)]`
4. Update imports: `use tuirealm::{..., MockComponent, Component, ...}` → `use tuirealm::{..., Component, AppComponent, ...}`

Be careful with ordering: rename `Component<` → `AppComponent<` first, then `MockComponent` → `Component`.

- [ ] **Step 2: Verify examples compile**

Run: `cargo build -p tuirealm --all-features --examples`
Expected: PASS

- [ ] **Step 3: Commit**

```bash
git add crates/tuirealm/examples/
git commit -m "refactor(tuirealm): update examples for Component/AppComponent rename"
```

---

### Task 7: Update tui-realm-stdlib

**Files:**
- Modify: All 19 component files in `crates/tui-realm-stdlib/src/components/`
- Modify: `crates/tui-realm-stdlib/src/lib.rs` (if it re-exports traits)
- Modify: All example files in `crates/tui-realm-stdlib/examples/`

- [ ] **Step 1: Update all stdlib component implementations**

For each file in `crates/tui-realm-stdlib/src/components/`:
1. `impl MockComponent for X` → `impl Component for X`
2. Update imports: `MockComponent` → `Component`

- [ ] **Step 2: Update all stdlib examples**

For each file in `crates/tui-realm-stdlib/examples/`:
1. `#[derive(MockComponent)]` → `#[derive(Component)]`
2. `impl Component<Msg, ...>` → `impl AppComponent<Msg, ...>`
3. `impl MockComponent` → `impl Component`
4. Update imports accordingly

- [ ] **Step 3: Verify stdlib compiles and tests pass**

Run: `cargo test -p tui-realm-stdlib --all-features`
Expected: PASS

Run: `cargo build -p tui-realm-stdlib --examples`
Expected: PASS

- [ ] **Step 4: Commit**

```bash
git add crates/tui-realm-stdlib/
git commit -m "refactor(tui-realm-stdlib): update for Component/AppComponent rename"
```

---

### Task 8: Update extra crates

**Files:**
- Modify: `crates/extra/tui-realm-textarea/src/lib.rs`
- Modify: `crates/extra/tui-realm-textarea/examples/editor.rs`
- Modify: `crates/extra/tui-realm-textarea/examples/single_line.rs`
- Modify: `crates/extra/tui-realm-treeview/src/lib.rs`
- Modify: `crates/extra/tui-realm-treeview/examples/demo.rs`

- [ ] **Step 1: Update tui-realm-textarea**

In `crates/extra/tui-realm-textarea/src/lib.rs`:
1. `impl MockComponent for TextArea` → `impl Component for TextArea`
2. Update imports and doc comments

In example files:
1. Same mechanical replacements as Task 6

- [ ] **Step 2: Update tui-realm-treeview**

In `crates/extra/tui-realm-treeview/src/lib.rs`:
1. `impl MockComponent for TreeView` → `impl Component for TreeView`
2. Update imports and doc comments

In example files:
1. Same mechanical replacements as Task 6

- [ ] **Step 3: Verify extra crates compile and tests pass**

Run: `cargo test -p tui-realm-textarea --all-features`
Run: `cargo test -p tui-realm-treeview --all-features`
Expected: PASS

- [ ] **Step 4: Commit**

```bash
git add crates/extra/
git commit -m "refactor(extra): update textarea and treeview for Component/AppComponent rename"
```

---

### Task 9: Update documentation

**Files:**
- Modify: `CLAUDE.md`
- Modify: `README.md`
- Modify: `docs/en/get-started.md`
- Modify: `docs/en/advanced.md`
- Delete: `docs/en/migrating-legacy.md`
- Modify: `docs/zh-cn/get-started.md`
- Modify: `docs/zh-cn/advanced.md`
- Delete: `docs/zh-cn/migrating-legacy.md`
- Modify: `crates/tuirealm_derive/README.md`

- [ ] **Step 1: Delete legacy migration guides**

```bash
git rm docs/en/migrating-legacy.md docs/zh-cn/migrating-legacy.md
```

- [ ] **Step 2: Update CLAUDE.md**

Replace all `MockComponent` references with `Component` and `Component` trait references (where they mean the event-handling trait) with `AppComponent` in the Architecture section and Key Traits section.

- [ ] **Step 3: Update README.md**

Update the feature description that mentions `#[derive(MockComponent)]`.

- [ ] **Step 4: Update English docs**

In `docs/en/get-started.md` and `docs/en/advanced.md`:
- Replace all `MockComponent` with `Component`
- Replace all `Component<Msg, UserEvent>` / `impl Component<...>` patterns with `AppComponent`
- Update `#[derive(MockComponent)]` to `#[derive(Component)]`
- Update explanatory text

- [ ] **Step 5: Update Chinese docs**

Same changes as Step 4 in `docs/zh-cn/get-started.md` and `docs/zh-cn/advanced.md`.

- [ ] **Step 6: Update derive crate README**

In `crates/tuirealm_derive/README.md`:
- Replace all `MockComponent` with `Component`
- Replace `Component` (event-handling trait) with `AppComponent`
- Update code examples

- [ ] **Step 7: Commit**

```bash
git add CLAUDE.md README.md docs/ crates/tuirealm_derive/README.md
git commit -m "docs: update all documentation for Component/AppComponent rename"
```

---

### Task 10: Full workspace verification

- [ ] **Step 1: Build entire workspace**

Run: `cargo build --workspace --all-features`
Expected: PASS

- [ ] **Step 2: Run all tests**

Run: `cargo test --workspace --all-features`
Expected: PASS

- [ ] **Step 3: Run clippy**

Run: `cargo clippy --workspace --all-targets --all-features -- -Dwarnings`
Expected: PASS

- [ ] **Step 4: Run format check**

Run: `cargo +nightly fmt --all -- --check`
Expected: PASS

- [ ] **Step 5: Grep for stale references**

Grep the entire workspace for `MockComponent`. The only remaining occurrences should be in:
- CHANGELOGs (historical records)
- This plan and the design spec

- [ ] **Step 6: Commit any fixups if needed**

If any issues were found in steps 1-5, fix and commit.
