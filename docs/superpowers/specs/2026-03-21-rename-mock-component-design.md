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

## Scope of Changes

### 1. Core trait definitions (`crates/tuirealm/src/core/component.rs`)

- Rename `trait MockComponent` to `trait Component`
- Rename `trait Component<Msg, UserEvent>: MockComponent + Any` to `trait AppComponent<Msg, UserEvent>: Component + Any`
- Update doc comments to reflect new names
- Update the `dyn Component` impl block to `dyn AppComponent`

### 2. Re-exports (`crates/tuirealm/src/core/mod.rs`, `crates/tuirealm/src/lib.rs`)

- Change all `MockComponent` re-exports to `Component`
- Change all `Component` re-exports to `AppComponent`

### 3. Derive macro (`crates/tuirealm_derive/src/lib.rs`)

- Rename `#[proc_macro_derive(MockComponent, ...)]` to `#[proc_macro_derive(Component, ...)]`
- Update generated code: `impl Component for ...` instead of `impl MockComponent for ...`
- Update `use ::tuirealm::MockComponent` to `use ::tuirealm::Component` in the generated `use` block
- Update crate-level doc comments

### 4. Core crate internal usage (`crates/tuirealm/src/`)

All internal references to `MockComponent` and `Component` must be updated:

- `core/application.rs` — `Application` generic bounds, method signatures
- `core/subscription.rs` — subscription handling
- `core/view.rs` — `View` component storage and method signatures
- `terminal.rs` — terminal adapter trait bounds
- `mock/` — test fixture types (the `Mock` prefix on test types like `MockInput`, `MockFooInput` stays — those genuinely are test mocks)
- All example files under `crates/tuirealm/examples/`

### 5. Standard library (`crates/tui-realm-stdlib/`)

- All stdlib components implement `MockComponent` directly — update to `Component`
- Example files reference both traits — update accordingly

### 6. Extra crates (`crates/extra/tui-realm-textarea/`, `crates/extra/tui-realm-treeview/`)

- Same pattern: `impl MockComponent` becomes `impl Component`, `impl Component<..>` becomes `impl AppComponent<..>`

### 7. Documentation

- Update `CLAUDE.md` to reflect new trait names
- Update any docs under `docs/en/` that reference the old names

## Non-Changes

- **Test mock types keep their names**: `MockInput`, `MockFooInput`, `MockBarInput`, `MockEvent`, `MockMsg`, etc. in `crates/tuirealm/src/mock/` are genuine test fixtures — the `Mock` prefix is correct for them.
- **Feature flag name**: `derive` feature stays the same.
- **`#[component = "field"]` attribute**: The attribute name on the derive macro stays `component` since it refers to the inner component field, which is still semantically correct.

## Migration Impact

This is a breaking change (part of 4.0). All downstream users will need to:

1. Replace `MockComponent` with `Component` in trait implementations
2. Replace `Component<Msg, UserEvent>` with `AppComponent<Msg, UserEvent>`
3. Replace `#[derive(MockComponent)]` with `#[derive(Component)]`

These are mechanical find-and-replace changes.
