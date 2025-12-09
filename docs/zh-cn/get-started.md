📍<u>**简体中文**</u> | <a href="../en/get-started.md">English</a>

# 快速入门 🏁

- [快速入门 🏁](#快速入门-)
  - [Realm 简介](#realm-简介)
  - [核心概念](#核心概念)
  - [原型组件 (MockComponent) 与 组件 (Component)](#原型组件 (MockComponent) 与 组件 (Component))
    - [原型组件](#原型组件)
    - [组件](#组件)
    - [属性 (Properties) 与状态 (States)](#属性 (Properties) 与状态 (States))
    - [事件 (Events) 与命令 (Commands)](#事件 (Events) 与命令 (Commands))
  - [应用、模型和视图](#应用模型和视图)
    - [视图 (View)](#视图 (View))
      - [焦点](#焦点)
    - [模型 (Model)](#模型 (Model))
    - [应用](#应用)
  - [生命周期（或 "tick"）](#生命周期或-tick)
  - [我们的第一个应用](#我们的第一个应用)
    - [实现计数器](#实现计数器)
    - [定义消息类型](#定义消息类型)
    - [定义组件标识符](#定义组件标识符)
    - [实现两个计数器组件](#实现两个计数器组件)
    - [实现模型](#实现模型)
    - [应用设置和主循环](#应用设置和主循环)
  - [下一步](#下一步)

---

## Realm 简介

您将学习到：

- tui-realm 的核心概念
- 如何从零开始编写 tui-realm 应用
- tui-realm 的亮点特性

tui-realm 是一个 ratatui **框架**，提供了实现有状态应用的简便方法。
首先，让我们看看 tui-realm 的主要特性以及为什么在构建终端用户界面时应选择此框架：

- ⌨️ **事件驱动**

    tui-realm 采用 Elm 架构的 `Event -> Msg` 模式。**事件**由称为 `Port` 的实体 (entities) 产生，这些实体作为事件监听器（如 stdin 读取器或 HTTP 客户端）工作，产生事件。这些事件随后被转发到**组件**，组件将产生一个**消息**。消息将根据其变体在您的应用模型中引起特定行为。
    相当简单，您的应用中的所有内容都将围绕此逻辑工作，因此实现任何功能都非常容易。

- ⚛️ 基于 **React** 和 **Elm**

    tui-realm 基于 [React](https://reactjs.org/) 和 [Elm](https://elm-lang.org/)。这两种方法有些不同，但我决定从每种方法中取其精华，将它们结合到 **Realm** 中。从 React 中我采用了**组件**概念。在 realm 中，每个组件代表一个图形实例，可能包含一些子组件；每个组件都有一个**状态**和一些**属性**。
    从 Elm 中我基本上采用了 Realm 中实现的每个其他概念。我真的很喜欢 Elm 作为一门语言，特别是 **TEA**（The Elm Architecture）。
    实际上，与 Elm 一样，在 realm 中应用的生命周期是 `Event -> Msg -> Update -> View -> Event -> ...`

- 🍲 **样板代码**

    tui-realm 在开始时可能看起来很难使用，但过一段时间后您会开始意识到您正在实现的代码只是从先前组件复制的样板代码。

- 🚀 快速设置

    自从最新的 tui-realm API（1.x）以来，tui-realm 变得非常容易学习和设置，这得益于新的 `Application` 数据类型、事件监听器和 `Terminal` 辅助工具。

- 🎯 单一的**焦点**和**状态**管理

    在 realm 中，您不必自己管理焦点和状态，一切都由**视图**自动管理，所有组件都挂载在视图中。使用 realm，您不再需要担心应用状态和焦点了。

- 🙂 易于学习

    得益于向用户公开的少量数据类型和指南，即使您以前从未使用过 tui 或 Elm，学习 tui-realm 也非常容易。

- 🤖 适应任何用例

    正如您将通过本指南学到的，tui-realm 公开了一些高级概念来创建自己的事件监听器、处理自己的事件以及实现复杂的组件。

---

## 核心概念

现在让我们看看 tui-realm 的核心概念。在简介中您可能已经读到了一些**粗体**的概念，但现在让我们详细看看它们。核心概念非常重要，幸运的是它们易于理解且数量不多：

- **MockComponent**：原型组件代表一个可复用的 UI 组件，可以具有一些用于渲染或处理命令的**属性**。如果需要，它还可以有自己的**状态**。实际上，它是一个 trait，公开了一些方法来渲染和处理属性、状态和事件。我们将在下一章中详细讨论。
- **Component**：组件是 原型组件的包装器，代表您应用中的单个组件。它直接接收事件并为应用消费者生成消息。在底层，它依赖其 原型组件进行属性/状态管理和渲染。
- **State**：状态代表组件的当前状态（例如，文本输入框中的当前文本）。状态取决于用户（或其他来源）如何与组件交互（例如，用户按下 'a'，字符被推送到文本输入框）。
- **Attribute**：属性描述组件中的单个属性。属性不应依赖于组件状态，而应仅在组件初始化时由用户配置。通常，原型组件公开许多要配置的属性，而使用 原型组件 的组件根据用户需求设置它们。
- **Event**：事件是一个**原始**实体，主要描述由用户引起的事件（如按键操作），但也可能由外部源生成（我们将在"高级概念"中讨论这些）。
- **Message**（通常称为 `Msg`）：消息是由组件在接收到**事件**后生成的逻辑事件。

    事件是*原始*的（如按键操作），而消息是面向应用的。消息随后由**更新例程**消费。我认为一个例子可以更好地解释：假设我们有一个弹出窗口组件，当按下 `ESC` 时，它必须报告给应用以隐藏它。那么事件将是 `Key::Esc`，它将消费它，并返回一个 `PopupClose` 消息。消息完全由用户通过模板类型定义，但我们将在本指南后面看到这一点。

- **Command**（通常称为 `Cmd`）：是由**组件**在接收到**事件**时生成的实体。组件使用它来操作其**MockComponent**。我们将在后面看到为什么需要这两个实体。
- **View**：视图是存储所有组件的地方。视图基本上有三个任务：
  
  - **管理组件的挂载/卸载**：组件在创建时挂载到视图中。视图防止挂载重复的组件，并在您尝试操作不存在的组件时发出警告。
  - **管理焦点**：视图保证一次只有一个组件处于活动状态。活动组件启用了一个专用属性（我们将在后面看到），所有事件都将转发给它。视图跟踪所有先前活动的组件，因此如果当前活动组件失去焦点，如果没有其他组件要激活，则先前活动的组件将变为活动状态。
  - **提供操作组件的 API**：一旦组件挂载到视图中，它们必须以安全的方式对外部可访问。这得益于视图公开的桥接方法。由于每个组件必须唯一标识才能被访问，您需要为组件定义一些 ID。
- **Model**：模型是您为应用定义的结构，用于实现**更新例程**。
- **Subscription** 或 *Sub*：订阅是一个规则集，它告诉**应用**基于某些规则将事件转发给其他组件，即使它们不处于活动状态。我们将在高级概念中讨论订阅。
- **Port**：Port 是一个事件监听器，它将使用名为 `Poll` 的 trait 来获取传入事件。Port 定义了要调用的 trait 和每次调用之间必须经过的时间间隔。事件随后被转发给订阅的组件。输入监听器是一个 port，但您也可以实现例如 HTTP 客户端来获取一些数据。无论如何，我们将在高级概念中看到 ports，因为它们不太常用。
- **Event Listener**：这是一个线程，它轮询 ports 以读取传入事件。事件随后报告给**应用**。
- **Application**：应用是围绕*视图*、*订阅* 和 *事件监听器* 的统一包装器。它公开了到视图的桥接、一些*订阅*的简写；但它的主要功能是 `tick()`。正如我们将在后面看到的，tick 是所有框架魔法发生的地方。
- **Update routine**：更新例程是一个函数，必须由**模型**实现，是*Update trait*的一部分。这个函数既简单又重要。它以可变引用形式接收*模型*、可变引用形式的*视图*和传入的**消息**。基于*消息*的值，它会在模型或视图上引起特定行为。如果您问的话，它只是一个*match case*，并且可以返回一个*消息*，这将导致例程被应用递归调用。稍后，当我们看到示例时，您会看到这有多酷。

---

## 原型组件 (MockComponent) 与 组件 (Component)

我们已经大致说过这两个实体是什么，但现在该在实践中看看它们了。
我们应该记住的第一件事是，它们都是**Traits**，并且根据设计，一个*Component*也是一个*MockComponent*。
让我们详细看看它们的定义：

### 原型组件

原型组件旨在*通用*（但不要太过）和*可重用*，但同时具有*单一职责*。
例如：

- ✅ 显示单行文本的 Label 是一个好的 原型组件。
- ✅ 像 HTML 中的 `<input>` 这样的 Input 组件是一个好的 原型组件。即使它可以处理许多输入类型，它仍然具有单一职责，是通用的且可重用的。
- ❌ 可以同时处理文本、单选按钮和复选框的输入是一个糟糕的 原型组件。它太通用了。
- ❌ 接收服务器远程地址的输入是一个糟糕的 原型组件。它不通用。

这些只是指南，但只是为了让您了解 原型组件是什么。

原型组件还处理**状态**和**属性**，这些完全基于您的需求由用户定义。有时您甚至可能有没有任何状态的组件（例如标签）。

实际上，原型组件是一个 trait，需要实现以下方法：

```rust
pub trait MockComponent {
    fn view(&mut self, frame: &mut Frame, area: Rect);
    fn query(&self, attr: Attribute) -> Option<AttrValue>;
    fn attr(&mut self, attr: Attribute, value: AttrValue);
    fn state(&self) -> State;
    fn perform(&mut self, cmd: Cmd) -> CmdResult;
}
```

trait 要求您实现：

- *view*：在提供区域中渲染组件的方法。您必须使用 `ratatui` widgets 根据组件的属性和状态来渲染组件。
- *query*：返回组件属性中某个属性的值。
- *attr*：为组件属性分配某个属性。
- *state*：获取当前组件状态。如果没有状态，则返回 `State::None`。
- *perform*：对组件执行提供的**命令**。此方法由**组件**调用，我们将在后面看到。命令应更改组件状态。一旦操作处理完毕，它必须将 `CmdResult` 返回给**组件**。

### 组件

所以，显然 原型组件定义了我们需要处理属性、状态和渲染的所有内容。那么为什么我们还没有完成，还需要一个组件 trait 呢？

1. MockComponent 必须是**通用的**：原型组件在库中分发（例如 `tui-realm-stdlib`），因此它们不能消费 `Event` 或产生 `Message`。
2. 由于第 1 点，我们需要一个能够产生 `Msg` 并消费 `Event` 的实体。这两个实体完全或部分由用户定义，这意味着它们对于每个 realm 应用都是不同的。这意味着组件必须适应应用。
3. **不可能满足所有人的需求**：我在 tui-realm 0.x 中尝试过，但这根本不可能。在某个时刻，我只是开始在其他属性中添加属性，但最终我不得不从头开始重新实现 stdlib 组件，只是为了获得一些不同的逻辑。原型组件很好，因为它们通用，但不要太过；它们必须对我们表现得像傻瓜一样。组件正是我们应用想要的。我们想要一个文本输入，但我们希望当我们输入 'a' 时它会改变颜色。您可以使用组件做到这一点，但不能使用 原型组件。哦，我几乎忘记了泛化 原型组件 最糟糕的事情：**键绑定**。

这么说，什么是组件？

组件是 原型组件 的应用特定唯一实现。让我们以表单为例，假设第一个字段是接收用户名的文本输入。如果我们用 HTML 来思考，它肯定是一个 `<input type="text" />` 对吧？您的网页中的许多其他组件也是如此。因此，文本输入将是 tui-realm 中的 `MockComponent`。但*那个*用户名输入字段将是您的**用户名文本输入**。`UsernameInput` 将包装一个 `Input` 原型组件，但基于传入事件，它将以不同方式操作 原型组件，并且如果与例如 `EmailInput` 相比，将产生不同的**消息**。

所以，让我说明从现在开始您必须记住的最重要的事情：**组件是唯一的 ❗** 在您的应用中。您**永远不应多次使用相同的组件**。

现在让我们看看组件在实践中是什么：

```rust
pub trait Component<Msg, UserEvent>: MockComponent
where
    Msg: PartialEq,
    UserEvent: Eq + PartialEq + Clone + PartialOrd,
{
    fn on(&mut self, ev: Event<UserEvent>) -> Option<Msg>;
}
```

相当简单吧？是的，我的意图是让它们尽可能轻量，因为您必须为视图中的每个组件实现一个。正如您可能注意到的，Component 需要实现 `MockComponent`，所以实际上我们也会有类似这样的东西：

```rust
pub struct UsernameInput {
    component: Input, // 其中 Input 实现了 `MockComponent`
}

impl Component for UsernameInput { ... }
```

您可能注意到的另一件事，可能会吓到你们中的一些人，是 Component 采用的两个泛型类型。
让我们看看这两种类型是什么：

- `Msg`：定义您的应用将在**更新例程**中处理的**消息**类型。实际上，在 tui-realm 中，消息不是在库中定义的，而是由用户定义的。我们将在后面的"第一个应用的制作"中详细讨论这一点。对于 Message 的唯一要求是它必须实现 `PartialEq`，因为您必须能够在**更新**中匹配它。
- `UserEvent`：用户事件定义了您的应用可以处理的自定义事件。正如我们之前所说，tui-realm 通常会发送有关用户输入或终端事件的事件，加上一个称为 `Tick` 的特殊事件（但我们将在后面讨论）。除了这些之外，我们已经看到还有其他称为 `Port` 的特殊实体，它们可能从其他源返回事件。由于 tui-realm 需要知道这些事件是什么，您需要提供您的 ports 将产生的类型。

    如果我们看一下 `Event` 枚举，一切都会变得清晰。

    ```rust
    pub enum Event<UserEvent>
    where
        UserEvent: Eq + PartialEq + Clone + PartialOrd,
    {
        /// 键盘事件
        Keyboard(KeyEvent),
        /// 终端窗口调整大小后引发的事件
        WindowResize(u16, u16),
        /// UI tick 事件（应可配置）
        Tick,
        /// 未处理的事件；空事件
        None,
        /// 用户事件；不会被标准库或默认输入事件监听器使用；
        /// 但可以在用户定义的 ports 中使用
        User(UserEvent),
    }
    ```

    如您所见，`Event` 有一个称为 `User` 的特殊变体，它接受一个特殊类型 `UserEvent`，确实可以用于使用用户定义的事件。

    > ❗如果您的应用没有任何 `UserEvent`，您可以通过传递 `Event<NoUserEvent>` 来声明事件，这是一个空枚举

### 属性 (Properties) 与状态 (States)

所有组件都由属性描述，并且通常也由状态描述。但它们之间有什么区别？

基本上**属性**描述组件如何渲染以及它应该如何行为。

例如，属性是**样式**、**颜色**或一些属性，如"这个列表应该滚动吗？"。
属性始终存在于组件中。

另一方面，状态是可选的，并且*通常*仅由用户可以与之交互的组件使用。
状态不会描述样式或组件的行为方式，而是组件的当前状态。状态通常也会在用户执行某个**命令**后更改。

让我们看看如何区分组件上的属性和状态，假设这个组件是一个*复选框*：

- 复选框的前景和背景是**属性**（在交互时不改变）
- 复选框选项是**属性**
- 当前选定的选项是**状态**。（它们在用户交互时改变）
- 当前高亮项是**状态**。

### 事件 (Events) 与命令 (Commands)

我们已经几乎看到了组件背后的所有方面，但我们仍然需要讨论一个重要概念，即 `Event` 和 `Cmd` 之间的区别。

如果我们看一下**组件** trait，我们会看到 `on()` 方法具有以下签名：

```rust
fn on(&mut self, ev: Event<UserEvent>) -> Option<Msg>;
```

并且我们知道 `Component::on()` 将调用其**MockComponent** 的 `perform()` 方法，以更新其状态。perform 方法具有以下签名：

```rust
fn perform(&mut self, cmd: Cmd) -> CmdResult;
```

如您所见，**组件**消费一个 `Event` 并产生一个 `Msg`，而由组件调用的 原型组件 消费一个 `Cmd` 并产生一个 `CmdResult`。

如果我们看一下两种类型声明，我们会发现在范围方面存在差异，让我们看一下：

```rust
pub enum Event<UserEvent>
where
    UserEvent: Eq + PartialEq + Clone + PartialOrd,
{
    /// 键盘事件
    Keyboard(KeyEvent),
    /// 终端窗口调整大小后引发的事件
    WindowResize(u16, u16),
    /// UI tick 事件（应可配置）
    Tick,
    /// 未处理的事件；空事件
    None,
    /// 用户事件；不会被标准库或默认输入事件监听器使用；
    /// 但可以在用户定义的 ports 中使用
    User(UserEvent),
}

pub enum Cmd {
    /// 描述用户"输入"了一个字符
    Type(char),
    /// 描述"光标"移动或其他类型的移动
    Move(Direction),
    /// `Move` 的扩展，定义滚动。步长应在属性中定义（如果有）。
    Scroll(Direction),
    /// 用户提交字段
    Submit,
    /// 用户"删除"了某些内容
    Delete,
    /// 用户切换了某些内容
    Toggle,
    /// 用户更改了某些内容
    Change,
    /// 用户定义的时间量已过，组件应更新
    Tick,
    /// 用户定义的命令类型。您不会在 stdlib 中找到这些类型的命令，但可以在自己的组件中使用它们。
    Custom(&'static str),
    /// `None` 不会做任何事情
    None,
}
```

在某些方面，它们看起来相似，但有些东西立即变得清晰：

- Event 严格绑定到"硬件"，它接收键事件、终端事件或其他源的事件。
- Cmd 完全独立于硬件和终端，它完全是关于 UI 逻辑的。我们仍然有 `KeyEvent`，但我们也有 `Type`、`Move`、`Submit`、自定义事件（但没有泛型）等等。

---

## 应用、模型和视图

现在我们已经了解了组件是什么，让我们看看如何将它们组合在一起以创建我们的应用。
正如我们在核心概念中看到的，应用由三个主要部分组成：

- **模型**：代表应用状态的结构。
- **视图**：存储所有组件并管理焦点的地方。
- **应用**：包装视图、订阅和事件监听器的实体。

让我们详细看看每一个。

### 视图 (View)

视图是存储所有组件的地方。视图基本上有三个任务：

- **管理组件的挂载/卸载**：组件在创建时挂载到视图中。视图防止挂载重复的组件，并在您尝试操作不存在的组件时发出警告。
- **管理焦点**：视图保证一次只有一个组件处于活动状态。活动组件启用了一个专用属性（我们将在后面看到），所有事件都将转发给它。视图跟踪所有先前活动的组件，因此如果当前活动组件失去焦点，如果没有其他组件要激活，则先前活动的组件将变为活动状态。
- **提供操作组件的 API**：一旦组件挂载到视图中，它们必须以安全的方式对外部可访问。这得益于视图公开的桥接方法。由于每个组件必须唯一标识才能被访问，您需要为组件定义一些 ID。

#### 焦点

焦点是视图的一个重要方面。视图保证一次只有一个组件处于活动状态。活动组件是启用了一个称为 `Attribute::Focus` 的专用属性的组件。此属性用于以不同方式渲染活动组件（例如，更改边框颜色）。

视图还跟踪所有先前活动的组件，因此如果当前活动组件失去焦点，如果没有其他组件要激活，则先前活动的组件将变为活动状态。

### 模型 (Model)

模型是您为应用定义的结构，用于实现**更新例程**。模型代表应用状态，并且通常包含应用需要跟踪的所有数据。

模型必须实现 `Update` trait，该 trait 要求实现 `update()` 方法。此方法以可变引用形式接收模型、可变引用形式的视图和传入的消息。基于消息的值，它会在模型或视图上引起特定行为。

### 应用

应用是围绕*视图*、*订阅*和*事件监听器*的超级包装器。它公开了到视图的桥接、一些*订阅*的简写；但它的主要功能是 `tick()`。

`tick()` 方法是所有框架魔法发生的地方。它执行以下操作：

1. 从事件监听器获取传入事件
2. 将事件转发给订阅的组件
3. 调用组件的 `on()` 方法，该方法产生消息
4. 调用模型的 `update()` 方法，传入消息
5. 重复直到应用退出

---

## 生命周期（或 "tick"）

tui-realm 应用的生命周期基于称为 "tick" 的概念。tick 是应用处理传入事件、更新模型和重新渲染视图的周期。

生命周期如下：

1. **事件**：事件由事件监听器从 ports 读取。事件随后被转发给订阅的组件。
2. **消息**：组件消费事件并产生消息。消息随后传递给模型的更新例程。
3. **更新**：模型的更新例程消费消息并更新模型状态或视图。
4. **视图**：视图根据当前模型状态和组件状态重新渲染。
5. **重复**：循环重复，直到应用退出。

---

## 我们的第一个应用

现在我们已经了解了 tui-realm 的核心概念，让我们创建我们的第一个应用。我们将创建一个简单的计数器应用，其中有两个计数器：一个可以通过按 `+` 和 `-` 递增和递减，另一个可以通过按 `w` 和 `s` 递增和递减。

### 实现计数器

首先，让我们定义我们的消息类型。消息类型是枚举，将代表我们的应用可以处理的所有可能消息。

### 定义消息类型

```rust
#[derive(PartialEq)]
pub enum Msg {
    AppClose,
    CounterIncrement(usize),
    CounterDecrement(usize),
}
```

这里我们定义了三种消息：

- `AppClose`：关闭应用
- `CounterIncrement(usize)`：递增指定 ID 的计数器
- `CounterDecrement(usize)`：递减指定 ID 的计数器

### 定义组件标识符

接下来，我们需要为我们的计数器定义标识符。由于每个组件必须唯一标识，我们将为每个计数器定义一个 ID。

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Id {
    Counter1,
    Counter2,
}
```

### 实现两个计数器组件

现在我们将实现两个计数器组件。每个计数器组件将包装一个 `Input` 原型组件，但基于传入事件，它将产生不同的消息。

```rust
pub struct Counter {
    component: Input,
    id: Id,
}

impl Counter {
    pub fn new(id: Id) -> Self {
        let mut component = Input::default()
            .foreground(Color::Yellow)
            .background(Color::Black)
            .placeholder("0");
        
        component.attr(Attribute::Title, AttrValue::Title("Counter".into()));
        
        Self { component, id }
    }
}

impl Component<Msg, NoUserEvent> for Counter {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent { code: KeyCode::Char('+'), .. }) => {
                Some(Msg::CounterIncrement(self.id as usize))
            }
            Event::Keyboard(KeyEvent { code: KeyCode::Char('-'), .. }) => {
                Some(Msg::CounterDecrement(self.id as usize))
            }
            Event::Keyboard(KeyEvent { code: KeyCode::Char('w'), .. }) if self.id == Id::Counter2 => {
                Some(Msg::CounterIncrement(self.id as usize))
            }
            Event::Keyboard(KeyEvent { code: KeyCode::Char('s'), .. }) if self.id == Id::Counter2 => {
                Some(Msg::CounterDecrement(self.id as usize))
            }
            Event::Keyboard(KeyEvent { code: KeyCode::Esc, .. }) => {
                Some(Msg::AppClose)
            }
            _ => None,
        }
    }
}

impl MockComponent for Counter {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        self.component.view(frame, area);
    }
    
    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.component.query(attr)
    }
    
    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.component.attr(attr, value);
    }
    
    fn state(&self) -> State {
        self.component.state()
    }
    
    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        self.component.perform(cmd)
    }
}
```

### 实现模型

现在我们将实现我们的模型。模型将包含两个计数器的当前值。

```rust
pub struct Model {
    counter1: i32,
    counter2: i32,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            counter1: 0,
            counter2: 0,
        }
    }
}

impl Update<Msg, NoUserEvent, ()> for Model {
    fn update(&mut self, msg: Option<Msg>, view: &mut View<Msg, NoUserEvent, ()>) -> Option<Msg> {
        match msg {
            Some(Msg::AppClose) => {
                // 退出应用
                std::process::exit(0);
            }
            Some(Msg::CounterIncrement(id)) => {
                if id == Id::Counter1 as usize {
                    self.counter1 += 1;
                    if let Some(mut counter) = view.data::<Counter>(Id::Counter1) {
                        counter.attr(Attribute::Value, AttrValue::String(self.counter1.to_string()));
                    }
                } else if id == Id::Counter2 as usize {
                    self.counter2 += 1;
                    if let Some(mut counter) = view.data::<Counter>(Id::Counter2) {
                        counter.attr(Attribute::Value, AttrValue::String(self.counter2.to_string()));
                    }
                }
            }
            Some(Msg::CounterDecrement(id)) => {
                if id == Id::Counter1 as usize {
                    self.counter1 -= 1;
                    if let Some(mut counter) = view.data::<Counter>(Id::Counter1) {
                        counter.attr(Attribute::Value, AttrValue::String(self.counter1.to_string()));
                    }
                } else if id == Id::Counter2 as usize {
                    self.counter2 -= 1;
                    if let Some(mut counter) = view.data::<Counter>(Id::Counter2) {
                        counter.attr(Attribute::Value, AttrValue::String(self.counter2.to_string()));
                    }
                }
            }
            _ => {}
        }
        None
    }
}
```

### 应用设置和主循环

最后，我们将设置我们的应用并运行主循环。

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化终端
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stdout()))?;
    
    // 创建模型和视图
    let model = Model::default();
    let mut view = View::new();
    
    // 创建计数器组件并挂载到视图
    let counter1 = Counter::new(Id::Counter1);
    let counter2 = Counter::new(Id::Counter2);
    
    view.mount(Id::Counter1, Box::new(counter1), None);
    view.mount(Id::Counter2, Box::new(counter2), None);
    
    // 创建应用
    let mut app = Application::new(model, view);
    
    // 运行主循环
    loop {
        terminal.draw(|f| {
            app.view().render(f, f.size());
        })?;
        
        if let Event::Keyboard(KeyEvent { code: KeyCode::Char('q'), .. }) = event::read()? {
            break;
        }
        
        app.tick()?;
    }
    
    Ok(())
}
```

---

## 下一步

恭喜！您已经创建了您的第一个 tui-realm 应用。您现在可以：

- 探索[高级概念](advanced.md)以了解订阅、ports 和其他高级功能
- 查看[从 tui-realm 0.x 迁移](migrating-legacy.md)指南，如果您有旧版应用
- 开始构建您自己的应用！

记住，tui-realm 旨在易于学习和使用，所以不要害怕尝试新事物。快乐编码！ 🚀