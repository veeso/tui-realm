# Monorepo Workspace Migration Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Consolidate five tui-realm crates into a single workspace monorepo at version 4.0.0 with edition 2024, merging git histories and rewriting commit messages to conventional commits.

**Architecture:** Use `git filter-repo` to merge histories from 4 external repos into this one, rewriting paths so files appear at their final monorepo locations. Then set up a Cargo workspace with shared metadata and dependencies. Finally migrate textarea (v2→v4) and treeview (v3→v4) to the tuirealm 4.0 API.

**Tech Stack:** Rust (edition 2024, MSRV 1.86), Cargo workspaces, git-filter-repo, ratatui 0.30

**Spec:** `.claude/superpowers/specs/2026-03-21-monorepo-workspace-migration-design.md`

---

### Task 0: Rewrite tui-realm core history paths

Before merging sub-repos, rewrite the existing tui-realm commit history so `src/` and `examples/` appear under `crates/tuirealm/` in all historical commits.

- [ ] **Step 1: Rewrite paths in existing history**

```bash
cd /Users/veeso/Sviluppo/opensource/rust/tui-realm
git filter-repo \
    --path-rename src/:crates/tuirealm/src/ \
    --path-rename examples/:crates/tuirealm/examples/ \
    --force
```

> **Note:** `git filter-repo --force` removes the `origin` remote. It must be re-added after all filter-repo operations are complete.

- [ ] **Step 2: Verify rewritten paths**

```bash
ls crates/tuirealm/src/lib.rs
ls crates/tuirealm/examples/
```

- [ ] **Step 3: Verify old root paths are gone**

```bash
# These should not exist
test ! -d src && test ! -d examples && echo "OK"
```

---

### Task 1: Merge git history from tuirealm_derive

**Files:**
- New files under: `crates/tuirealm_derive/` (from history rewrite)

- [ ] **Step 1: Clone tuirealm_derive to temp dir and rewrite paths**

```bash
git clone /Users/veeso/Sviluppo/opensource/rust/tuirealm_derive /tmp/tuirealm_derive_rewritten
cd /tmp/tuirealm_derive_rewritten
git filter-repo --to-subdirectory-filter crates/tuirealm_derive/
```

- [ ] **Step 2: Add as remote, fetch, and merge**

```bash
cd /Users/veeso/Sviluppo/opensource/rust/tui-realm
git remote add tuirealm_derive /tmp/tuirealm_derive_rewritten
git fetch tuirealm_derive
git merge tuirealm_derive/main --allow-unrelated-histories -m "chore: merge tuirealm_derive history"
git remote remove tuirealm_derive
```

- [ ] **Step 3: Verify files exist**

```bash
ls crates/tuirealm_derive/src/lib.rs
```

- [ ] **Step 4: Clean up temp clone**

```bash
rm -rf /tmp/tuirealm_derive_rewritten
```

---

### Task 2: Merge git history from tui-realm-stdlib (feature/4.0 branch)

**Files:**
- New files under: `crates/tui-realm-stdlib/`

- [ ] **Step 1: Clone stdlib to temp dir, checkout feature/4.0, and rewrite paths**

```bash
git clone /Users/veeso/Sviluppo/opensource/rust/tui-realm-stdlib /tmp/tui-realm-stdlib_rewritten
cd /tmp/tui-realm-stdlib_rewritten
git checkout origin/feature/4.0
git checkout -b feature/4.0
git filter-repo --to-subdirectory-filter crates/tui-realm-stdlib/
```

- [ ] **Step 2: Add as remote, fetch, and merge**

```bash
cd /Users/veeso/Sviluppo/opensource/rust/tui-realm
git remote add tui-realm-stdlib /tmp/tui-realm-stdlib_rewritten
git fetch tui-realm-stdlib
git merge tui-realm-stdlib/feature/4.0 --allow-unrelated-histories -m "chore: merge tui-realm-stdlib history"
git remote remove tui-realm-stdlib
```

- [ ] **Step 3: Verify files exist**

```bash
ls crates/tui-realm-stdlib/src/lib.rs
```

- [ ] **Step 4: Clean up**

```bash
rm -rf /tmp/tui-realm-stdlib_rewritten
```

---

### Task 3: Merge git history from tui-realm-textarea

**Files:**
- New files under: `crates/extra/tui-realm-textarea/`

- [ ] **Step 1: Clone textarea to temp dir and rewrite paths**

