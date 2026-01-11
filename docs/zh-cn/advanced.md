📍<u>**简体中文**</u> | <a href="../en/advanced.md">English</a>

# 高级概念

- [高级概念](#高级概念)
  - [简介](#简介)
  - [订阅 (Subscriptions)](#订阅 (Subscriptions))
    - [处理订阅](#处理订阅)
    - [事件子句 (Event clauses) 详解](#事件子句 (Event clauses) 详解)
    - [订阅子句 (Sub clauses) 详解](#订阅子句 (Sub clauses) 详解)
    - [订阅锁](#订阅锁)
  - [Tick 事件](#tick-事件)
  - [端口](#端口)
  - [实现新组件](#实现新组件)
    - [组件应该是什么样子](#组件应该是什么样子)
    - [定义组件属性](#定义组件属性)
    - [定义组件状态](#定义组件状态)
    - [定义 Cmd API](#定义-cmd-api)
    - [渲染组件](#渲染组件)
  - [属性注入器](#属性注入器)
  - [下一步](#下一步)

---

## 简介

本指南将向您介绍 tui-realm 的所有高级概念，这些概念在[入门指南](get-started.md)中未涵盖。尽管 tui-realm 相当简单，但它也可以变得非常强大，这得益于我们将在本文档中介绍的所有这些功能。

您将学习到：

- 如何处理订阅 (Subscriptions)，使某些组件在特定情况下监听特定事件
- 什么是 `Event::Tick`
- 如何通过 `Ports` 使用自定义事件源
- 如何实现新组件

---

## 订阅 (Subscriptions)

> 订阅是一个规则集，它告诉**应用程序**基于某些规则将事件转发给其他组件，即使它们不处于活动状态。

正如我们在 tui-realm 的基本概念中已经介绍的，应用负责将事件从端口转发到组件。
默认情况下，事件仅转发给当前活动组件，但这可能相当烦人：

- 首先，我们可能需要一个组件始终监听传入事件。想象一些轮询远程服务器的加载器。它们不能仅在获得焦点时更新，它们可能需要在*事件监听器*每次接收到来自*端口*的事件时更新。没有*订阅*，这将是不可能的。
- 有时这只是"太无聊"和范围的问题：在示例中我有两个计数器，它们都监听 `<ESC>` 键来退出应用并返回 `AppClose` 消息。但是告诉应用是否应该终止是它们的责任吗？我的意思是，它们只是计数器，所以它们不应该知道是否关闭应用，对吧？除此之外，为每个组件编写 `<ESC>` 的情况来返回 `AppClose` 也非常烦人。有一个不可见的组件始终监听 `<ESC>` 来返回 `AppClose` 会舒服得多。

那么订阅实际上是什么，我们如何创建它们？

订阅定义为：

```rust
pub struct Sub<UserEvent>(EventClause<UserEvent>, SubClause)
where
    UserEvent: Eq + PartialEq + Clone;
```

所以它是一个元组结构，接受一个 `EventClause` 和一个 `SubClause`，让我们深入了解：

- **事件子句**是传入事件必须满足的匹配子句。正如我们之前所说，应用必须知道是否将某个*事件*转发给某个组件。所以它必须检查的第一件事是它是否正在监听那种事件。

    事件子句声明如下：

    ```rust
    pub enum EventClause<UserEvent>
    where
        UserEvent: Eq + PartialEq + Clone,
    {
        /// 无论何种事件都转发
        Any,
        /// 检查是否按下了某个键
        Keyboard(KeyEvent),
        /// 检查窗口是否已调整大小
        WindowResize,
        /// 在 tick 时转发事件
        Tick,
        /// 在此特定用户事件时转发事件。
        /// 用户事件的匹配方式取决于其 partialEq 实现
        User(UserEvent),
    }
    ```

- **订阅子句**是必须由与订阅关联的组件满足的附加条件，以便转发事件：

    ```rust
    pub enum SubClause {
        /// 始终将事件转发给组件
        Always,
        /// 如果目标组件具有提供的属性和提供的值，则转发事件
        /// 如果组件上不存在该属性，结果始终为 `false`。
        HasAttrValue(Attribute, AttrValue),
        /// 如果目标组件具有提供的状态，则转发事件
        HasState(State),
        /// 如果内部子句为 `false`，则转发事件
        Not(Box<SubClause>),
        /// 如果两个内部子句都为 `true`，则转发事件
        And(Box<SubClause>, Box<SubClause>),
        /// 如果至少一个内部子句为 `true`，则转发事件
        Or(Box<SubClause>, Box<SubClause>),
    }
    ```

因此，当接收到事件时，如果一个**不活动**的组件满足事件子句和订阅子句，那么事件也将转发给该组件。

> ❗ 为了转发事件，必须同时满足 `EventClause` 和 `SubClause`

让我们详细看看如何处理订阅以及如何使用子句。

### 处理订阅

您可以在组件挂载时和任何时候创建订阅。

要在 `mount` 时订阅组件，只需向 `mount()` 提供 `Sub` 向量：

```rust
app.mount(
    Id::Clock,
    Box::new(
        Clock::new(SystemTime::now())
            .alignment(Alignment::Center)
            .background(Color::Reset)
            .foreground(Color::Cyan)
            .modifiers(TextModifiers::BOLD)
    ),
    vec![Sub::new(SubEventClause::Tick, SubClause::Always)]
);
```

或者您可以在任何时候创建新订阅：

```rust
app.subscribe(&Id::Clock, Sub::new(SubEventClause::Tick, SubClause::Always));
```

如果您需要删除订阅，可以简单地取消订阅：

```rust
app.unsubscribe(&Id::Clock, SubEventClause::Tick);
```

### 事件子句 (Event clauses) 详解

事件子句用于定义应为哪种事件设置订阅。
一旦应用检查是否要转发事件，它必须首先检查事件子句并验证其是否满足与传入事件的边界。事件子句有：

- `Any`：事件子句被满足，无论是什么类型的事件。一切都取决于 `SubClause` 的结果。
- `Keyboard(KeyEvent)`：为了满足子句，传入事件必须是 `Keyboard` 类型，并且 `KeyEvent` 必须完全相同。
- `WindowResize`：为了满足子句，传入事件必须是 `WindowResize` 类型，无论窗口大小如何。
- `Tick`：为了满足子句，传入事件必须是 `Tick` 类型。
- `User(UserEvent)`：为了被满足，传入事件必须是 `User` 类型。`UserEvent` 的值必须匹配，根据为此类型实现 `PartialEq` 的方式。

### 订阅子句 (Sub clauses) 详解

订阅子句在事件子句满足后验证，它们定义了一些必须由**目标**组件（与订阅关联的组件）满足的子句。
特别是订阅子句有：

- `Always`：子句始终满足
- `HasAttrValue(Id, Attribute, AttrValue)`：如果目标组件（在 `Id` 中定义）在其 `Props` 中具有 `Attribute` 和 `AttrValue`，则满足子句。
- `HasState(Id, State)`：如果目标组件（在 `Id` 中定义）具有等于提供状态的 `State`，则满足子句。
- `IsMounted(Id)`：如果目标组件（在 `Id` 中定义）已挂载到视图中，则满足子句。

除了这些，还可以使用表达式组合订阅子句：

- `Not(SubClause)`：如果内部子句不满足，则满足子句（取反结果）
- `And(SubClause, SubClause)`：如果两个子句都满足，则满足子句
- `Or(SubClause, SubClause)`：如果至少一个子句满足，则满足子句。

使用 `And` 和 `Or`，您甚至可以创建长表达式，请记住它们是递归评估的，例如：

`And(Or(A, And(B, C)), And(D, Or(E, F)))` 被评估为 `(A || (B && C)) && (D && (E || F))`

### 订阅锁

可以暂时禁用订阅传播。
为此，您只需要调用 `application.lock_subs()`。

无论何时想要恢复事件传播，只需调用 `application.unlock_subs()`。

---

## Tick 事件

Tick 事件是一种特殊的事件，由**应用**以指定的间隔引发。
每当初始化**应用**时，您可以指定 tick 间隔，如以下示例所示：

```rust
let mut app: Application<Id, Msg, NoUserEvent> = Application::init(
    EventListenerCfg::default()
        .tick_interval(Duration::from_secs(1)),
);
```

使用 `tick_interval()` 方法，我们指定 tick 间隔。
每次 tick 间隔过去时，应用运行时将抛出一个 `Event::Tick`，它将在 `tick()` 时转发给当前活动组件和所有订阅了 `Tick` 事件的组件。

Tick 事件的目的是基于某个间隔调度操作。

---

## 端口

端口基本上是**事件生产者**，由应用*事件监听器*处理。
通常，tui-realm 应用只会消费输入事件或 tick 事件，但如果我们需要*更多*事件怎么办？

例如，我们可能需要一个工作器来获取远程服务器的数据。端口允许您创建自动化的工作器，这些工作器将产生事件，如果您正确设置了一切，您的模型和组件将被更新。

现在让我们看看如何设置*端口*：

1. 首先我们需要为我们的应用定义 `UserEvent` 类型：

    ```rust
    #[derive(PartialEq, Clone, PartialOrd)]
    pub enum UserEvent {
        GotData(Data)
        // ... 如果您需要，其他事件
    }

    impl Eq for UserEvent {}
    ```

2. 实现*端口*，我命名为 `MyHttpClient`

    ```rust
    pub struct MyHttpClient {
        // ...
    }
    ```

    现在我们需要为*端口*实现 `Poll` trait。
    Poll trait 告诉应用事件监听器如何在*端口*上轮询事件：

    ```rust
    impl Poll<UserEvent> for MyHttpClient {
        fn poll(&mut self) -> ListenerResult<Option<Event<UserEvent>>> {
            // ... 做点什么 ...
            Ok(Some(Event::User(UserEvent::GotData(data))))
        }
    }
    ```

3. 应用中的端口设置

    ```rust
    let mut app: Application<Id, Msg, UserEvent> = Application::init(
        EventListenerCfg::default()
            .default_input_listener(Duration::from_millis(10))
            .port(
                Box::new(MyHttpClient::new(/* ... */)),
                Duration::from_millis(100),
            ),
    );
    ```

    在事件监听器构造函数中，您可以定义任意数量的端口。当您声明一个端口时，您需要传递一个包含实现*Poll* trait 的类型的 box 和一个间隔。
    间隔定义了每次轮询端口之间的间隔。

---

## 实现新组件

在 tui-realm 中实现新组件实际上相当简单，但要求您至少对**tui-rs widgets**有基本的了解。

除了 tui-rs 知识，您还应该记住*MockComponent*和*Component*之间的区别，以免实现糟糕的组件。

说了这些，让我们看看如何实现一个组件。对于这个示例，我将实现 stdlib 中 `Radio` 组件的简化版本。

### 组件应该是什么样子

我们需要定义的第一件事是组件应该是什么样子。
在这种情况下，组件是一个包含选项列表的框，您可以选择一个，这是用户的选择。
用户将能够在不同的选择之间移动并提交一个。

### 定义组件属性

一旦我们定义了组件的样子，我们就可以开始定义组件属性：

- `Background(Color)`：将定义组件的背景颜色
- `Borders(Borders)`：将定义组件的边框属性
- `Foreground(Color)`：将定义组件的前景颜色
- `Content(Payload(Vec(String)))`：将定义单选组的可能选项
- `Title(Title)`：将定义框标题
- `Value(Payload(One(Usize)))`：将作为属性工作，但也会更新状态，用于当前选定的选项。

```rust
pub struct Radio {
    props: Props,
    // ...
}

impl Radio {

    // 构造函数...

    pub fn foreground(mut self, fg: Color) -> Self {
        self.attr(Attribute::Foreground, AttrValue::Color(fg));
        self
    }

    // ...
}

impl MockComponent for Radio {

    // ...

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        match attr {
            Attribute::Content => {
                // 重置选择
                let choices: Vec<String> = value
                    .unwrap_payload()
                    .unwrap_vec()
                    .iter()
                    .map(|x| x.clone().unwrap_str())
                    .collect();
                self.states.set_choices(&choices);
            }
            Attribute::Value => {
                self.states
                    .select(value.unwrap_payload().unwrap_one().unwrap_usize());
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

由于此组件可以是交互式的，并且用户必须能够选择某个选项，我们必须实现一些状态。
组件状态必须跟踪当前选定的项。出于实际原因，我们还使用可用选择作为状态。

```rust
struct OwnStates {
    choice: usize,        // 选定的选项
    choices: Vec<String>, // 可用选择
}

impl OwnStates {

    /// 将选择索引移动到下一个选择
    pub fn next_choice(&mut self) {
        if self.choice + 1 < self.choices.len() {
            self.choice += 1;
        }
    }


    /// 将选择索引移动到上一个选择
    pub fn prev_choice(&mut self) {
        if self.choice > 0 {
            self.choice -= 1;
        }
    }


    /// 从文本跨度向量设置 OwnStates 选择
    /// 此外重置当前选择，如果可能则保留索引，或将其设置为第一个可用值
    pub fn set_choices(&mut self, spans: &[String]) {
        self.choices = spans.to_vec();
        // 如果可能则保留索引
        if self.choice >= self.choices.len() {
            self.choice = match self.choices.len() {
                0 => 0,
                l => l - 1,
            };
        }
    }

    pub fn select(&mut self, i: usize) {
        if i < self.choices.len() {
            self.choice = i;
        }
    }
}
```

然后我们可以定义 `state()` 方法

```rust
impl MockComponent for Radio {

    // ...

    fn state(&self) -> State {
        State::One(StateValue::Usize(self.states.choice))
    }

    // ...

}
```

### 定义 Cmd API

一旦我们定义了组件状态，我们就可以开始考虑命令 API。命令 API 定义了组件在面对传入命令时的行为方式以及它应该返回什么类型的结果。

对于此组件，我们将处理以下命令：

- 当用户向右移动时，当前选择递增
- 当用户向左移动时，当前选择递减
- 当用户提交时，返回当前选择

```rust
impl MockComponent for Radio {

    // ...

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        match cmd {
            Cmd::Move(Direction::Right) => {
                // 递增选择
                self.states.next_choice();
                // 返回更改的 CmdResult
                CmdResult::Changed(self.state())
            }
            Cmd::Move(Direction::Left) => {
                // 递减选择
                self.states.prev_choice();
                // 返回更改的 CmdResult
                CmdResult::Changed(self.state())
            }
            Cmd::Submit => {
                // 返回提交
                CmdResult::Submit(self.state())
            }
            _ => CmdResult::None,
        }
    }

    // ...

}
```

### 渲染组件

最后，我们可以实现组件 `view()` 方法，该方法将渲染组件：

```rust
impl MockComponent for Radio {
    fn view(&mut self, render: &mut Frame, area: Rect) {
        if self.props.get_or(Attribute::Display, AttrValue::Flag(true)) == AttrValue::Flag(true) {
            // 创建选择
            let choices: Vec<Spans> = self
                .states
                .choices
                .iter()
                .map(|x| Spans::from(x.clone()))
                .collect();
            let foreground = self
                .props
                .get_or(Attribute::Foreground, AttrValue::Color(Color::Reset))
                .unwrap_color();
            let background = self
                .props
                .get_or(Attribute::Background, AttrValue::Color(Color::Reset))
                .unwrap_color();
            let borders = self
                .props
                .get_or(Attribute::Borders, AttrValue::Borders(Borders::default()))
                .unwrap_borders();
            let title = self.props.get(Attribute::Title).map(|x| x.unwrap_title());
            let focus = self
                .props
                .get_or(Attribute::Focus, AttrValue::Flag(false))
                .unwrap_flag();
            let div = crate::utils::get_block(borders, title, focus, None);
            // 创建颜色
            let (bg, fg, block_color): (Color, Color, Color) = match focus {
                true => (foreground, background, foreground),
                false => (Color::Reset, foreground, Color::Reset),
            };
            let radio: Tabs = Tabs::new(choices)
                .block(div)
                .select(self.states.choice)
                .style(Style::default().fg(block_color))
                .highlight_style(Style::default().fg(fg).bg(bg));
            render.render_widget(radio, area);
        }
    }

    // ...
}
```

---

## 属性注入器

属性注入器是 trait 对象，必须实现 `Injector` trait，可以在组件挂载时为其提供一些属性（定义为 `Attribute` 和 `AttrValue` 的元组）。
Injector trait 定义如下：

```rs
pub trait Injector<ComponentId>
where
    ComponentId: Eq + PartialEq + Clone + Hash,
{
    fn inject(&self, id: &ComponentId) -> Vec<(Attribute, AttrValue)>;
}
```

然后您可以使用 `add_injector()` 方法将注入器添加到您的应用中。

无论何时将新组件挂载到视图中，都会为应用中定义的每个注入器调用 `inject()` 方法，提供挂载组件的 id 作为参数。

---

## 下一步

如果您来自 tui-realm 0.x 并希望迁移到 tui-realm 1.x，有一个指南解释了[如何从 tui-realm 0.x 迁移到 1.x](migrating-legacy.md)。
否则，我认为您现在就可以开始实现您的 tui-realm 应用了 😉。

如果您有任何问题，请随时打开带有 `question` 标签的问题，我会尽快回答您 🙂。
