# 从 tui-realm 3.x 迁移

<a href="../en/migrating-4.0.md">English</a> | 📍<u>**简体中文**</u>

- [从 tui-realm 3.x 迁移](#从-tui-realm-3x-迁移)
  - [简介](#简介)

---

## 简介

本文档是 4.0 发布之前的进行中文档，随着改动的引入逐条列出关键变更。

### ratatui 0.30

`ratatui` 已升级到 0.30，所有破坏性变更请阅读其 [博客文章](https://ratatui.rs/highlights/v030)。

### 用 ratatui 等价类型替换 `TextSpan`

原先的 `tuirealm::props::TextSpan` 已被替换为 `ratatui::text::{Span, Line, Text}`。

由于有了新类型，引入了新的 `AttrValue` 和 `PropValue` 变体：`TextSpan`、`TextLine` 和 `Text`。

### 把 `(String, Alignment)` 标题替换为真正的结构体

原先的 `(String, Alignment)` 元组已被功能更完整的 `Title` struct 替换。

由于 title 现在底层使用 `Line`，现在可以对标题中的单个字符做样式处理。

### 移除 `PropPayload` 和 `State` 的 `Tup3`、`Tup4` 变体

`PropPayload` 和 `State` 的 3 元与 4 元元组变体被移除，因为它们会膨胀枚举大小，而实际上几乎没人使用。

如果仍需多种类型，可使用 `PropPayload::Vec`，或为更具描述性的字段使用 `PropPayload::Any` 承载自定义结构体。

### 将 `PropPayload` 和 `State` 的 `Tup2` 变体重命名为 `Pair`

既然其他元组变体都已移除，把 `Tup2` 重命名为 `Pair` 更具描述性。

### 将 `PropPayload` 和 `State` 的 `One` 变体重命名为 `Single`

由于 `Tup2` 改名为 `Pair`，并考虑到其他变体，`Single` 比 `One` 更符合命名规范。

### Attribute `Alignment` 拆分为水平与垂直

为与 ratatui `0.30` 把 `Alignment` 改名为 `HorizontalAlignment` 的变化保持一致，`tui-realm` 将原有的 Attribute `Alignment` 改名为 `AlignmentHorizontal`，并新增一个名为 `AlignmentVertical` 的属性对应 `VerticalAlignment`。

### 移除 Dataset 相关值

`Dataset` 实际上只被 `tui_realm_stdlib::components::Chart` 使用，而且即便如此也不需要存在 `Props` 中，因此可以轻松改为通过 `PropPayload::Any` 承载。

### 移除 `Props::get`(旧) 和 `Props::get_or`

`Props::get` 被移除，改用 `Props::get_ref`，以与 `Vec::get` 等标准库类型的返回类型保持一致。
这也让克隆对用户更显式。

`Props::get_or` 被移除，因为它依赖 `Props::get`，而无法合理地改造为使用 `Props::get_ref`。

### 将 `Props::get_ref` 重命名为 `Props::get`

由于旧 `Props::get` 已被移除，并为了更好地与 `Vec::get` 等标准库类型对齐，`::get_ref` 被重命名为 `::get`。

### `Component::on` 的 `Event` 参数现为引用

在 4.0 中，`Component::on` 的 `Event` 参数现在是引用。这让我们得以移除此前一直进行的中间克隆，现在是否克隆由用户决定。

### `termion` 后端 / 适配器变更

`termion` 后端适配器已重构，以更契合 `termion` 的工作方式。

实际上意味着 `new` 不再存在，取而代之的是更具体的新函数：

- `new_raw`
- `new_alternate_raw`
- `new_mouse_alternate_raw`
- `new_mouse_raw`

此外，`TerminalBridge::new_termion` 和 `init_termion` 已被移除，改用 `TerminalBridge::new_init_termion`。

### 移除 `PropBoundExt`

`as_any` 和 `as_any_mut` 现在直接在 `dyn PropBound` 上实现，不再需要额外导入另一个 trait。

### `Poll` 返回类型变更

在 4.0 中，`Poll::poll` 和 `PollAsync::poll` 的返回类型从 `ListenerResult` 改为 `PortResult`。
由此带来的新 `PortError` 可以提供更多关于发生什么的上下文。它还支持标明错误是 Intermittent (应再次轮询) 还是 Permanent (应停止该 port)。

这是因为 `ListenerError` 的变体大多是内部使用的。
`ListenerResult` 也被改为非公开。

### `poll` 使用独立的错误类型

除了 [`*Poll::poll` 返回类型变更](#poll-返回类型变更)，`ApplicationError` 新增了专门的 `Poll` 变体，不再与 Listener 启动/停止错误混用。

### 把 `PollStrategy::UpToNoWait` 改为 `PollStrategy::UpTo`

旧的 `PollStrategy::UpTo` 被移除，由原先叫 `PollStrategy::UpToNoWait` 的替代。

这样改是因为 "对每个 N 等待 TIMEOUT，如果有事件可用" 这种行为对事件驱动型 tui 应用并不实用。
(新的 `UpTo`，即以前的 `UpToNoWait`，会 "等待 TIMEOUT 一次，收集事件，之后在有事件可用时最多再收集 N-1 个事件 (不再阻塞)")

### Poll timeout 移至 `PollStrategy` 中

此前保存在 `EventListener(Cfg|Builder)` 上的 timeout 被移至 `PollStrategy`。
这样做是因为某些策略根本不使用 timeout，而另一些策略对该时长有不同的含义。

### 移除 `TerminalBridge`

`TerminalBridge` 包装器被移除，因为相比直接使用后端或直接使用 trait 它并没有带来任何好处。

Panic handler 和 restore 现在在各后端上按需实现。
各后端的具体说明见相应后端的 `Restore` 和 `On Panic` 小节。

### 移除 `Update` trait

为与 `view` 等其他 "外部" 函数保持一致，决定移除 `Update` trait，因为它从未真正作为任何地方的 bounds 被要求。

这使它与之前也没有 trait 的 `view` 等函数保持一致。
同时允许自定义 `update` 函数的调用方式，例如如果你从不返回消息用于递归处理，就可以省略。

迁移非常简单，把 `impl Update for Model` 改为 `impl Model`，并可能将可见性改为 `pub fn`。

### 导出清理：要求按模块限定导入

`tuirealm` 和 `tui-realm-stdlib` 中根级的 re-export 已被移除。现在必须通过模块路径导入类型：

```rust
// 之前 (3.x)
use tuirealm::{Application, Component, MockComponent, Event, State, Frame};

// 之后 (4.0)
use tuirealm::application::Application;
use tuirealm::component::{AppComponent, Component};
use tuirealm::event::Event;
use tuirealm::state::State;
use tuirealm::ratatui::Frame;
```

对于 `tui-realm-stdlib`，组件类型现在位于 `components` 下：

```rust
// 之前
use tui_realm_stdlib::Input;

// 之后
use tui_realm_stdlib::components::Input;
```

### `MockComponent` 重命名为 `Component`

`Component` trait (事件处理) 已重命名为 `AppComponent`。
`MockComponent` trait (渲染、state、props) 已重命名为 `Component`。
派生宏 `#[derive(MockComponent)]` 改为 `#[derive(Component)]`，以匹配新的 `Component` 名称。

```rust
// 之前 (3.x)
use tuirealm::{MockComponent, Component};

#[derive(MockComponent)]
struct MyWidget { component: Input }

impl Component<Msg, UserEvent> for MyWidget {
    fn on(&mut self, ev: Event<UserEvent>) -> Option<Msg> { ... }
}

// 之后 (4.0)
use tuirealm::component::{AppComponent, Component};  // traits

#[derive(Component)]
struct MyWidget { component: Input }

impl AppComponent<Msg, UserEvent> for MyWidget {
    fn on(&mut self, ev: &Event<UserEvent>) -> Option<Msg> { ... }
}
```

### 将 `Component::query` 改为返回借用内容

`Component::query` 被修改为允许并倾向于返回借用内容，但仍支持返回拥有所有权的内容。
这让消费者可以决定何时真正需要克隆，除 `PropPayload::Any` 外几乎任何情况都不再强制克隆。

### 将 `CmdResult::None` 重命名为 `CmdResult::NoChange`

`CmdResult` 的 `None` 变体被重命名为 `NoChange`，让该变体的含义一眼更清晰。

### 将 `Attribute::FocusStyle` 重命名为 `Attribute::UnfocusedBorderStyle`

`Attribute` 的 `FocusStyle` 变体被重命名为 `UnfocusedBorderStyle`，更能一眼看出该属性的作用。

### 将 `Attribute::HighlightColor` 重命名为 `Attribute::HighlightStyle`

`Attribute` 的 `HighlightColor` 变体被移除，取而代之添加了 `HighlightStyle`，以完整配置样式 (含 modifiers) 而不仅是颜色。

### 将 `::highlighted_*` 函数重命名为 `::highlight_*`

为命名一致并与 `ratatui` 中的函数名对齐，所有 `highlighted_*` 函数 (例如 `::highlighted_str`) 改为 `highlight_*` (例如 `::highlight_str`)。