```bash
git clone /Users/veeso/Sviluppo/opensource/rust/tui-realm-textarea /tmp/tui-realm-textarea_rewritten
cd /tmp/tui-realm-textarea_rewritten
git filter-repo --to-subdirectory-filter crates/extra/tui-realm-textarea/
```

- [ ] **Step 2: Add as remote, fetch, and merge**

```bash
cd /Users/veeso/Sviluppo/opensource/rust/tui-realm
git remote add tui-realm-textarea /tmp/tui-realm-textarea_rewritten
git fetch tui-realm-textarea
git merge tui-realm-textarea/main --allow-unrelated-histories -m "chore: merge tui-realm-textarea history"
git remote remove tui-realm-textarea
```

- [ ] **Step 3: Verify files exist**

```bash
ls crates/extra/tui-realm-textarea/src/lib.rs
```

- [ ] **Step 4: Clean up**

```bash
rm -rf /tmp/tui-realm-textarea_rewritten
```

---

### Task 4: Merge git history from tui-realm-treeview

**Files:**
- New files under: `crates/extra/tui-realm-treeview/`

- [ ] **Step 1: Clone treeview to temp dir and rewrite paths**

```bash
git clone /Users/veeso/Sviluppo/opensource/rust/tui-realm-treeview /tmp/tui-realm-treeview_rewritten
cd /tmp/tui-realm-treeview_rewritten
git filter-repo --to-subdirectory-filter crates/extra/tui-realm-treeview/
```

- [ ] **Step 2: Add as remote, fetch, and merge**

```bash
cd /Users/veeso/Sviluppo/opensource/rust/tui-realm
git remote add tui-realm-treeview /tmp/tui-realm-treeview_rewritten
git fetch tui-realm-treeview
git merge tui-realm-treeview/main --allow-unrelated-histories -m "chore: merge tui-realm-treeview history"
git remote remove tui-realm-treeview
```

- [ ] **Step 3: Verify files exist**

```bash
ls crates/extra/tui-realm-treeview/src/lib.rs
```

- [ ] **Step 4: Clean up**

```bash
rm -rf /tmp/tui-realm-treeview_rewritten
```

---

### Task 5: Rewrite commit messages to conventional commits

After all histories are merged, rewrite all commit messages across the entire repo history using `git filter-repo --message-callback`.

- [ ] **Step 1: Export all commit messages for analysis**

```bash
cd /Users/veeso/Sviluppo/opensource/rust/tui-realm
git log --format="%H %s" > /tmp/tui-realm-commits.txt
```

- [ ] **Step 2: Generate a Python message-callback script**

Create a Python script at `/tmp/rewrite-commits.py` that maps old commit messages to conventional commit format. The script should:
- Read each commit message
- Classify based on keywords and patterns:
  - Messages starting with `feat:`, `fix:`, `chore:`, etc. → keep as-is
  - `Merge` → `chore: <original message>`
  - `Add`/`Implement`/`Create` → `feat: <cleaned message>`
  - `Fix`/`Bugfix`/`Resolve` → `fix: <cleaned message>`
  - `Update`/`Bump`/`Upgrade` deps → `chore(deps): <cleaned message>`
  - `Update`/`Refactor`/`Rename`/`Move`/`Clean` code → `refactor: <cleaned message>`
  - `Test`/`Add test` → `test: <cleaned message>`
  - `Doc`/`README`/`CHANGELOG` → `docs: <cleaned message>`
  - Version bumps → `chore(release): <cleaned message>`
  - CI/workflow → `ci: <cleaned message>`
  - Fallback → `chore: <cleaned message>`
- Add scope based on which files the commit touches (use path prefix from rewritten history):
  - `crates/tuirealm_derive/` → `(derive)`
  - `crates/tui-realm-stdlib/` → `(stdlib)`
  - `crates/extra/tui-realm-textarea/` → `(textarea)`
  - `crates/extra/tui-realm-treeview/` → `(treeview)`
  - `src/` or `crates/tuirealm/` → `(core)`

- [ ] **Step 3: Run git filter-repo with the message callback**

```bash
cd /Users/veeso/Sviluppo/opensource/rust/tui-realm
git filter-repo --message-callback "$(cat /tmp/rewrite-commits.py)" --force
```

- [ ] **Step 4: Spot-check the rewritten messages**

```bash
git log --oneline | head -30
git log --oneline | tail -30
```

- [ ] **Step 5: Restore origin remote**

```bash
git remote add origin git@github.com:veeso/tui-realm.git
```

- [ ] **Step 6: Clean up**

