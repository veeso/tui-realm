# Design: Rename MockComponent (Issue #161)

## Summary

Rename the two core component traits to eliminate the confusing `Mock` prefix:

| Current Name | New Name | Rationale |
|---|---|---|
| `MockComponent` | `Component` | It *is* the component — rendering, state, props, commands |
| `Component<Msg, UserEvent>` | `AppComponent<Msg, UserEvent>` | A full mountable component that adds event-to-message mapping |

The derive macro changes from `#[derive(MockComponent)]` to `#[derive(Component)]`.

## Motivation

`MockComponent` is the foundational trait in tui-realm, yet the `Mock` prefix suggests it's a test double. In reality it's the base component abstraction. The current `Component` trait extends it with event handling and is what gets mounted into an `Application`. Renaming clarifies the hierarchy:

- `Component` — reusable building block (view, query, attr, state, perform)
- `AppComponent` — application-level component that wraps a `Component` and handles events

## Rename Ordering

Because the current `Component` name is being reused for the renamed `MockComponent`, the rename must be done in the correct order to avoid collisions:

1. **First**: rename current `Component` → `AppComponent` everywhere
2. **Second**: rename `MockComponent` → `Component` everywhere

Alternatively, use a temporary placeholder (e.g., `MockComponent` → `__Component`) then do the final rename. Either way, the implementation must not create a state where two traits share the same name.

## Scope of Changes

### 1. Core trait definitions (`crates/tuirealm/src/core/component.rs`)

- Rename `trait MockComponent` to `trait Component`
- Rename `trait Component<Msg, UserEvent>: MockComponent + Any` to `trait AppComponent<Msg, UserEvent>: Component + Any`
- Update doc comments to reflect new names
- Update the `dyn Component` impl block to `dyn AppComponent`

### 2. Re-exports (`crates/tuirealm/src/core/mod.rs`, `crates/tuirealm/src/lib.rs`)

- Change all `MockComponent` re-exports to `Component`
- Change all `Component` re-exports to `AppComponent`
- Update crate-level rustdoc in `lib.rs` (line 33 references `#[derive(MockComponent)]`)

### 3. Derive macro (`crates/tuirealm_derive/src/lib.rs`)

- Rename `#[proc_macro_derive(MockComponent, ...)]` to `#[proc_macro_derive(Component, ...)]`
- Update generated code: `impl Component for ...` instead of `impl MockComponent for ...`
- Update `use ::tuirealm::MockComponent` to `use ::tuirealm::Component` in the generated `use` block
- Update crate-level doc comments
- Update panic messages (e.g., `"MockComponent must be derived by a Struct"`)
- Update `crates/tuirealm_derive/Cargo.toml` description field

### 4. Core crate internal usage (`crates/tuirealm/src/`)

All internal references must be updated. Key clarification: references to the *current* `Component` trait (the one with `on()`) become `AppComponent`; references to `MockComponent` become `Component`.

- `core/command.rs` — doc comments reference `MockComponent` and `Component` (lines 1, 10, 63)
- `core/application.rs` — imports current `Component` (→ `AppComponent`), generic bounds, method signatures
- `core/subscription.rs` — subscription handling
- `core/view.rs` — the `WrappedComponent` type alias (`Box<dyn Component<Msg, UserEvent>>` → `Box<dyn AppComponent<Msg, UserEvent>>`), `get_component`/`get_component_mut` return types, all method signatures
- `terminal.rs` — terminal adapter trait bounds
- `mock/` — test fixtures: the *type names* (`MockInput`, `MockFooInput`, etc.) stay, but their *trait impls* change: `#[derive(MockComponent)]` → `#[derive(Component)]`, `impl Component<..>` → `impl AppComponent<..>`, `impl MockComponent` → `impl Component`
- Inline test code (e.g., `view.rs` test module creates structs with `#[derive(MockComponent)]` and `impl Component<..>`)
- All example files under `crates/tuirealm/examples/`

### 5. Standard library (`crates/tui-realm-stdlib/`)

- All stdlib components implement `MockComponent` directly — update to `Component`
- Example files reference both traits — update accordingly

### 6. Extra crates (`crates/extra/tui-realm-textarea/`, `crates/extra/tui-realm-treeview/`)

- Same pattern: `impl MockComponent` becomes `impl Component`, `impl Component<..>` becomes `impl AppComponent<..>`

### 7. Documentation

- Update `CLAUDE.md` to reflect new trait names
- Update docs under `docs/en/` that reference the old names
- Update docs under `docs/zh-cn/` that reference the old names
- Update root `README.md`
- Update `crates/tuirealm_derive/README.md`
- CHANGELOGs are historical records and are left as-is

## Non-Changes

- **Test mock type names stay**: `MockInput`, `MockFooInput`, `MockBarInput`, `MockEvent`, `MockMsg`, etc. in `crates/tuirealm/src/mock/` are genuine test fixtures — the `Mock` prefix is correct for them. (Their trait implementations *do* change per section 4.)
- **Feature flag name**: `derive` feature stays the same.
- **`#[component = "field"]` attribute**: The attribute name on the derive macro stays `component` since it refers to the inner component field.

## Verification Strategy

After all changes:

1. `cargo build --workspace --all-features` must succeed
2. `cargo test --workspace --all-features` must pass
3. `cargo clippy --workspace --all-targets --all-features -- -Dwarnings` must pass
4. Grep for `MockComponent` across the workspace — zero occurrences expected outside of CHANGELOGs and this spec

## Migration Impact

This is a breaking change (part of 4.0). All downstream users will need to:

1. Replace `MockComponent` with `Component` in trait implementations
2. Replace `Component<Msg, UserEvent>` with `AppComponent<Msg, UserEvent>`
3. Replace `#[derive(MockComponent)]` with `#[derive(Component)]`

These are mechanical find-and-replace changes.
