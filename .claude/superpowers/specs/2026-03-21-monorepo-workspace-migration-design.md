# Monorepo Workspace Migration (#162)

## Goal

Consolidate five separate tui-realm crates into a single workspace monorepo, bump all to 4.0.0, edition 2024, and migrate textarea/treeview to the tuirealm 4.0 API.

## Crates

| Crate | Source | Current Version | New Version | 4.0 Status |
|-------|--------|-----------------|-------------|------------|
| `tuirealm` | This repo (current `src/`) | 3.3.0 | 4.0.0 | Ready |
| `tuirealm_derive` | `veeso/tuirealm_derive` main branch | 2.0.1 | 4.0.0 (skips 3.x) | Ready (no tuirealm dep, proc-macro only) |
| `tui-realm-stdlib` | `veeso/tui-realm-stdlib` `feature/4.0` branch | 3.1.0 | 4.0.0 | Ready (uses git rev dep) |
| `tui-realm-textarea` | `veeso/tui-realm-textarea` main branch | 2.1.0 | 4.0.0 | Needs migration (on tuirealm v2) |
| `tui-realm-treeview` | `veeso/tui-realm-treeview` main branch | 3.0.0 | 4.0.0 | Needs migration (on tuirealm v3) |

Note: `tuirealm_derive` jumps from 2.x to 4.0.0 intentionally, to align all crate versions in the ecosystem.

## Directory Structure

```
tui-realm/
├── Cargo.toml                  # workspace manifest (no [package])
├── crates/
│   ├── tuirealm/               # core crate
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   └── examples/
│   ├── tuirealm_derive/        # proc-macro crate (must retain proc-macro = true, cannot export non-macro items)
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── tui-realm-stdlib/       # standard components library
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   └── examples/
│   └── extra/
│       ├── tui-realm-textarea/ # textarea component
│       │   ├── Cargo.toml
│       │   ├── src/
│       │   └── examples/
│       └── tui-realm-treeview/ # treeview component
│           ├── Cargo.toml
│           ├── src/
│           └── examples/
├── docs/
├── rustfmt.toml
└── CLAUDE.md
```

## Workspace Manifest (`Cargo.toml`)

### `[workspace]`

Uses `resolver = "3"` (the default for edition 2024).

```toml
[workspace]
resolver = "3"
members = [
    "crates/tuirealm",
    "crates/tuirealm_derive",
    "crates/tui-realm-stdlib",
    "crates/extra/tui-realm-textarea",
    "crates/extra/tui-realm-treeview",
]
```

### `[workspace.package]`

Shared package metadata inherited by all crates via `field.workspace = true`:

- `edition = "2024"`
- `rust-version = "1.86"`
- `license = "MIT"`
- `authors = ["Christian Visintin <christian.visintin@veeso.dev>", "hasezoey <hasezoey@gmail.com>"]`
- `categories = ["command-line-utilities"]`
- `keywords = ["tui", "terminal"]`
- `repository = "https://github.com/veeso/tui-realm"`
- `homepage = "https://github.com/veeso/tui-realm"`

### `[workspace.dependencies]`

All shared dependencies defined here with versions. Child crates reference them as `dep = { workspace = true }` or `dep = { workspace = true, optional = true }`.

Workspace crate deps:
- `tuirealm = { path = "crates/tuirealm", version = "4.0.0", default-features = false }`
- `tuirealm_derive = { path = "crates/tuirealm_derive", version = "4.0.0" }`
- `tui-realm-stdlib = { path = "crates/tui-realm-stdlib", version = "4.0.0", default-features = false }`

External deps (tuirealm core):
- `ratatui = { version = "0.30", default-features = false, features = ["std", "layout-cache"] }`
- `bitflags = "2"`
- `dyn-clone = "1"`
- `lazy-regex = "3"`
- `thiserror = "2"`
- `serde = { version = "1", features = ["derive"] }`
- `async-trait = "0.1"`
- `futures-util = { version = "0.3", default-features = false }`
- `tokio = { version = "1" }`
- `tokio-util = { version = "0.7" }`

Backend deps:
- `crossterm = "0.29"`
- `termion = "4"`
- `termwiz = "0.23"`