```bash
rm /tmp/tui-realm-commits.txt /tmp/rewrite-commits.py
```

---

### Task 6: Set up workspace Cargo.toml manifests

After Task 0's path rewrite, `src/` and `examples/` already live under `crates/tuirealm/`. After Tasks 1-4, all sub-repo files are already in their target directories. This task creates the workspace and child Cargo.toml files.

**Files:**
- Create: `crates/tuirealm/Cargo.toml`
- Modify: `Cargo.toml` (root) → workspace manifest
- Delete: conflicting `Cargo.lock` files from imported crates
- Delete: `crates/tuirealm/Cargo.toml` leftovers from imported repos (if any)

- [ ] **Step 1: Remove conflicting Cargo files from imported crates**

```bash
# Remove any Cargo.lock files from sub-crates (the workspace root will have one)
find crates/ -name "Cargo.lock" -delete
```

- [ ] **Step 2: Create workspace root Cargo.toml**

Use @cargo-toml-conventions skill. Create root `Cargo.toml`:

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

[workspace.package]
authors = [
    "Christian Visintin <christian.visintin@veeso.dev>",
    "hasezoey <hasezoey@gmail.com>",
]
categories = ["command-line-utilities"]
edition = "2024"
homepage = "https://github.com/veeso/tui-realm"
keywords = ["tui", "terminal"]
license = "MIT"
repository = "https://github.com/veeso/tui-realm"
rust-version = "1.86"

[workspace.dependencies]
# Workspace crates
tui-realm-stdlib = { path = "crates/tui-realm-stdlib", version = "4.0.0", default-features = false }
tuirealm = { path = "crates/tuirealm", version = "4.0.0", default-features = false }
tuirealm_derive = { path = "crates/tuirealm_derive", version = "4.0.0" }

# Core deps
async-trait = "0.1"
bitflags = "2"
dyn-clone = "1"
futures-util = { version = "0.3", default-features = false }
lazy-regex = "3"
ratatui = { version = "0.30", default-features = false, features = ["std", "layout-cache"] }
serde = { version = "1", features = ["derive"] }
thiserror = "2"
tokio = { version = "1" }
tokio-util = { version = "0.7" }

# Backend deps
crossterm = "0.29"
termion = "4"
termwiz = "0.23"

# Derive deps
proc-macro2 = "1"
quote = "1"
syn = "2"

# Stdlib/extra deps
cli-clipboard = "0.4"
orange-trees = "0.1.0"
textwrap = "0.16"
tui-textarea = "0.7"
unicode-width = "0.2"

# Dev deps
pretty_assertions = "1"
rand = "0.10"
tempfile = "3"
toml = "0.9"
```

- [ ] **Step 3: Create crates/tuirealm/Cargo.toml**

Use @cargo-toml-conventions skill. Convert the existing Cargo.toml into a child manifest that inherits workspace fields:

```toml
[package]
name = "tuirealm"
version = "4.0.0"
authors.workspace = true
categories.workspace = true
description = "A ratatui framework to build tui interfaces, inspired by React and Elm."
documentation = "https://docs.rs/tuirealm"
edition.workspace = true
homepage.workspace = true
include = ["examples/**/*", "src/**/*", "LICENSE", "README.md", "CHANGELOG.md"]
keywords.workspace = true
license.workspace = true
readme = "README.md"
repository.workspace = true
rust-version.workspace = true

[dependencies]
async-trait = { workspace = true, optional = true }
bitflags = { workspace = true }
dyn-clone = { workspace = true }
futures-util = { workspace = true, optional = true }
lazy-regex = { workspace = true }
ratatui = { workspace = true }
serde = { workspace = true, optional = true }
thiserror = { workspace = true }
tokio = { workspace = true, features = ["rt", "macros", "time"], default-features = false, optional = true }
tokio-util = { workspace = true, features = ["rt"], default-features = false, optional = true }
tuirealm_derive = { workspace = true, optional = true }

crossterm = { workspace = true, optional = true }
termion = { workspace = true, optional = true }
termwiz = { workspace = true, optional = true }

[dev-dependencies]
pretty_assertions = { workspace = true }
tempfile = { workspace = true }
tokio = { workspace = true, features = ["full"] }
toml = { workspace = true }

[features]
default = ["derive", "crossterm"]
async-ports = [
    "dep:async-trait",
    "dep:tokio",
    "dep:tokio-util",
    "crossterm?/event-stream",
    "dep:futures-util",
]
derive = ["dep:tuirealm_derive"]
serialize = ["dep:serde", "bitflags/serde"]

