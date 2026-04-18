## 4.0.0

Released on 2026-04-18

### ⚠ Breaking Changes
- TerminalAdapter as a trait; draw as a method of TerminalAdapter
  > TerminalAdapter as a trait; draw as a method of TerminalAdapter
- `CmdResult::Custom(&'static str)` changed to `CmdResult::Custom(&'static str, State)`
  > `CmdResult::Custom(&'static str)` changed to `CmdResult::Custom(&'static str, State)`
- 3.0.0
  > I hate semver
- **props:** remove "TextSpan", introduce "Span", "Line" and "Text"
  > "TextSpan" has been replaced with "Span", "Line" and "Text".
- **props:** replace title tuple with struct
  > Titles are now a "Title" struct instead of a tuple.
- **props:** remove "PropPayload::Tup*" variants
  > "PropPayload::Tup*" variants
- **core:** change "Component::on" parameter "Event" to be a reference
  > "Component::on" parameter "Event" is now a reference.
- **utils:** remove "get_title_or_center"
  > "utils::get_title_or_center" has been removed
- rename Component to AppComponent, MockComponent to Component (#161)
  > rename Component to AppComponent, MockComponent to Component (#161)

### Added

- termion 2- changelog- readme- ratatui 0.28; dropped support for tui-rs- 💥 TerminalAdapter as a trait; draw as a method of TerminalAdapter- 💥 `CmdResult::Custom(&'static str)` changed to `CmdResult::Custom(&'static str, State)`- Added new `subclause_and!(Id::Foo, Id::Bar, Id::Baz)` and `subclause_or!(Id::Foo, Id::Bar, Id::Baz)` macros.- **event:** add event handling for mouse events (#79)
  > crossterm only, as termion does not support mouse events- **listener:** poll a port multiple times until either "max_poll" or returned "None" (#78)
  > * feat(listener): poll a port multiple times until either "max_poll" or returned "None"- **core::props::Props:** add function "get_ref" (#89)
  > to get the AttrValue by reference instead of always cloning.
  > Also add tests for all set / get functions.- **core::props::AttrValue:** add "as_*" function (#91)
  > to get the value by reference conveniently without panicing- 2.1.0- Added new `SubEventClause::Discriminant`
  > works as `SubEventClause::User`, but only checks the discriminant of the enum, instead of the whole enum. (e.g. `Foo::Bar(2)` has the same discriminant as `Foo::Bar(20)`, while when using `SubEventClause::User`, it would be different)- Rust edition 2024- async ports (#97)
  > * feat: Added support for Async Ports.
  > 
  > It is now possible to create async ports by implementing the `PollAsync` trait and then by passing the `tokio::rt::Handle` to the `EventListenerCfg::port` method.- 2.4.0- 💥 3.0.0- 3.1.0- Add `PropPayload::Any` & `State::Any` for arbitrary data exchange (#120)- 3.3.0- 💥 **props:** remove "TextSpan", introduce "Span", "Line" and "Text"- **props:** rename "{Span, Line, Text}" to "{SpanStatic, LineStatic, TextStatic}"
  > To better show that those aliases are meant for static use only.- **props::TableBuilder:** allow "add_col" to accept anything "Into<Line>"- 💥 **props:** replace title tuple with struct
  > fixes tui-realm-stdlib#45- 💥 **props:** remove "PropPayload::Tup*" variants
  > This reduces the "PropPayload" size from 256 to 64.
  > 
  > From my quick skim over cargo dependents, there are practically no consumers of the "Tup*" variants, with the only case being "tuifeed"; which arguably could be better in "Any".- 💥 **core:** change "Component::on" parameter "Event" to be a reference
  > And remove all the clones that had always been done- update MSRV to 1.86- **props:** re-add "PropPayload::Tup2"
  > As it did have some common useage in stdlib- **props:** rename "Tup2" to "Pair"- **state:** remove Tup3 and Tup4 variants
  > To better match PropPayload- **state:** rename "Tup2" to "Pair"- **props:** remove "Dataset" related values
  > As they only had a very specific use-case and was the last biggest "PropValue" variant.- **ratatui:** enable feature "layout-cache"
  > This is a default feature in ratatui and is recommended to be enabled.- **adapter::termion:** allow configuring modes- **props::any:** remove "PropBoundExt" (#149)
  > Due being able to directly implement the functions.
  > This means to use those functions, no additional trait need to be imported now.- add ability to get & downcast mounted Components- **adapter:** add termwiz as a terminal backend adapter- **adapter::termwiz:** add ability to set custom Terminal options
  > To allow Inline viewports- **adapter::crossterm:** add ability to set custom Terminal options
  > To allow Inline viewports- **adapter::termion:** add ability to set custom Terminal options
  > To allow Inline viewports- **listener:** change "Poll" to have its own Error & Result type
  > Separate from "ListenerError"/"Result".- **listener:** move poll errors to separate type- **listener::PortError:** add ability to pass context to errors
  > And have a way to stop sync ports on permanent errors.- **listener:** remove panic on "poll_timeout(Duration::ZERO)"
  > Due to https://github.com/rust-lang/rust/issues/39364 being fixed and not resulting in a panic anymore- change poll timeout to be on "PollStrategy"- **adapter::crossterm:** add panic handling for all combinations- **adapter::crossterm:** dont enable toggle "Mouse" support in "*_alternate_screen" methods- **adapter::crossterm:** automatically call "restore" on "Drop"
  > To match other backends- **terminal:** remove "TerminalBridge"- **application:** replace "PollStrategy::UpTo" with "PollStrategy::UpToNoWait"- **props:** rename "Alignment" to "AlignmentHorizontal"- **props:** introduce "AlignmentVertical"- update MSRV to 1.86- remove "Update" trait- tuirealm 2.x- Rust 2024; tuirealm 3- Add automatic scrolling for `Input` component (`InputStates`) (#37)- 3.1.0- update all components for the tuirealm TextSpan change
  > This in turn also updates all span/line/text handling to not overwrite styles.- **list:** change to take "Vec<Line>" instead of "Table"
  > As it does not make sense for the list to have columns (or more than 1).
  > 
  > This also fixes the example again which was broken in 8aa75d90c560f6dba333248a8719a9639ff5a988.- **chart:** copy over core's Dataset and make use of "PropPayload::Any"- **chart::ChartDataset:** use "Line" for name
  > Allows styling the name differently- **chart:** allow convenience attr without vec
  > For use with a single dataset- 💥 **utils:** remove "get_title_or_center"
  > As commonly no title means nothing should be rendered.
  > Also the output basically only put into "get_block", which allows Options.
  > Finally, if a empty string is added as a title to a block, it is *always* drawn, even if that border side is not specified to be drawn.- update all components for the tuirealm Title change- update all components for "Component::on" and "Tup*" changes- update for core "Dataset" removal- update MSRV to 1.86- update all components for ratatui 0.30- change "highlighted_str" to take "Line" instead of "String"- **line_gauge:** allow customizing Filled & Unfilled Symbol & Style- apply change of "State" and "PropPayload" "::One" to "::Single" rename- apply changes from "Alignment" to "HorizontalAlignment" rename- rename "alignment" functions to "alignment_horizontal"- **prop_ext:** introduce "CommonProps" Props helper- rename "ProgressBar" to "Gauge"
  > To align with ratatui widget naming.- tuirealm 2.x- add field-level attribute
  > Also move some code to own functions.
  > And add new example of how to use the attributes- support Tuple structs- support tuple struct field-level attribute- tuirealm 2.x- **widget::TreeWidget::highlight_symbol:** take a "&str" instead of a "String"- **TreeView:** change string-taking function to take "Into<String>"
  > instead of taking "AsRef<str>" and then converting to String- now selected line rendered in the middle of an area- 3.0.0- tuirealm 2.x- Set margin of layout- **textarea:** update for tuirealm 4.0- **treeview:** update for tuirealm 4.0- update MSRV to 1.88
  > As some nested dependency updated and requires it (darling), but was somehow not made a breaking change.- Remove feature `clipboard` from textarea (#188)
  > * docs: apps using tui-realm - opencode-kanban- add TestTerminalAdapter and refactor TerminalAdapter into a trait
  > Replace the monolithic TerminalBridge with a TerminalAdapter<B> trait
  > and per-backend adapter structs (Crossterm, Termion, Termwiz, Test).
  > The new TestTerminalAdapter wraps ratatui's TestBackend for integration
  > testing without requiring a real terminal.
  > 
  > Also fixes enter_alternate_screen error mapping in termwiz adapter
  > and updates all examples to use the trait's draw() method directly.- **tuirealm::PropValue:** add missing "*_color" and "*_table" functions- **tuirealm::Props:** remove "get" and "get_or" functions- **stdlib::Input:** change functions that take "InputType" to take a reference where possible- **tuirealm::AttrValue:** change "as_borders" to not returns a reference
  > As it is a cheap Copy type- **tuirealm::Props:** rename "get_ref" to just "get"- **tuirealm:** add TestEventListener for integration testing- **tuirealm:** add basic "AttrValueRef" and "Prop*Ref" enums- **tuirealm:** add PartialEq impl for Ref and non-Ref types- **tuirealm:** add "From<non-refType> for refType" impls- **tuirealm:** add convenience "as_*_ref" functions- **tuirealm:** add "From<refType> for non-refType" impls- **tuirealm:** change reftype "as_*" function to consume self
  > As "Self" is "Copy", this makes it easier to chain with later "QueryResult::as_ref"- **tuirealm:** add "QueryResult" type- **tuirealm::Component:** change "::query" to return "QueryResult"- **tuirealm::CmdResult:** add variant "Visual"- **tuirealm::Event:** change some "as_*" function to be public
  > For the most commonly that would be unwrapped as a specific one.- add "Attribute::AlwaysActive"
  > And implement it for "CommonProps"- **stdlib:** make use of "Attribute::AlwaysActive"- **stdlib::utils:** add function to wrap ratatui Lines
  > Without trimming, First-fit approach.- **stdlib::TextArea:** change to use "AttrValue::Text" / ratatui Text
  > Instead of using spans for lines.- **tuirealm::CmdResult:** rename "None" to "NoChange"- **tuirealm::Attribute:** rename "FocusStyle" to "UnfocusedBorderStyle"- **stdlib::CommonProps:** change "get" to return a reference
  > And de-duplicate getting code- **treeview:** add "::style" builder function
  > Like any other std component- change "Attribute::HighlightColor" to "Attribute::HighlightStyle"- rename "highlighted_*" function to "highlight_*"- **stdlib::Input::placeholder:** change to take a "Line"
  > Instead of string + style.
  > Also fixes the issue of the placeholder style to be applied to the whole area (like borders).- **tuirealm::AttrValue:** add variant "Marker"- **stdlib::Canvas:** make use of "AttrValue::Marker"- **tuirealm::testing:** change to use "Size" struct over width + height
  > As this makes it more obvious and uses less parameters

### Attribute

- :Disabled

### CI

- mirror to codeberg- coverage

### Changed

- **adapter::termion:** use the correct ratatui path- **component:** remove need for extra trait for "as_any" functions- **listener::ListenerMsg:** remove extraneous "Tick" variant
  > And instead use "Event::Tick"- **chart:** move into module directory- **chart:** move "as_tuidataset" function to be on "ChartDataset"- **paragraph:** switch to use "CommonProps"
  > Also rename "wrap" to "wrap_trim" to make it easier to understand.- **textarea:** switch to use "CommonProps"- **span:** switch to use "CommonProps"- **label:** switch to use "CommonProps"- **chart:** switch to use "CommonProps"- **bar_chart:** switch to use "CommonProps"- **canvas:** switch to use "CommonProps"- **checkbox:** switch to use "CommonProps"- **container:** switch to use "CommonProps"- **input:** switch to use "CommonProps"- **line_gauge:** switch to use "CommonProps"- **list:** switch to use "CommonProps"- **progress_bar:** switch to use "CommonProps"- **radio:** switch to use "CommonProps"- **select:** switch to use "CommonProps"- **sparkline:** switch to use "CommonProps"- **spinner:** switch to use "CommonProps"- **table:** switch to use "CommonProps"- **TreeView::get_block:** take a "&str" and no option
  > this function is only called once, which is already defaulted, so no need to take a option.- **TreeView::view:** dont clone "hg_str"- **tree_state::TreeState::move_down:** clone less in loop- remove root-level re-exports, expose modules only (#168)
  > Remove wildcard re-exports from tuirealm lib.rs and stdlib lib.rs.
  > All crates now use module-qualified import paths
  > (e.g. tuirealm::component::MockComponent instead of tuirealm::MockComponent).
  > Make component, state, view modules public in core.
  > Make components module public in stdlib.
  > Make tree_state, widget modules public in treeview.
  > Update derive macro generated code to use module paths.
  > Update all examples and doctests.- 💥 rename Component to AppComponent, MockComponent to Component (#161)
  > Two-pass rename to avoid collisions:
  > - Pass 1: Component<Msg, UserEvent> → AppComponent<Msg, UserEvent>
  > - Pass 2: MockComponent → Component
  > 
  > Also renames derive macro from #[derive(MockComponent)] to #[derive(Component)].
  > Updated all crates, examples, doctests, and documentation.- **tuirealm:** move "AttrValue" to its own file- **tuirealm:** rename module "value" to "prop_value"- **tuirealm:** move "Props" to its own file- **treeview:** make use of "CommonProps"- **stdlib::Select:** remove draw logic duplication- **stdlib::CommonProps::get_block:** use "is_active" over same code

### Cmd

- :Cancel

### CmdResult

- :Batch

### Documentation

- **get-started:** improve readability and fix some typos (#61)
  > * docs(get-started): typo "Wehere"
  > 
  > * docs(get-started): improve wording "you'll have may noticed"
  > 
  > * docs(get-started): improve wording "until you will umount"
  > 
  > * docs(get-started): improve wording "This is done thankfully to the"
  > 
  > * docs(get-started): remove "will" from "which will may"- **README:** fix badges (#62)- csvs- **get-started:** replace tui-rs with ratatui- systemd-manager-tui- Readme- **README:** add "quetty" as "app that uses tui-realm" (#124)- translate docs into Simplified Chinese (#128)- **migrating-4.0:** add file- remove "PartialOrd" bound
  > Originally removed in #111 / 608a14ab797c3aa1dd87ef4c0dd05a272251b5c2- **README:** extend & rewrite for 4.0- **README:** add "tuirealm-orx-tree" as a community component- **get-started:** update to read better & to align with current version
  > A lot of code examples were outdated or were bigger than they need to be.- **get-started:** replace lifecycle image with mermaid- **advanced:** update to read better & to align with current version- move language selector after first heading
  > To resolve MD041- duplicate lib.rs text to README- ignore code snippit mentioning "extern crate"
  > Due to builds on 1.85 failing with "an `extern crate` loading macros must be at the crate root"- release- **tuilream/README:** fix dead link due to monorepo merge- fix references to "veeso/tuirealm_derive" repo- fix references to "veeso/tui-realm-textarea" repo- fix references to "veeso/tui-realm-treeview" repo- fix references to "veeso/tui-realm-stdlib" repo- fix references to "veeso/tui-realm" repo (old crate as root)- add export cleanup and Component rename to 4.0 migration guide- fix more places that were missed for the "MockComponent" -> "Component" rename- **migrating-4.0:** remove "CommonProps::get" entry
  > As "CommonProps" is new to 4.0- **tuirealm:** fix stale API in guides, sync zh-cn translations

### Fixed

- duplicated events with ratatui on Windows- windows- fmt- lint- copy- serde- 1.9.2- readme- ci- docs- docs- docs- ci- ci- docs- changelog- deps- use AtomicBool instead of RwLock- new apps- manifest- docs- subclause_and_not!- macros were not usable from external crates since the `tuirealm::` namespace of the recursive macro was not specified- 2.4.1- **border:** add derive "Copy" (#105)- Fix `subclause_and_not!` macro, which was creating a `not(AndMany...)` instead of `AndMany(not(...), not(...), ...)`.- **view:** preserve focus in active, if the id is the same as old (#119)- **listener::stop:** dont wait until the sync worker has stopped
  > To fix deadlock with new test barrier- **listener::poll*:** return "ListenerDied" on "Disconnected" errors
  > Instead of "PollFailed"- **event_listener::termion:** change to have the Termion Input Listener Port be non-blocking- **event_listener::termion:** partially implement Mouse Event translation- **terminal:** fix accidentally not publicly exporting "TermwizTerminalAdapter"- **examples/demo:** add keybindings help text
  > And refactor to not recommend using "assert" everywhere- **lib:** dont make "macros" module public
  > Macros in rust are always exported on the top-level, so our "macros" module was practically empty- select border type is not applied- 1.3.1- fmt- compatibility with ratatui 0.26 (#21)- 1.3.2- manifest- readme£- changelog- Fix Styling inconsistencies (#44)- Fixup to help builds (#42)- **input:** limit cursor positon to input area (#36)- Manual Spinner stepping (#39)
  > * feat(spinner): add option to disable auto-step in view
  > 
  > * chore(examples/spinner): change one spinner to make use of manual stepping- **chart:** ensure the curser is always reset when "states.data" is overwritten- **chart:** have actually working "Marker" mappings- add missing "modifiers" function
  > To components which did not have it yet- **utils::get_block:** dont have the fallback inactive style use "Reset"- changelog- ci- Fix: Added support for structs with lifetime and generics; It is now possible to specify the component to apply the `#derive(MockComponent)` to, using the `#[component("ComponentName")]` attribute.- change the error message to what is actually expected
  > As, at least currently, "component = \"\" does not work and required "component(\"\")".- support "component = \"field\"" syntax- LayoutDirection::Horizontal needs to be accounted for in render_node- docs- manifest- changelog- **tree_state::get_last_open_heir:** should not panic if the node is open, but does not have any children- return CmdResult::Changed from Textarea::perform when state changes
  > Textarea components always returned CmdResult::None from perform(),
  > preventing callers from detecting state changes. Now the stdlib Textarea
  > returns Changed with the scroll index when navigation moves it, and the
  > editable TextArea returns Changed with the lines content when text is
  > modified.- avoid redundant allocation in Textarea::perform and add tests
  > Eliminate the second Vec<String> collection when comparing lines after
  > a command by comparing the owned snapshot directly against the slice.
  > Add unit tests covering CmdResult variants for all perform() branches.- address PR #197 review comments
  > - Move test utils (buffer_to_string, render_to_string) into
  >   tuirealm::testing module to de-duplicate across crates
  > - Fix undo/redo test: assert CmdResult::Changed for both operations
  > - Add paste and paste_multiline tests for textarea component
  > - Rename snapshot test from default to singleline
  > - Convert phantom and spinner snapshots to inline assertions
  > - Fix gauge label bug: only apply .label() when explicitly set,
  >   letting ratatui show its default percentage display otherwise- return "CmdResult::Invalid" when invalid everywhere
  > Instead of "CmdResult::None".- **textarea:** return "CmdResult::Visual" for movement without line change- **textarea:** remove accidentally added leftover function
  > They were from a previous implementation, but forgotten to be removed again.- **stdlib:** only apply highlight style if active- **stdlib::Input:** display placeholder, regardless of where the cursor is
  > Fixes a potential issue of the cursor being out-of-bounds of the typed text, hence having a empty display text, but there is actual text, and the placeholder would show.

### Input

- Escape input if is KEY + (ALT or CTRL), but NOT if is CTRL+ALT+KEY or CTRL+ALT+SHIFT+KEY

### Miscellaneous

- 3.2.0- sort deps- hasezoey added to project owners- **workflows/*:** reduce test files; share some work- **workflows/tests:** use caching- **workflows/tests:** try to use parallel builds- **workflows/tests:** disable termion tests on windows- **workflows/tests:** pre-install llvm-cov required components- **workflows/tests:** dont set "cfg(coverage)"
  > As we dont need it- **example/demo::clock:** note that the time is UTC- **example/demo::counter:** correctly handle initial value
  > And other "attr" and "query" for counter value- dont recommend "clear_screen" after "leave_alternate_screen" (#147)- **examples:** add "event_display" debug example
  > To display events recieved and how they are translated.- **examples/inline_display:** add example to showcase tui-realm can be used inline too- **workflows/codeberg-mirror:** only run on main repository (#166)- add .claude/ and .idea/ to gitignore- move tuirealm files to crates/tuirealm- email- lint- Added hasezoey to project authors- **workflows/*:** combine workflows (#47)
  > * chore(workflows/*): combine workflows
  > 
  > re tui-realm(core)
  > https://github.com/veeso/tui-realm/pull/136
  > 
  > * chore(workflows/tests): increase test MSRV
  > 
  > Due to lack of Cargo.lock, cargo will install the latest tui-realm, which has a higher MSRV than 3.x versions before
  > 
  > * ci: removed windows/macos runs
  > 
  > ---------- **examples/spinner:** make it easier to see changes- **Cargo.toml:** update core to highest possible without changes- **Cargo.toml:** update core to highest possible without changes- **examples:** update for "Poll"'s error updates- **Cargo.toml:** update core to highest possible without changes- **examples:** update for "PollStrategy" timeout parameter changes- **examples:** update for "TerminalBridge" removal- **Cargo.toml:** update core to highest possible without changes- **Cargo.toml:** remove "crossterm" dev-dependencies
  > unused dependency (provided by tui-realm)- **examples:** update "rand" version- **Cargo.toml:** remove unnecessary "termion" feature- **Cargo.toml:** remove unnecessary "crossterm" feature- **examples:** update for the "Update" trait removal- **examples:** add common Model struct- **examples:** make use of common Model struct- move stdlib files to crates/tuirealm-stdlib- **workflows:** replace "actions-rs/toolchain" with "dtolnay/rust-toolchain"- explicitly set MSRV and test it- change MSRV to 1.85.1 to match tui-realm 3.1.0- move derive files to crates/tuirealm-derive- fmt and conduct- rustfmt.toml- move treeview files to crates/tuirealm-treeview- move textarea files to crates/tuirealm-textarea- enable cargo workspace support- **README:** add basic monorepo introduction- add basic root gitignore- sort ".github" files, except workflows- **Cargo.lock:** add file- sort workflows- share core with stdlib- share core & stdlib with derive- **textarea & derive:** upgrade rust edition to "2024"- **treeview:** rename example "demo" to "filesystem"
  > To avoid name collision with core example- add CLAUDE.md and git-cliff config- remove stale plans/specs from .claude/- Slight cleanup (#187)
  > * docs: apps using tui-realm - opencode-kanban- funding- move root files to root and delete duplicates- **CONTRIBUTING:** add "Commit Messages" section- move rustfmt files to root and delete duplicates- add insta dev-dependency for snapshot testing (#191)- remove .claude/ from tracking and add to .gitignore- Do not track Cargo.lock- **tuirealm:** add "insta" as a dev-dependency
  > Due to doc-tests requiring it- **examples:** rename "Msg::None" to "Msg::Redraw" to better indicate what it does- **examples:** change "Model::update" function to not take option and dont return messages
  > As nothing makes use of it.- **examples:** remove unnecessary "let _ ="- **stdlib::example:** fix typo- **textarea::examples:** sort event handling into categories
  > For better overview- **textarea::examples:** handle window resize- **textarea::examples:** perform "TEXTAREA_CMD_MOVE_(BOTTOM|TOP)"- **examples:** make use of "Event::as_keyboard"
  > To reduce cognitive load and make it easier to write.- **stdlib::examples:** enable ticks
  > So that spinner actually works again- **stdlib::examples:** add "inactive" style
  > To easily differentiate what component is currently selected.

### Performance

- Smoother ProgressBars (#41)
  > * fix(progress_bar): enable Gauge option "use_unicode"
  > 
  > * chore(examples/progress_bar): showcase use_unicode functionality
  > 
  > By having one bar load fast and one slowly.
  > Also add text for keyboard shortcuts.
  > 
  > ---------- **textarea:** only compute margin area when there is a block

### README

- todotui

### Testing

- **application:** change test to be be less flakey- **application:** change another flaky test
  > Re macos arm tests- **application:** increase wait time
  > Re macos arm tests- **listener:** add test barrier
  > For tests to consistently wait until events are ready and not more than expected.- **application::should_do_tick:** test "TryFor" actually taking listed time- **application:** apply testing barrier for consistent events
  > Also use better PollStrategies where not explicitly testing those.
  > 
  > This also reverts 9370658d17eb090dbb8b41764ab0288d2812d976 and 8fd21b0a6d46a14771441f8e0625f2c1dbd75d53- **application:** barrier more tick dependant tests- **application:** de-duplicate some test code- **worker:** test that poll skips sending a message if Port returns "None"
  > To try to reduce coverage variance between runs.- **worker:** test that "run" exists if "poll" fails
  > To try to reduce coverage variance between runs.- enable doc-tests
  > As procmacros are otherwise hard to test.
  > This provides at least a basic baseline of "not breaking things".- add shared test helpers and phantom component tests (#191)- add label, paragraph, span component tests (#191)- add gauge, line_gauge, sparkline component tests (#191)- add input component tests (#191)- add radio and checkbox component tests (#191)- add list and table component tests (#191)- add select component tests (#191)- add bar_chart and chart component tests (#191)- add spinner and canvas component tests (#191)- add container and textarea (stdlib) component tests (#191)- add textarea crate component tests (#191)- add treeview component tests (#191)- **tuirealm::io_err_to_port_err:** add tests- **textarea:** add tests for paragraph movement- **derive:** add basic tests- **stdlib::input:** add snapshot test for no borders
  > re stdlib#62- remove need for "common.rs" file
  > As all the functionality is now included in core

### TextSpan

- :From takes AsRef<str>

### UserEvents

- remove unnecessary bounds and consistent generic names (#111)
  > * feat: remove "PartialOrd" bound for UserEvents
  > 
  > as that functionality is seemingly never used.
  > 
  > * fix(poll): add missing "Send" bound
  > 
  > * style(view): rename generic "K" to "ComponentId"
  > 
  > * style: consistent UserEvent generic names
  > 
  > * feat: remove bounds from Input Listeners where not necessary
  > 
  > they dont actually store events.
  > 
  > * style(application): rename generic "K" to "ComponentId"
  > 
  > * style(subscription): fix typo "where" to "when"
  > 
  > also clarify why forwarding may be necessary.
  > 
  > * style(subscription): consistent generic "ComponentId" name

### Build

- **ratatui:** upgrade to 0.30- **deps:** tui-textarea 0.7

### Deps

- **syn:** update to "2.0"- **tuirealm:** update to 2.1.0- upgrade to tuirealm 3.x

### Revert

- do not track Cargo.lock

### Style

- **core::props::value::PropValue:** consistent docs- improve documentation for async ports (#108)- **terminal::adapter:** link to ratatui function documentation- **props:** fix doc links for "Span", "Line" and "Text"- **application:** fix doc link for "restart_listener"- **event_listener::crossterm_async:** fix documentation linking to sync crossterm- **event_listener::termwiz:** add backend
  > I am not entirely confidend i have the mapping correct- **event::WindowResize:** document what each value in the tuple is- **listener:** clean-up "Poll" and "PollAsync" docs- **application::PollStrategy:** fix typo- **application::PollStrategy:** update and extend documentation- **adapter:** add doc-comments for "raw" and "raw_mut" functions- **adapter:** add basic "Panic handler" documentation- **adapter:** note termion and termwiz automatically restore, but eat panic messages- **adapter:** change "Panic handler" sections to be "Restore" & "On Panic"- **adapter::TerminalAdapter:** add "Expectations" sections for trait- resolve some "cargo doc" warnings- **lib:** reword some of the introduction text- remove heading in module documentation
  > As that messes with "cargo doc"'s short descriptions and is mostly redundant- update doc-comments to be easier to read
  > And actually be correct- lint- **utils:** remove stray backtick (#38)- clarify relationship between "Label", "Span", "Paragraph" and "Textarea"- **phantom:** update docs to be on the struct itself- **utils:** update documentation to not container extra headers- **chart:** update documentation to not container extra headers- **bar_chart:** update documentation to not container extra headers- **canvas:** update documentation to not container extra headers- **checkbox:** update documentation to not container extra headers- **container:** update documentation to not container extra headers- **input:** update documentation to not container extra headers- **line_gauge:** update documentation to not container extra headers- **list:** update documentation to not container extra headers- **radio:** update documentation to not container extra headers- **select:** update documentation to not container extra headers- **sparkline:** update documentation to not container extra headers- **spinner:** update documentation to not container extra headers- **table:** update documentation to not container extra headers- **textarea:** update documentation to not container extra headers- update documentation to not container extra headers- **prop_ext:** add module documentation- Lint- add "automatically_derived" attribute- update unsupported struct type error- apply "clippy --fix"- format- lint- **lib:** fix table not being displayed correctly- clippy auto-fix style for rust 1.88 upgrade- de-dent "view" example code- change big Display if to "matches"- apply common rustfmt config- remove redundant gitignores- **tuirealm::CmdResult:** enhance documentation- update TODOs- **stdlib::examples::bar_chart:** remove "disabled" calls
  > As they are not necessary

### Utils

- :get_block must be public