Derive deps:
- `proc-macro2 = "1"`
- `quote = "1"`
- `syn = "2"`

Stdlib/extra deps:
- `unicode-width = "0.2"`
- `textwrap = "0.16"`
- `orange-trees = "0.1.0"`
- `tui-textarea = "0.7"`
- `cli-clipboard = "0.4"`

Dev deps:
- `pretty_assertions = "1"`
- `tempfile = "3"`
- `toml = "0.9"`
- `rand = "0.10"`

## Child Crate Manifests

Each child `Cargo.toml`:
- Uses `field.workspace = true` for shared package fields
- Defines only crate-specific fields: `name`, `version = "4.0.0"`, `description`, `documentation`, `readme`, `include`
- References dependencies via `dep = { workspace = true }`, adding `optional = true` or `features = [...]` as needed
- `tuirealm_derive` must retain `[lib] proc-macro = true` and cannot depend on `tuirealm`

## Dependency Graph

```
tuirealm_derive (no deps on workspace crates, proc-macro only)
tuirealm -> tuirealm_derive (optional, via "derive" feature)
tui-realm-stdlib -> tuirealm (with "derive" feature)
tui-realm-textarea -> tuirealm (with "derive" feature)
tui-realm-treeview -> tuirealm (with "derive" feature)
tui-realm-textarea dev-dep -> tui-realm-stdlib
tui-realm-treeview dev-dep -> tui-realm-stdlib
```

## Migration: textarea (v2 to v4)

Changes required based on `docs/en/migrating-4.0.md` and the v2->v3->v4 delta:

1. **Edition**: 2021 -> 2024
2. **rust-version**: set to 1.86
3. **crossterm**: update to 0.29
4. **tui-textarea**: check compatibility, update if needed
5. **TextSpan**: Replace any `tuirealm::props::TextSpan` usage with `ratatui::text::{Span, Line, Text}`. New `AttrValue`/`PropValue` variants: `TextSpan`, `TextLine`, `Text`.
6. **State variants**: `One` -> `Single`, `Tup2` -> `Pair`, remove `Tup3`/`Tup4`
7. **PropPayload variants**: Same renames as State — `One` -> `Single`, `Tup2` -> `Pair`, remove `Tup3`/`Tup4`
8. **Component::on**: Parameter `Event` is now `&Event` (reference)
9. **Alignment**: `Alignment` -> `AlignmentHorizontal` / `AlignmentVertical`
10. **Title**: `(String, Alignment)` -> `Title` struct
11. **PropBoundExt**: Removed, methods on `dyn PropBound` directly
12. **Update trait**: Removed (examples need update)
13. **TerminalBridge**: Removed (examples need update)
14. **Poll return types**: `ListenerResult` -> `PortResult`, `PortError` with Intermittent/Permanent
15. **PollStrategy**: `UpToNoWait` -> `UpTo`, old `UpTo` removed. Timeout moved into `PollStrategy`.
16. **ApplicationError**: New `Poll` variant separate from Listener errors
17. **Dataset**: Removed from AttrValue if used

## Migration: treeview (v3 to v4)

Smaller delta (v3 -> v4 only):

1. **Edition**: verify 2024, **rust-version**: verify 1.86
2. **TextSpan**: Replace with ratatui types. New `AttrValue`/`PropValue` variants available.
3. **Component::on**: `Event` -> `&Event`
4. **State variants**: `One` -> `Single`, `Tup2` -> `Pair` if used
5. **PropPayload variants**: Same renames if used
6. **Title**: `(String, Alignment)` -> `Title` struct if used
7. **Alignment**: Split into `AlignmentHorizontal` / `AlignmentVertical` if used
8. **Update trait**: Removed (examples need update)
9. **TerminalBridge**: Removed (examples need update)
10. **Poll return types**: `ListenerResult` -> `PortResult` if used
11. **PollStrategy**: `UpToNoWait` -> `UpTo` if used. Timeout moved into `PollStrategy`.
12. **ApplicationError**: New `Poll` variant if used

## History Merge Strategy

Use `git filter-repo` to merge histories from all sub-repos, preserving full commit history with rewritten paths.

### Process per sub-repo