crossterm = ["dep:crossterm", "ratatui/crossterm"]
termion = ["dep:termion", "ratatui/termion"]
termwiz = ["dep:termwiz", "ratatui/termwiz"]

[[example]]
name = "async-ports"
path = "examples/async_ports.rs"
required-features = ["async-ports", "crossterm"]

[[example]]
name = "demo"
path = "examples/demo/demo.rs"
required-features = ["crossterm"]

[[example]]
name = "user-events"
path = "examples/user_events/user_events.rs"
required-features = ["crossterm"]

[[example]]
name = "arbitrary-data"
path = "examples/arbitrary_data.rs"
required-features = ["crossterm"]

[[example]]
name = "inline-display"
path = "examples/inline_display.rs"
required-features = ["crossterm"]

[package.metadata.docs.rs]
all-features = true
rustdoc-args = ["--cfg", "docsrs"]
```

- [ ] **Step 4: Delete the old root Cargo.toml (replaced by workspace manifest)**

The old root `Cargo.toml` was the tuirealm package manifest. It's now replaced by the workspace manifest. The old content has been moved to `crates/tuirealm/Cargo.toml`.

- [ ] **Step 5: Regenerate Cargo.lock**

```bash
cargo generate-lockfile
```

- [ ] **Step 6: Verify core crate builds**

```bash
cargo build -p tuirealm
cargo test -p tuirealm
```

- [ ] **Step 7: Commit**

```bash
git add -A
git commit -m "refactor(core): set up workspace with tuirealm in crates/tuirealm/"
```

---

### Task 7: Set up tuirealm_derive workspace manifest

**Files:**
- Modify: `crates/tuirealm_derive/Cargo.toml`

- [ ] **Step 1: Rewrite Cargo.toml to use workspace inheritance**

Use @cargo-toml-conventions skill:

```toml
[package]
name = "tuirealm_derive"
version = "4.0.0"
authors.workspace = true
categories.workspace = true
description = "Derive macro which automatically implements the MockComponent trait"
documentation = "https://docs.rs/tuirealm_derive"
edition.workspace = true
homepage.workspace = true
include = ["src/**/*", "LICENSE", "README.md", "CHANGELOG.md"]
keywords.workspace = true
license.workspace = true
readme = "README.md"
repository.workspace = true
rust-version.workspace = true

[lib]
doctest = false
name = "tuirealm_derive"
proc-macro = true

[dependencies]
proc-macro2 = { workspace = true }
quote = { workspace = true }
syn = { workspace = true }
```

- [ ] **Step 2: Verify derive crate builds**

```bash
cargo build -p tuirealm_derive
```

- [ ] **Step 3: Commit**

```bash
git add crates/tuirealm_derive/Cargo.toml
git commit -m "chore(derive): update Cargo.toml for workspace inheritance and bump to 4.0.0"
```

---

### Task 8: Set up tui-realm-stdlib workspace manifest

**Files:**
- Modify: `crates/tui-realm-stdlib/Cargo.toml`

- [ ] **Step 1: Rewrite Cargo.toml to use workspace inheritance and path deps**

Use @cargo-toml-conventions skill. Replace the git rev dependency with workspace path dep:

```toml
[package]
name = "tui-realm-stdlib"
version = "4.0.0"
authors.workspace = true
categories.workspace = true
description = "Standard components library for tui-realm."
documentation = "https://docs.rs/tui-realm-stdlib"
edition.workspace = true
homepage.workspace = true
include = ["examples/**/*", "src/**/*", "LICENSE", "README.md", "CHANGELOG.md"]
keywords.workspace = true
license.workspace = true
readme = "README.md"
repository.workspace = true
rust-version.workspace = true

[dependencies]
textwrap = { workspace = true }
tuirealm = { workspace = true, features = ["derive"] }
unicode-width = { workspace = true }

[dev-dependencies]
pretty_assertions = { workspace = true }
rand = { workspace = true }
tuirealm = { workspace = true, features = ["crossterm"] }

[[example]]
name = "bar_chart"
path = "examples/bar_chart.rs"

[[example]]
name = "canvas"
path = "examples/canvas.rs"

[[example]]
name = "chart"
path = "examples/chart.rs"

[[example]]
name = "checkbox"
path = "examples/checkbox.rs"

[[example]]
name = "container"
path = "examples/container.rs"

[[example]]
name = "input"
path = "examples/input.rs"

