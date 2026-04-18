# 高级概念

<a href="../en/advanced.md">English</a> | 📍<u>**简体中文**</u>

- [高级概念](#高级概念)
  - [简介](#简介)
  - [订阅 (Subscriptions)](#订阅-subscriptions)
    - [处理订阅](#处理订阅)
    - [事件子句 (Event clauses) 详解](#事件子句-event-clauses-详解)
    - [订阅子句 (Sub clauses) 详解](#订阅子句-sub-clauses-详解)
    - [订阅锁](#订阅锁)
  - [Tick 事件](#tick-事件)
  - [端口 (Ports)](#端口-ports)
  - [实现新组件](#实现新组件)
    - [组件应该是什么样子](#组件应该是什么样子)
    - [定义组件属性](#定义组件属性)
    - [定义组件状态](#定义组件状态)
    - [定义 Command API](#定义-command-api)
    - [渲染组件](#渲染组件)
  - [属性注入器](#属性注入器)
  - [下一步](#下一步)

---

## 简介

本指南将介绍 `tui-realm` 的所有高级概念，这些内容未在 [入门指南](get-started.md) 中涉及。
尽管 `tui-realm` 相当简洁，但得益于本文档涵盖的这些特性，它也可以非常强大。

你将学习到：

- 如何处理 **Subscriptions**，使某些组件在特定条件下监听特定事件，即使未获得焦点。
- 如何通过 `Ports` 使用自定义事件源。
- 相比 [入门指南](get-started.md)，更详细地了解如何设计可复用的自定义组件。

---

## 订阅 (Subscriptions)

> 订阅是一个规则集，它告诉 **Application** 即使组件未处于活动状态也要将事件转发给它们。

正如我们在 `tui-realm` 基础概念中已经介绍的，*Application* 负责将事件从 *Ports* 转发到 *Components*。
默认情况下，事件仅转发给当前活动组件，但这可能带来一些麻烦：

- 首先，我们可能需要一个组件始终监听特定的传入事件。设想某些 *Components* 需要从远程服务器获取数据。
  它们不能仅在获得焦点时更新，很可能需要在每次 *Port* 产生事件并被 *Event listener* 接收时都更新。
  没有 *Subscriptions*，这很难实现。
- 有时这只是重复和范围问题：在 [入门指南](get-started.md#我们的第一个应用) 的示例中我们有两个 counter，
  它们都在监听 `<ESC>` 键以退出应用，返回 `AppClose` 消息。
  但告诉应用是否应该终止真的是它们的职责吗？
  毕竟它们只是计数器，所以它们不应该知道是否关闭应用对吧？
  除此之外，为每个组件编写 `<ESC>` 匹配以返回 `AppClose` 也非常麻烦。
  有一个始终监听 `<ESC>` 并返回 `AppClose` 的不可见组件会舒服得多。

那么订阅究竟是什么，我们如何创建它？

订阅定义为：

```rust
pub struct Sub<ComponentId, UserEvent>(EventClause<UserEvent>, Arc<SubClause<ComponentId>>)
where
    ComponentId: Eq + PartialEq + Clone + Hash,
    UserEvent: Eq + PartialEq + Clone;
```

它接受 2 个参数：

- `EventClause<UserEvent>`：**事件子句**，决定要转发哪种类型的事件。这实际上直接镜像了 `Event` 的变体。
- `SubClause<ComponentId>`：实际的 *规则集*，决定 *何时* 将给定事件转发给目标 Component。

**`SubClause`** 有许多变体以允许广泛的可能性，例如它有 `Always`，将总是转发事件，没有特定规则；它有 `IsMounted`，与 `Always` 类似，但仅在该 **Id** 挂载有组件时才转发；最后还有 `And`、`Or` 和 `Not` 这样的逻辑组合器。还有一些其他应该见名知意的变体。我们会在后文详细介绍。

所以当接收到事件时，如果一个 **非活动** 的组件同时满足 *event clause* 和 *sub clause*，则该事件也将转发给它。

> ❗ 要转发事件，必须同时满足 `EventClause` 和 `SubClause`

注意，如果一个组件是活动的，它不会因同时获得焦点和订阅而收到两次该事件。

让我们详细看看如何处理订阅以及如何使用子句。

### 处理订阅

你可以在组件挂载时创建订阅，也可以在之后动态创建。

要在 `mount` 时订阅组件，向 `mount()` 提供 `Sub` 的向量即可：

```rust
app.mount(
    Id::Clock,
    Box::new(
        Clock::new(SystemTime::now())
            .alignment(HorizontalAlignment::Center)
    ),
    vec![Sub::new(EventClause::Tick, SubClause::Always)]
);
```

也可以随时创建新订阅：

```rust
app.subscribe(&Id::Clock, Sub::new(EventClause::Tick, SubClause::Always));
```

如果需要删除订阅，可以简单地取消订阅：

```rust
app.unsubscribe(&Id::Clock, EventClause::Tick);
```

> ❗ 如果一个 `EventClause` 有多个规则，`unsubscribe` 会删除 *所有* 匹配该子句的订阅。

### 事件子句 (Event clauses) 详解

Event clauses 用于定义订阅应针对哪种事件生效。
一旦 application 检查是否转发事件，它必须首先检查 event clause 并验证它是否满足与传入事件的条件。事件子句包括：

- `Any`：该事件子句总被满足，不论事件类型是什么。之后一切取决于 `SubClause` 的结果。
- `Keyboard(KeyEvent)`：要满足该子句，传入事件必须是 `Keyboard` 类型，并且 `KeyEvent` 必须完全一致。
- `WindowResize`：要满足该子句，传入事件必须是 `WindowResize` 类型，不论窗口尺寸为何。
- `Tick`：要满足该子句，传入事件必须是 `Tick` 类型。
- `User(UserEvent)`：要满足，传入事件必须是 `User` 类型。`UserEvent` 的值必须匹配，依据该类型 `PartialEq` 的实现。
- `Discriminant(UserEvent)`：要满足，传入事件必须是 `User` 类型。然后 `UserEvent` 的 `std::mem::discriminant` 必须匹配。

> ❗ `Discriminant` 只检查 `UserEvent` 的顶层变体，意思是像 `UserEvent::VariantA(OtherEnum::A)` 与 `UserEvent::Variant(OtherEnum::B)` 这样的 **也会匹配**。如果这不是你想要的行为，使用 `User(UserEvent)` 并实现自定义 `PartialEq`。

### 订阅子句 (Sub clauses) 详解

Sub clauses 在 event clause 满足后进行校验。它们定义一些必须满足才能真正转发事件的条件。
具体来说，sub clauses 包括：

- `Always`：子句总是满足。
- `HasAttrValue(Id, Attribute, AttrValue)`：如果 **Id** 对应的 Component 具有属性 `Attribute` 且值为 `AttrValue` 则满足。
- `HasState(Id, State)`：如果 **Id** 对应的 Component 的 `State` 等于提供的 state 则满足。
- `IsMounted(Id)`：如果 **Id** 对应的 Component 已挂载到 View 中则满足。

> 如果当前订阅对应的 Component 不依赖其他 Component 已挂载，则应使用 `Always` 而非 `IsMounted(Self)`。

除了这些，还可以用逻辑表达式组合 Sub clauses：

- `Not(SubClause)`：内部子句 **不** 满足时满足 (逻辑 NOT)
- `And(SubClause, SubClause)`：两个子句都满足时满足 (逻辑 AND)
- `Or(SubClause, SubClause)`：两个子句中至少一个满足时满足 (逻辑 OR)

用 `And` 和 `Or` 你可以构造更长的表达式，注意它们会递归求值，例如：

`And(Or(A, And(B, C)), And(D, Or(E, F)))` 求值为 `(A || (B && C)) && (D && (E || F))`。

支持短路求值。例如 `Or(A, B)` 中若 `A` 求值为 `true`，则 `B` **不会** 被求值。
同样 `And(A, B)` 中若 `A` 求值为 `false`，则 `B` **不会** 被求值。

另外，为了更好地优化内存局部性，还提供了其他变体：

- `AndMany`：基本上与 `And` 相同，但允许一次处理多于 (或少于) 2 个子句 (同样支持短路)
- `OrMany`：基本上与 `Or` 相同，但允许一次处理多于 (或少于) 2 个子句 (同样支持短路)

### 订阅锁

可以暂时禁用订阅的事件转发。
为此只需调用 `Application::lock_subs()`。

想恢复事件传播时调用 `Application::unlock_subs()`。
锁定期间产生的事件 *不会* 在解锁后被转发。

---

## Tick 事件

Tick 事件是一种特殊事件，由 **Application** 以指定的间隔引发。
每次初始化 **Application** 时可以指定 tick 间隔，如下例：

```rust
let app = Application::init(
    EventListenerCfg::default()
        .tick_interval(Duration::from_secs(1)),
);
```

通过 `tick_interval()` 方法指定 tick 间隔。
每次 tick 间隔过去时，application 运行时会产生一个 `Event::Tick`，在 `tick()` 中被转发给当前活动组件以及所有订阅了 `Tick` 事件的组件。

Tick 事件的用途是基于某个间隔安排动作。例如让 spinner 一致地步进。

---

## 端口 (Ports)

Ports 基本上是 **事件生产者**，由 application 的 *Event listener* 处理。
简单的 `tui-realm` 应用只会消费核心提供的事件，但如果我们需要 *更多* 事件呢？

比如我们可能需要一个 worker 从远程服务器拉取数据。你不希望 TUI 在数据拉取完成前阻塞 (不处理任何事件) 对吧？
Ports 允许你创建产生事件的 worker，只要你正确设置了它们，model 和组件都会得到更新。

让我们看看如何设置自定义 *Port*：

1. 首先为应用定义 `UserEvent` 类型：

    ```rust
    #[derive(PartialEq, Clone)]
    pub enum UserEvent {
        GotData(Data)
        // ... 如果需要其他事件
    }

    impl Eq for UserEvent {}
    ```

2. 实现 *Port*，我把它命名为 `MyHttpClient`

    ```rust
    pub struct MyHttpClient {
        // ...
    }
    ```

    现在我们需要为 *Port* 实现 `Poll` trait。
    如果你用过 async rust，这个 trait 与 `std::future::Future` 很类似。
    Poll trait 告诉 application event listener 如何在 *port* 上轮询事件：

    ```rust
    impl Poll<UserEvent> for MyHttpClient {
        fn poll(&mut self) -> PortResult<Option<Event<UserEvent>>> {
            // ... 做一些事情 ...
            Ok(Some(Event::User(UserEvent::GotData(data))))
        }
    }
    ```

3. 将 Port 加入 Application

    ```rust
    let mut app: Application<Id, Msg, UserEvent> = Application::init(
        EventListenerCfg::default()
            /* ...其他 ports，例如 "crossterm_input_listener" */
            .port(
                Box::new(MyHttpClient::new()),
                Duration::from_millis(100),
            ),
    );
    ```

    在 event listener 构造器上可以定义任意数量的 ports。声明一个 port 时需要传入一个装有实现了 *Poll* trait 的类型的 box 以及一个间隔。
    间隔定义了每次对该 port 轮询之间的时间差。

`tui-realm` 中的 Ports 也可以被称为 `Actor Pattern`。你可以在 [Actors with Tokio](https://ryhl.io/blog/actors-with-tokio/) 博文中阅读更多关于 rust 中此模式的内容。

`tui-realm` 也支持异步 ports (目前仅通过 `tokio`)，可通过 `async-ports` feature 启用。

如果你的应用已经使用 async，建议优先使用 async ports 而非同步 ports。

---

## 实现新组件

在 tui-realm 中实现组件非常简单。这个例子会实现一个比 [入门指南](get-started.md#component) 中更复杂的 Component，需要了解 *Component* 和 *AppComponent* 的区别以及至少一点 *ratatui widgets* 的知识。

说完这些，让我们看看如何实现一个更复杂的组件。这个例子我们会实现 stdlib 中 `Radio` 组件的一个简化版。

### 组件应该是什么样子

我们需要定义的第一件事是组件的外观 (即显示的输出)。
这里组件应是一个横向列出所有选项的方框。为此我们使用 ratatui widget `Tabs`。

接下来定义组件的交互：
我们希望用户能左右移动来选择选项，并能提交当前选中的选项。

### 定义组件属性

定义好外观后，我们开始定义要暴露的组件属性：

- `Foreground(Color)`：定义组件前景色
- `Background(Color)`：定义组件背景色
- `HighlightedColor(Color)`：定义高亮选项的背景色
- `Borders(Borders)`：定义组件的边框属性
- `Content(Payload(Vec(String)))`：定义 radio 组的可选项
- `Title(Title)`：定义方框标题
- `Value(Payload(Single(Usize)))`：作为属性工作，但也会更新当前选中项的状态。

```rust
pub struct Radio {
    props: Props,
    states: RadioState,
}

impl Radio {
    // 构造函数...

    pub fn foreground(mut self, fg: Color) -> Self {
        self.attr(Attribute::Foreground, AttrValue::Color(fg));
        self
    }

    // 其他属性的 builder 函数...
}

impl Component for Radio {
    // ...

    fn query<'a>(&'a self, attr: Attribute) -> Option<QueryResult<'a>> {
        self.props.get_for_query(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        match attr {
            Attribute::Content => {
                // 覆盖 choices，如果可能则保留索引
                let choices: Vec<String> = value
                    .unwrap_payload()
                    .unwrap_vec()
                    .into_iter()
                    .map(|x| x.unwrap_str())
                    .collect();
                self.states.set_choices(choices);
            }
            Attribute::Value => {
                let index = value.unwrap_payload().unwrap_single().unwrap_usize();
                self.states.select(index);
            }
            attr => {
                self.props.set(attr, value);
            }
        }
    }

    // ...
}
```

### 定义组件状态

由于该组件是交互式的，用户必须能选择某个选项，所以我们要实现一些状态。
组件状态必须跟踪当前选中项。为方便起见，也把可用选项当作状态存储。

```rust
struct RadioState {
    choice: usize,        // 选中的选项
    choices: Vec<String>, // 可用选项
}

impl RadioState {
    /// 将 choice 索引向前移动
    pub fn next_choice(&mut self) {
        if self.choice + 1 < self.choices.len() {
            self.choice += 1;
        }
    }

    /// 将 choice 索引向后移动
    pub fn prev_choice(&mut self) {
        if self.choice > 0 {
            self.choice -= 1;
        }
    }

    /// 选择指定索引
    pub fn select(&mut self, i: usize) {
        if i < self.choices.len() {
            self.choice = i;
        }
    }

    /// 从字符串向量设置 RadioState 的 choices。
    /// 此外重置当前选择，如果可能则保留索引，否则设为第一个可用值
    pub fn set_choices<S: Into<Vec<String>>>(&mut self, spans: S) {
        self.choices = spans.into();
        // 如果可能则保留索引
        if self.choice >= self.choices.len() {
            self.choice = self.choices.len().saturating_sub(1);
        }
    }
}
```

然后我们定义 `Component` 的 `state()` 方法：

```rust
impl Component for Radio {
    // ...

    fn state(&self) -> State {
        State::Single(StateValue::Usize(self.states.choice))
    }

    // ...
}
```

### 定义 Command API

定义好组件状态后，可以开始考虑 Command API。Command API 定义了组件对传入命令的响应行为及返回结果类型。

对于此组件，我们处理以下命令：

- 用户右移时，当前 choice 递增
- 用户左移时，当前 choice 递减
- 用户提交时，返回当前 choice

```rust
impl Component for Radio {
    // ...

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        match cmd {
            Cmd::Move(Direction::Right) => {
                // 递增 choice
                self.states.next_choice();
                // 改变时返回 CmdResult
                CmdResult::Changed(self.state())
            }
            Cmd::Move(Direction::Left) => {
                // 递减 choice
                self.states.prev_choice();
                // 改变时返回 CmdResult
                CmdResult::Changed(self.state())
            }
            Cmd::Submit => {
                // 返回 Submit
                CmdResult::Submit(self.state())
            }
            _ => CmdResult::Invalid(cmd),
        }
    }

    // ...
}
```

### 渲染组件

最后我们实现 `Component` 的 `view()` 方法用于渲染组件：

```rust
impl Component for Radio {
    fn view(&mut self, render: &mut Frame, area: Rect) {
        if matches!(self.props.get(Attribute::Display), Some(AttrValue::Flag(false))) {
            return;
        }

        // 构造 choices
        let choices: Vec<Line> = self
            .states
            .choices
            .iter()
            .map(|x| Line::from(x.as_str()))
            .collect();

        // 获取其他样式属性
        let foreground = self
            .props
            .get(Attribute::Foreground)
            .and_then(AttrValue::as_color)
            .unwrap_or(Color::Reset);
        let background = self
            .props
            .get(Attribute::Background)
            .and_then(AttrValue::as_color)
            .unwrap_or(Color::Reset);
        let highlight_bg = self
            .props
            .get(Attribute::HighlightedColor)
            .and_then(AttrValue::as_color)
            .unwrap_or(Color::Reset);

        let normal_style = Style::default()
            .fg(foreground)
            .bg(background);

        let highlight_style = normal_style.patch(Style::default().bg(highlight_bg));

        // 组装 Block (边框)
        let borders = self
            .props
            .get(Attribute::Borders)
            .and_then(AttrValue::as_borders)
            .unwrap_or_default();
        let title = self
            .props
            .get(Attribute::Title)
            .and_then(AttrValue::as_title)
            .cloned()
            .unwrap_or_default();
        let focus = self
            .props
            .get(Attribute::Focus)
            .and_then(AttrValue::as_flag)
            .unwrap_or(false);

        let block = Block::default()
            .title_top(title.content)
            .borders(borders.sides)
            .border_style(if focus {
                borders.style()
            } else {
                Style::default().fg(Color::DarkGray)
            });

        // 最后用 ratatui widget 绘制内容
        let tabs = Tabs::new(choices)
            .block(block)
            .select(self.states.choice)
            .style(normal_style)
            .highlight_style(highlight_style);
        render.render_widget(tabs, area);
    }

    // ...
}
```

---

## 属性注入器

属性注入器是 trait 对象，必须实现 `Injector` trait，在组件挂载时提供某些属性 (定义为 `Attribute` 与 `AttrValue` 的元组)。
Injector trait 定义如下：

```rs
pub trait Injector<ComponentId>
where
    ComponentId: Eq + PartialEq + Clone + Hash,
{
    fn inject(&self, id: &ComponentId) -> Vec<(Attribute, AttrValue)>;
}
```

然后可以用 `add_injector()` 方法把注入器加入到应用中。

每当你挂载一个新组件到 view 时，会为应用中定义的每个注入器调用 `inject()` 方法，并以挂载组件的 id 作为参数。

---

## 下一步

这是 `tui-realm` 当前可用指南的结尾，但建议继续阅读：

- [`tuirealm`](https://docs.rs/tuirealm/latest/tuirealm/) 的文档
- [`tui-realm-stdlib`](https://docs.rs/tui-realm-stdlib/latest/tui_realm_stdlib/) 的文档

如果有任何问题，欢迎开带 `question` 标签的 issue，我会尽快回复 🙂。