1. Clone the sub-repo into a temp directory
2. Checkout the target branch (e.g., `feature/4.0` for stdlib)
3. Run `git filter-repo --to-subdirectory-filter crates/<path>/` to rewrite all commits so files appear under their final monorepo path
4. In the tui-realm repo, add the rewritten clone as a remote
5. Fetch and merge with `--allow-unrelated-histories`

### Conventional Commit Rewriting

After all histories are merged, rewrite all ~781 commit messages to follow conventional commits format using `git filter-repo --message-callback`. Claude will classify each commit by reading the diff context:

- `feat:` / `feat(<scope>):` — new features, new components, new API surface
- `fix:` / `fix(<scope>):` — bug fixes
- `refactor:` / `refactor(<scope>):` — code restructuring without behavior change
- `chore:` / `chore(<scope>):` — version bumps, CI, dependency updates, formatting, tooling
- `docs:` / `docs(<scope>):` — documentation changes
- `test:` / `test(<scope>):` — test additions/changes
- `style:` / `style(<scope>):` — formatting-only changes
- `perf:` / `perf(<scope>):` — performance improvements

Scope should reflect the crate: `core`, `derive`, `stdlib`, `textarea`, `treeview`.

Merge commits will be prefixed with `chore:`. Commits that already follow conventional format will be kept as-is.

The message callback script will be generated by reading all commit messages + diffs and producing a mapping file, then applying it in a single `filter-repo` pass.

### Repos and branches

- `tuirealm_derive`: `main` -> `crates/tuirealm_derive/`
- `tui-realm-stdlib`: `feature/4.0` -> `crates/tui-realm-stdlib/`
- `tui-realm-textarea`: `main` -> `crates/extra/tui-realm-textarea/`
- `tui-realm-treeview`: `main` -> `crates/extra/tui-realm-treeview/`

### tui-realm (this repo) history

The existing tui-realm history also needs conventional commit rewriting. Since the core crate moves to `crates/tuirealm/`, paths should also be rewritten via filter-repo (`--to-subdirectory-filter` is not suitable here since there are top-level files like docs/ that stay). Instead, use `--path-rename src/:crates/tuirealm/src/` and similar renames for examples/.

## Implementation Order

1. **History merge**: Use `git filter-repo` to merge all sub-repo histories with path rewriting
2. **Conventional commits**: Rewrite all commit messages across the merged history
3. **Workspace setup**: Set up workspace root manifest and move `tuirealm` core into `crates/tuirealm/`
4. **Import derive**: Set up `crates/tuirealm_derive/` manifest with workspace deps
5. **Import stdlib**: Set up `crates/tui-realm-stdlib/` manifest, switch git dep to path
6. **Import textarea**: Set up `crates/extra/tui-realm-textarea/` manifest, migrate v2 -> v4
7. **Import treeview**: Set up `crates/extra/tui-realm-treeview/` manifest, migrate v3 -> v4
8. **Update CLAUDE.md**: Reflect new workspace structure, commands, module layout
9. **Final verification**: Full build, test, clippy, fmt

Each step should be verified with `cargo build` and `cargo test` before proceeding.

## Verification

After all changes:
1. `cargo build --all-features` — must compile
2. `cargo test --all-features` — all tests pass
3. `cargo clippy --all-targets --all-features -- -Dwarnings` — no warnings
4. `cargo +nightly fmt --all -- --check` — formatting clean (nightly required for unstable rustfmt options like `group_imports` and `imports_granularity`)
5. Each crate's examples should compile (may not all be runnable in CI due to terminal requirement)
6. Per-crate builds should also be verified (`cargo build -p <crate>`) to catch feature unification issues that `--all-features` may mask

## Feature Unification Note

Cargo workspaces unify features when building multiple crates together. Running `cargo build --all-features` may activate features in crates that don't explicitly request them (e.g., `crossterm` could be activated in textarea via stdlib). Per-crate builds (`cargo build -p tui-realm-textarea`) should also be verified to ensure each crate compiles correctly with only its declared features.

## Out of Scope

- Publishing to crates.io
- CI/CD pipeline updates
- Archiving the old repositories
- README/documentation updates beyond what's needed for compilation