[[example]]
name = "label"
path = "examples/label.rs"

[[example]]
name = "line_gauge"
path = "examples/line_gauge.rs"

[[example]]
name = "list"
path = "examples/list.rs"

[[example]]
name = "paragraph"
path = "examples/paragraph.rs"

[[example]]
name = "gauge"
path = "examples/gauge.rs"

[[example]]
name = "radio"
path = "examples/radio.rs"

[[example]]
name = "select"
path = "examples/select.rs"

[[example]]
name = "span"
path = "examples/span.rs"

[[example]]
name = "sparkline"
path = "examples/sparkline.rs"

[[example]]
name = "spinner"
path = "examples/spinner.rs"

[[example]]
name = "table"
path = "examples/table.rs"

[[example]]
name = "textarea"
path = "examples/textarea.rs"
```

- [ ] **Step 2: Verify stdlib builds and tests pass**

```bash
cargo build -p tui-realm-stdlib
cargo test -p tui-realm-stdlib
```

- [ ] **Step 3: Commit**

```bash
git add crates/tui-realm-stdlib/Cargo.toml
git commit -m "chore(stdlib): update Cargo.toml for workspace inheritance and bump to 4.0.0"
```

---

### Task 9: Migrate tui-realm-textarea from v2 to v4

**Files:**
- Modify: `crates/extra/tui-realm-textarea/Cargo.toml`
- Modify: `crates/extra/tui-realm-textarea/src/lib.rs`
- Modify: `crates/extra/tui-realm-textarea/src/fmt.rs` (if needed)
- Modify: `crates/extra/tui-realm-textarea/examples/editor.rs`
- Modify: `crates/extra/tui-realm-textarea/examples/single_line.rs`

- [ ] **Step 1: Rewrite Cargo.toml for workspace**

Use @cargo-toml-conventions skill:

```toml
[package]
name = "tui-realm-textarea"
version = "4.0.0"
authors.workspace = true
categories.workspace = true
description = "textarea component for tui-realm"
documentation = "https://docs.rs/tui-realm-textarea"
edition.workspace = true
homepage.workspace = true
include = ["examples/**/*", "src/**/*", "LICENSE", "README.md", "CHANGELOG.md"]
keywords.workspace = true
license.workspace = true
readme = "README.md"
repository.workspace = true
rust-version.workspace = true

[dependencies]
cli-clipboard = { workspace = true, optional = true }
lazy-regex = { workspace = true }
tui-textarea = { workspace = true }
tuirealm = { workspace = true, features = ["derive"] }

[dev-dependencies]
crossterm = { workspace = true }
pretty_assertions = { workspace = true }
tui-realm-stdlib = { workspace = true }

[features]
default = ["crossterm"]
clipboard = ["dep:cli-clipboard"]
crossterm = ["tuirealm/crossterm"]
search = ["tui-textarea/search"]
termion = ["tuirealm/termion"]

[[example]]
name = "editor"
path = "examples/editor.rs"
required-features = ["crossterm"]

