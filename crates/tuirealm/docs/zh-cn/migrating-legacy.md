📍<u>**简体中文**</u> | <a href="../en/migrating-legacy.md">English</a>

# 从 tui-realm 0.x 迁移

- [从 tui-realm 0.x 迁移](#从-tui-realm-0x-迁移)
  - [简介](#简介)
  - [为什么选择 tui-realm 1.x 🤔](#为什么选择-tui-realm-1x-)
    - [旧 API 的问题](#旧-api-的问题)
  - [变化内容](#变化内容)
  - [如何迁移到 tui-realm 1.x](#如何迁移到-tui-realm-1x)

---

## 简介

本指南将逐步解释如何将 tui-realm 0.x 应用程序迁移到 1.x 版本。

您将学习到：

- 为什么应该迁移到 tui-realm 1.x
- 什么是新的，什么已被弃用
- 如何实际逐步迁移应用

## 为什么选择 tui-realm 1.x 🤔

可以说 tui-realm 1.x 一直是我的计划，自从 tui-realm 0.x 发布以来。
我一直梦想为我的项目"termscp"创建一个事件驱动的框架，但问题基本上有两个：

- 我真的没有清晰的想法如何实现这样的东西
- 我真的不想在一个可能失败的框架上工作
- tui-realm 一开始并不存在。它的第一个版本嵌入在 termscp 中，所以我甚至不想让它成为一个公共 crate

但自那时以来，很多事情发生了变化：

- 我对 Rust 变得更加自信，并且非常了解如何使用高级概念
- 我的新工作让我进步了很多，现在我在编程和设计库方面更加出色
- 我看到 tui-realm 实际上有效，不仅对我有效

所以一切似乎都在说"你应该为 tui-realm 实现新的 api"，但这真的有必要吗？

### 旧 API 的问题

我希望我不是唯一一个这样想的人，但 tui-realm 的旧 API 真的很糟糕，特别是：

- 缺乏事件抽象：组件直接处理键盘事件，这很糟糕。特别是对于 stdlib。如果用户想使用"WASD"而不是"箭头键"来移动，他必须重新实现整个组件才能有这种行为。这真的很糟糕。我本可以为组件实现 `keymap`。是的，但这只是另一个补丁。
- 属性系统是个好主意，但在实现中很糟糕。它在可以存储的内容方面太有限了，我在每个版本中都更改了它，因为它**太有限了**。一开始有一些用于文本、对齐、样式等的静态属性。然后有一个 HashMap，最后是 `own` 属性，它是字符串和 `PropPayload` 的映射。你知道什么吗？它也很糟糕。我从未理解为什么我从未想过像 tui-realm 1.x 的新属性系统这样简单的东西。我几乎忘记了最糟糕的部分：属性构建器。实现起来真的很无聊，使用起来更糟糕。
- Crossterm 0.20：当 crossterm 0.20 发布时，我真的疯了。他们从 `KeyEvent` 中移除了 `Eq`，这导致每个人都在更新例程中实现**可怕的**匹配案例。这可能是促使我实现 tui-realm 1.x 的最强点之一。
- `Msg` 并非真正的消息：消息机制本身不错...  (当然，因为我从 Elm lang 搬运的！！！)，但实现过于静态，不是用户定义的，总体上无用。你最终只会得到巨大的更新函数来匹配它们。
- 不支持其他后端：我甚至需要解释这一点吗？只支持 crossterm。我仍然认为 crossterm 很好，因为它可以在每个平台上运行。但但是，如果您不需要 Windows 支持，termion 要好得多。人们有品味。如果用户不喜欢 crossterm，该用户将不会使用 tui-realm。
- 初始化配置过于繁琐：配置一个 realm 应用花费的时间太长了。

---

## 变化内容

如果您阅读了之前的指南，您已经看到了新 api 的实体是什么，但不要惊慌，即使看起来不像，许多东西仍然有相同的用途，只是更花哨：

- Props 仍然存在，但不是保存随机属性，而是现在保存 `Attribute` 和 `AttrValue` 的映射。这真的很像 CSS，我们都讨厌它，但我们都知道它工作得很好。
- PropsBuilder 已被 原型组件 的构造函数取代
- Msg 不再在 tui-realm 中定义，您为您自己的应用定义您自己的消息。不再有无用的消息在应用中传播。只有您实际需要的消息。
- Event 和 KeyEvent 现在终于支持 Eq，因为我在 tui-realm 中包装了 crossterm 结构。
- Crossterm 不再是强制性的，您终于可以使用 termion 和任何您喜欢的东西（实际上只实现了 termion，但您可以实现 tui 支持的其他后端。但是真的，还有人在使用 rustbox 吗？）
- View 已被应用部分取代。我的意思是，仍然有一个视图，但您在程序中持有一个应用来处理视图。
- **更新 trait** 现在是强制性的 (:feelsgood) 以便在应用上调用 `tick()` 方法。
- Component 已被 MockComponent 取代（并且方法名称已更改）。
- 您需要为 UI 中的所有元素实现一个 Component。

---

## 如何迁移到 tui-realm 1.x

现在是时候看看如何做了。我会很快解释这一点，但不要指望这对您来说很快。

我会对您诚实：迁移应用**不会**快，但会很容易，只是无聊。
花时间迁移您的应用，在一个全新的分支上工作，真的：花时间。完成迁移可能需要几个小时，但相信我：当您完成时，您会真的感到满意，看到应用看起来有多好。

现在让我们逐步看看如何执行迁移：

1. 更新 Cargo.toml 中的依赖项

    您仍然想使用 crossterm 还是想使用 termion？

    如果您想继续使用 crossterm：

    ```toml
    tuirealm = "^1.0.0"
    ```

    如果您想选择 termion：

    ```toml
    tuirealm = { "version" = "^1.0.0", default-features = false, features = [ "derive", "with-termion" ] }
    ```

    不用担心将 crossterm 迁移到 termion。没有必要。我们稍后会看到原因。

    哦，不要忘记迁移 stdlib（如果您使用它的话（我知道您使用它 😉））

    ```toml
    tui-realm-stdlib = "^1.0.0"
    ```

    或使用您喜欢的后端（必须与 tuirealm 匹配！）

    ```toml
    tui-realm-stdlib = { "version" = "^1.0.0", default-features = false, features = [ "with-termion" ] }
    ```

2. 删除所有终端构造函数！让我们使用 TerminalBridge

    在 tui-realm 1.x 中，我实现了 `TerminalBridge` 以与终端有一个抽象层，以便在所有后端上具有相同的 API（我知道，这不应该在 realm 中实现，而应该在 tui-rs 中。但是...）。

    所以首先删除所有进入/离开备用屏幕和切换原始模式的方法，并将上下文中的终端替换为：

    ```rust
    use tuirealm::terminal::TerminalBridge;
    
    Context {
      // ...
      terminal: TerminalBridge::new().expect("无法初始化终端"),
    }
    ```

    终端桥是您与终端一起工作所需的一切，并在 termion 和 crossterm 或您使用的任何东西上提供相同的方法。

3. 定义一个枚举，包含您使用的所有组件的 id

    可能在 tui-realm 0.x 中，您使用一些常量作为组件的 id。在 tui-realm 1.x 中，我们需要使用一个类型作为组件的标识符，然后必须提供给应用才能工作。
    您仍然可以使用字符串，但字符串很糟糕，枚举要好得多：

    ```rust
    #[derive(Debug, Eq, PartialEq, Clone, Hash)]
    pub enum Id {
        AddressInput,
        PasswordInput,
        ProtocolRadio,
        GlobalListener,
    }
    ```

4. 定义您的应用将处理的消息

    花时间思考您的应用将处理哪种消息。
    记住：消息是您的**模型**或**视图**需要关注的事件，而不是组件需要接收的内容。

    ```rust
    #[derive(Debug, PartialEq)]
    pub enum Msg {
        AppClose,
        FormSubmit,
        ProtocolChanged(FileTransferProtocol),
        None,
    }
    ```

5. 将模型与视图分开

    在您当前的实现中，您可能有一个同时保存模型数据和视图的结构。这不再有效（不会构建）。您需要有一个结构来保存模型结构和应用：

    然后：

    ```rust
    struct Activity {
      context: Context,
      protocol: FileTransferProtocol,
      address: String,
      view: View, // 在用户级别被应用替换
    }
    ```

    现在：

    ```rust
    struct Activity {
      model: Model,
      application: Application<Id, Msg, NoUserEvent>,
    }

    struct Model {
      context: Context,
      protocol: FileTransferProtocol,
      address: String,
    }

    impl Update for Model {
      // ...（将使用应用传递的视图，这就是为什么模型不能持有视图）
    }
    ```

6. 为您要使用的每个组件实现 Component trait

    花时间做这件事，这将需要很长时间。基本上，您需要为应用中的所有组件实现一个 `Component`。
    组件将始终具有 `component: impl MockComponent` 作为属性，它将使用由您或 stdlib 实现的原型组件。如果您使用 stdlib 组件，请记住使用命令 api 来匹配事件和结果。请记住，您不必为组件实现 `MockComponent`（除非您需要指定替代行为），有一个神奇的 `#[derive(MockComponent)]` 过程宏。
    在组件的构造函数中，您将指定以前在属性构建器中设置的所有内容：

    然后：

    ```rust
    InputPropsBuilder::default()
      .with_foreground(fg)
      .with_borders(Borders::ALL, BorderType::Rounded, fg)
      .with_label(label, Alignment::Left)
      .with_input(typ);
    ```

    现在：

    ```rust
    use tui_realm_stdlib::Input;

    #[derive(MockComponent)]
    pub struct AddressInput {
        component: Input,
    }

    impl Default for AddressInput {
        fn default() -> Self {
            Self {
                component: Input::default()
                    .foreground(Color::LightBlue)
                    .borders(
                        Borders::default()
                            .color(Color::LightBlue)
                            .modifiers(BorderType::Rounded),
                    )
                    .input_type(InputType::Text)
                    .placeholder(
                        "192.168.1.10",
                        Style::default().fg(Color::Rgb(120, 120, 120)),
                    )
                    .title("远程地址", Alignment::Left),
            }
        }
    }

    impl Component<Msg, NoUserEvent> for AddressInput {
        fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
            let result = match ev {
                Event::Keyboard(KeyEvent {
                    code: Key::Enter,
                    modifiers: KeyModifiers::NONE,
                }) => return Some(Msg::FormSubmit),
                Event::Keyboard(KeyEvent {
                    code: Key::Char(ch),
                    modifiers: KeyModifiers::NONE,
                }) => self.perform(Cmd::Type(ch)),
                Event::Keyboard(KeyEvent {
                    code: Key::Left,
                    modifiers: KeyModifiers::NONE,
                }) => self.perform(Cmd::Move(Direction::Left)),
                Event::Keyboard(KeyEvent {
                    code: Key::Right,
                    modifiers: KeyModifiers::NONE,
                }) => self.perform(Cmd::Move(Direction::Right)),
                Event::Keyboard(KeyEvent {
                    code: Key::Home,
                    modifiers: KeyModifiers::NONE,
                }) => self.perform(Cmd::GoTo(Position::Begin)),
                Event::Keyboard(KeyEvent {
                    code: Key::End,
                    modifiers: KeyModifiers::NONE,
                }) => self.perform(Cmd::GoTo(Position::End)),
                Event::Keyboard(KeyEvent {
                    code: Key::Delete,
                    modifiers: KeyModifiers::NONE,
                }) => self.perform(Cmd::Cancel),
                Event::Keyboard(KeyEvent {
                    code: Key::Backspace,
                    modifiers: KeyModifiers::NONE,
                }) => self.perform(Cmd::Delete),
                Event::Keyboard(KeyEvent {
                    code: Key::Tab,
                    modifiers: KeyModifiers::NONE,
                }) => return Some(Msg::AddressInputBlur),
                _ => return None,
            };
            Some(Msg::None)
        }
    }
    ```

7. 为您的模型实现更新例程

    为 `Model` 实现 `Update` trait，匹配所有 `Msg` 并执行您需要执行的操作。

    ```rust
    impl Update<Id, Msg, NoUserEvent> for Model {
        fn update(&mut self, view: &mut View<Id, Msg, NoUserEvent>, msg: Option<Msg>) -> Option<Msg> {
            match msg.unwrap_or(Msg::None) {
                Msg::AppClose => {
                    self.quit = true;
                    None
                }
                // ... 
                Msg::None => None,
            }
        }
    }
    ```

8. 更新您之前的 `on()` 调用：

    ```rust
    if let Ok(sz) = app.tick(&mut model, PollStrategy::Once) {
        if sz > 0 {
            // 注意：如果至少处理了一个消息，则重绘
            model.redraw = true;
        }
    }
    // 重绘
    if model.redraw {
        // 视图必须由您自己实现！
        model.view(&mut app);
        model.redraw = false;
    }
    ```