[[example]]
name = "single-line"
path = "examples/single_line.rs"
required-features = ["crossterm"]
```

- [ ] **Step 2: Migrate src/lib.rs API changes**

Apply all v2→v4 migration changes to `crates/extra/tui-realm-textarea/src/lib.rs`:

1. **PropPayload::One → PropPayload::Single** (line 272, 523):
   - `PropPayload::One(PropValue::Usize(max))` → `PropPayload::Single(PropValue::Usize(max))`

2. **PropPayload::Tup2 → PropPayload::Pair** (lines 299, 321, 514, 529):
   - `PropPayload::Tup2((...)` → `PropPayload::Pair((...)`

3. **AttrValue::Title((String, Alignment)) → AttrValue::Title(Title)** (lines 254-258, 384):
   - `AttrValue::Title((t.as_ref().to_string(), a))` → `AttrValue::Title(Title::from(t.as_ref()).alignment(a))`
   - `AttrValue::Title((title, alignment))` in pattern match → `AttrValue::Title(title)` and use `title` directly as a `Title` struct

4. **Alignment → HorizontalAlignment** (line 158):
   - `use tuirealm::props::{ Alignment, ...}` → `use tuirealm::props::{ HorizontalAlignment, ...}`
   - The `title` method signature: `a: Alignment` → `a: HorizontalAlignment`

5. **title_alignment deprecated** (line 385):
   - `block.title(title).title_alignment(alignment)` → use `Title` struct directly: `block.title(title.content).title_alignment(...)` or use `Block::title_top(title.content)` per ratatui 0.30 API

6. **Remove `extern crate lazy_regex`** (line 151): In edition 2024, `#[macro_use] extern crate` is not needed. Just use `use lazy_regex::*;` or use qualified paths.

7. **Update doc example** (lines 96-138): Remove references to `TerminalBridge`, `Update`, `Alignment` → `HorizontalAlignment`, `State::One` → `State::Single` (if present in docs)

- [ ] **Step 3: Update get_block method for Title struct**

In `get_block()` method, change the title handling. The `Title` struct has a `content: LineStatic` field where alignment is already embedded via `Title::alignment()`, and a `position: TitlePosition` field:

```rust
// Old:
if let Some(AttrValue::Title((title, alignment))) = self.query(Attribute::Title) {
    block = block.title(title).title_alignment(alignment);
}

// New:
use tuirealm::props::{Title, TitlePosition};

if let Some(AttrValue::Title(title)) = self.query(Attribute::Title) {
    // alignment is already embedded in title.content (the Line)
    block = match title.position {
        TitlePosition::Top => block.title_top(title.content.clone()),
        TitlePosition::Bottom => block.title_bottom(title.content.clone()),
    };
}
```

Note: Check if `TitlePosition` exists. If not, just use `block.title_top(title.content.clone())`.

- [ ] **Step 4: Migrate examples**

Read and update `crates/extra/tui-realm-textarea/examples/editor.rs` and `single_line.rs`:
- Replace `TerminalBridge` with direct backend usage
- Replace `Update` trait with `impl Model`
- Replace `Alignment` with `HorizontalAlignment`
- Replace `Event` by-value to `&Event` in `Component::on`
- Update `PollStrategy` if used
- Update any `State::One` to `State::Single`

- [ ] **Step 5: Verify textarea builds and tests pass**

```bash
cargo build -p tui-realm-textarea
cargo test -p tui-realm-textarea
```

- [ ] **Step 6: Commit**

```bash
git add crates/extra/tui-realm-textarea/
git commit -m "feat(textarea): migrate to tuirealm 4.0 API and bump to 4.0.0"
```

---

### Task 10: Migrate tui-realm-treeview from v3 to v4

**Files:**
- Modify: `crates/extra/tui-realm-treeview/Cargo.toml`
- Modify: `crates/extra/tui-realm-treeview/src/lib.rs`
- Modify: `crates/extra/tui-realm-treeview/src/widget.rs`
- Modify: `crates/extra/tui-realm-treeview/src/tree_state.rs`
- Modify: `crates/extra/tui-realm-treeview/examples/demo.rs`

- [ ] **Step 1: Rewrite Cargo.toml for workspace**

Use @cargo-toml-conventions skill:

```toml
[package]
name = "tui-realm-treeview"
version = "4.0.0"
authors.workspace = true
categories.workspace = true
description = "Treeview component for tui-realm"
documentation = "https://docs.rs/tui-realm-treeview"
edition.workspace = true
homepage.workspace = true
include = ["examples/**/*", "src/**/*", "LICENSE", "README.md", "CHANGELOG.md"]
keywords.workspace = true
license.workspace = true
readme = "README.md"
repository.workspace = true
rust-version.workspace = true

[dependencies]
orange-trees = { workspace = true }
tuirealm = { workspace = true, features = ["derive"] }
unicode-width = { workspace = true }

[dev-dependencies]
crossterm = { workspace = true }
pretty_assertions = { workspace = true }
tui-realm-stdlib = { workspace = true }

[features]
default = ["crossterm"]
crossterm = ["tuirealm/crossterm"]
termion = ["tuirealm/termion"]

[[example]]
name = "demo"
path = "examples/demo.rs"
required-features = ["crossterm"]
```

- [ ] **Step 2: Migrate TextSpan usage in src/lib.rs**

The `NodeValue` impl for `Vec<TextSpan>` needs to change. In v4, `TextSpan` no longer exists as a standalone struct — it's now `ratatui::text::Span` (re-exported as `tuirealm::ratatui::text::Span`).

Replace the import and impl:

```rust
// Old (line 219):
use tuirealm::props::{
    Alignment, AttrValue, Attribute, Borders, Color, Props, Style, TextModifiers, TextSpan,
};

// New:
use tuirealm::props::{
    AttrValue, Attribute, Borders, Color, HorizontalAlignment, Props, Style, TextModifiers,
};
use tuirealm::ratatui::text::Span;
```

Replace `NodeValue for Vec<TextSpan>` (lines 239-253):

```rust
// Old:
impl NodeValue for Vec<TextSpan> {
    fn render_parts_iter(&self) -> impl Iterator<Item = (&str, Option<Style>)> {
        self.iter().map(|span| {
            (
                span.content.as_str(),
                Some(
                    Style::new()
                        .fg(span.fg)
                        .bg(span.bg)
                        .add_modifier(span.modifiers),
                ),
            )
        })
    }
}

// New:
impl NodeValue for Vec<Span<'static>> {
    fn render_parts_iter(&self) -> impl Iterator<Item = (&str, Option<Style>)> {
        self.iter().map(|span| {
            (span.content.as_ref(), Some(span.style))
        })
    }
}
```

- [ ] **Step 3: Migrate Title usage in src/lib.rs**

Replace `AttrValue::Title((String, Alignment))` pattern:

```rust
// Old (line 326):
pub fn title<S: Into<String>>(mut self, t: S, a: Alignment) -> Self {
    self.attr(Attribute::Title, AttrValue::Title((t.into(), a)));
    self
}

// New:
use tuirealm::props::Title;

pub fn title<S: Into<String>>(mut self, t: S, a: HorizontalAlignment) -> Self {
    self.attr(Attribute::Title, AttrValue::Title(Title::from(t.into()).alignment(a)));
    self
}
```

Update `view()` method title extraction (lines 457-462). Pass by reference to avoid unnecessary clone:

```rust
// Old:
let title = self
    .props
    .get_ref(Attribute::Title)
    .and_then(|v| v.as_title())
    .map(|v| (v.0.as_str(), v.1))
    .unwrap_or(("", Alignment::Center));
// ...
let div = Self::get_block(borders, title, focus, inactive_style);

// New:
let title = self
    .props
    .get_ref(Attribute::Title)
    .and_then(|v| v.as_title());
// ...
let div = Self::get_block(borders, title, focus, inactive_style);
```

Update `get_block` signature and usage to accept `Option<&Title>`:

```rust
// Old:
fn get_block(
    props: Borders,
    title: (&str, Alignment),
    focus: bool,
    inactive_style: Option<Style>,
) -> Block<'_> {
    Block::default()
        .borders(props.sides)
        .border_style(...)
        .border_type(props.modifiers)
        .title(title.0)
        .title_alignment(title.1)
}

// New:
use tuirealm::props::{Title, TitlePosition};

fn get_block(
    props: Borders,
    title: Option<&Title>,
    focus: bool,
    inactive_style: Option<Style>,
) -> Block<'_> {
    let mut block = Block::default()
        .borders(props.sides)
        .border_style(match focus {
            true => props.style(),
            false => inactive_style
                .unwrap_or_else(|| Style::default().fg(Color::Reset).bg(Color::Reset)),
        })
        .border_type(props.modifiers);
    if let Some(title) = title {
        block = match title.position {
            TitlePosition::Top => block.title_top(title.content.clone()),
            TitlePosition::Bottom => block.title_bottom(title.content.clone()),
        };
    }
    block
}
```

Note: Check if `TitlePosition` exists. If not, just use `block.title_top(title.content.clone())`.

- [ ] **Step 4: Migrate State::One → State::Single**

In `src/lib.rs`, replace all `State::One` with `State::Single`:

```rust
// state() method (line 531):
Some(id) => State::Single(StateValue::String(id.to_string())),

// Tests: all State::One(...) → State::Single(...)
```

- [ ] **Step 5: Update doc examples in lib.rs**

Update the doc comment example (lines 80-183):
- Remove `extern crate tui_realm_treeview;` and `extern crate tuirealm;` (unnecessary in edition 2024)
- `Component::on(&mut self, ev: Event<NoUserEvent>)` → `Component::on(&mut self, ev: &Event<NoUserEvent>)`
- `State::One(StateValue::String(node))` → `State::Single(StateValue::String(node))`
- `Alignment` → `HorizontalAlignment`

- [ ] **Step 6: Migrate examples/demo.rs**

Read and update `crates/extra/tui-realm-treeview/examples/demo.rs`:
- Replace `TerminalBridge` with direct backend usage
- Replace `Update` trait with `impl Model`
- Replace `Alignment` with `HorizontalAlignment`
- Replace `Event` by-value to `&Event` in `Component::on`
- Replace `State::One` with `State::Single`
- Update `PollStrategy` if used

- [ ] **Step 7: Verify treeview builds and tests pass**

```bash
cargo build -p tui-realm-treeview
cargo test -p tui-realm-treeview
```

- [ ] **Step 8: Commit**

```bash
git add crates/extra/tui-realm-treeview/
git commit -m "feat(treeview): migrate to tuirealm 4.0 API and bump to 4.0.0"
```

---

### Task 11: Clean up imported files

Remove files that shouldn't be in the monorepo (per-crate CI, gitignore, rustfmt.toml duplicates, etc.)

**Files:**
- Delete: `crates/tuirealm_derive/.github/` (if exists)
- Delete: `crates/tui-realm-stdlib/.github/` (if exists, not present in feature/4.0)
- Delete: `crates/extra/tui-realm-textarea/.github/`
- Delete: `crates/extra/tui-realm-treeview/.github/`
- Delete: `crates/extra/tui-realm-textarea/rustfmt.toml` (use root rustfmt.toml)
- Delete: `crates/extra/tui-realm-treeview/rustfmt.toml` (use root rustfmt.toml)
- Delete: `crates/extra/tui-realm-textarea/.gitignore` (use root .gitignore)
- Delete: `crates/extra/tui-realm-treeview/.gitignore` (use root .gitignore)
- Delete: `crates/tuirealm_derive/.gitignore` (use root .gitignore)
- Delete: any `.DS_Store` files
- Delete: `crates/*/CODE_OF_CONDUCT.md`, `crates/*/CONTRIBUTING.md` (keep only root copies)

- [ ] **Step 1: Remove per-crate CI/meta files**

```bash
rm -rf crates/tuirealm_derive/.github
rm -rf crates/extra/tui-realm-textarea/.github
rm -rf crates/extra/tui-realm-treeview/.github
rm -f crates/extra/tui-realm-textarea/rustfmt.toml
rm -f crates/extra/tui-realm-treeview/rustfmt.toml
rm -f crates/tuirealm_derive/.gitignore
rm -f crates/extra/tui-realm-textarea/.gitignore
rm -f crates/extra/tui-realm-treeview/.gitignore
find crates/ -name ".DS_Store" -delete
rm -f crates/tuirealm_derive/CODE_OF_CONDUCT.md
rm -f crates/tui-realm-stdlib/CODE_OF_CONDUCT.md crates/tui-realm-stdlib/CONTRIBUTING.md
rm -f crates/extra/tui-realm-textarea/CODE_OF_CONDUCT.md crates/extra/tui-realm-textarea/CONTRIBUTING.md
rm -f crates/extra/tui-realm-treeview/CODE_OF_CONDUCT.md crates/extra/tui-realm-treeview/CONTRIBUTING.md
```

- [ ] **Step 2: Commit**

```bash
git add -A
git commit -m "chore: remove per-crate CI, meta files, and duplicates"
```

---

### Task 12: Update CLAUDE.md for workspace structure

**Files:**
- Modify: `CLAUDE.md`

- [ ] **Step 1: Update CLAUDE.md**

Update commands, module layout, and feature flags to reflect the new workspace structure. Key changes:
- Commands now reference workspace: `cargo build --workspace`, `cargo test --workspace --all-features`
- Per-crate commands: `cargo build -p tuirealm`, `cargo test -p tui-realm-stdlib`
- Module layout updated to reflect `crates/` structure
- Add workspace members list
- Update feature flags section to cover all crates

- [ ] **Step 2: Commit**

```bash
git add CLAUDE.md
git commit -m "docs: update CLAUDE.md for workspace structure"
```

---

### Task 13: Final verification

- [ ] **Step 1: Full workspace build**

```bash
cargo build --workspace --all-features
```

- [ ] **Step 2: Full test suite**

```bash
cargo test --workspace --all-features
```

- [ ] **Step 3: Clippy**

```bash
cargo clippy --workspace --all-targets --all-features -- -Dwarnings
```

- [ ] **Step 4: Format check**

```bash
cargo +nightly fmt --all -- --check
```

If formatting issues, fix with:

```bash
cargo +nightly fmt --all
```

- [ ] **Step 5: Per-crate verification (feature unification check)**

```bash
cargo build -p tuirealm
cargo build -p tuirealm_derive
cargo build -p tui-realm-stdlib
cargo build -p tui-realm-textarea
cargo build -p tui-realm-treeview
```

- [ ] **Step 6: Fix any issues and commit**

```bash
git add -A
git commit -m "chore: final formatting and fixes"
```
