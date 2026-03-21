# tuirealm_derive

<p align="center">
  <img src="https://rawcdn.githack.com/veeso/tui-realm/39c38c3bd905f724403481514adb2cf2b4e69a7b/docs/images/cargo/tui-realm-512.png" width="256" height="256" alt="logo" />
</p>

<p align="center">~ Automatically implements MockComponent ~</p>
<p align="center">
  <a href="#get-started-">Get started</a>
  ·
  <a href="https://github.com/veeso/tui-realm" target="_blank">tui-realm</a>
  ·
  <a href="https://docs.rs/tuirealm_derive" target="_blank">Documentation</a>
</p>

<p align="center">Developed by <a href="https://veeso.github.io/" target="_blank">@veeso</a></p>
<p align="center">Current version: 2.0.0 (12/10/2024)</p>

<p align="center">
  <a href="https://opensource.org/licenses/MIT"
    ><img
      src="https://img.shields.io/badge/License-MIT-teal.svg"
      alt="License-MIT"
  /></a>
  <a href="https://crates.io/crates/tuirealm_derive"
    ><img
      src="https://img.shields.io/crates/d/tuirealm_derive.svg"
      alt="Downloads counter"
  /></a>
  <a href="https://crates.io/crates/tuirealm_derive"
    ><img
      src="https://img.shields.io/crates/v/tuirealm_derive.svg"
      alt="Latest version"
  /></a>
  <a href="https://ko-fi.com/veeso">
    <img
      src="https://img.shields.io/badge/donate-ko--fi-red"
      alt="Ko-fi"
  /></a>
</p>

---

- [tuirealm\_derive](#tuirealm_derive)
  - [About tuirealm\_derive 👑](#about-tuirealm_derive-)
  - [Get started 🏁](#get-started-)
  - [Support the developer ☕](#support-the-developer-)
  - [Changelog ⏳](#changelog-)
  - [License 📃](#license-)

## About tuirealm_derive 👑

tuirealm_derive is a crate which implements the procedural macro `MockComponent` which can be used to automatically implement
the `MockComponent` trait for a tui-realm `Component`.
Indeed, as you already know if you're a tui-realm user, you've got two kind of component entities:

- MockComponent: generic graphic component which is not bridged to the application and is "reusable"
- Component: which uses a MockComponent as "backend" and is bridged to the application using the **Event -> Msg** system.

The Component wraps the MockComponent along with additional states. Such as:

```rust
pub struct IpAddressInput {
  component: Input,
}

impl MockComponent for IpAddressInput {
  
  ...

  fn state(&self) -> State {
    self.component.state()
  }

  ...

}

impl Component<Msg, UserEvent> for IpAddressInput {

  fn on(&mut self, ev: Event<UserEvent>) -> Option<Msg> {
    let cmd: Cmd = match ev {
      ...
    };
    match self.perform(cmd) {
      ...
    }
  }

}
```

Since `Component` **MUST** implement `MockComponent`, we need to implement the mock component trait too, which in most of the case it will just call the MockComponent methods on the inner `component` field. This is obviously kinda annoying to do for each component. That's why I implemented this procedural macro, which will automatically implement this logic on your component.

So basically instead of implementing `MockComponent` for your components, you can just do as follows:

```rust
#[derive(MockComponent)]
pub struct IpAddressInput {
  component: Input,
}

impl Component<Msg, UserEvent> for IpAddressInput {
  ...
}
```

With the directive `#[derive(MockComponent)]` we **don't have to** implement the mock component trait.

> ❗ In order to work, the procedural macro requires you to name the "inner" mock component as `component` as I did in the example.

If we give a deeper look at the macro, we'll see that what it does is:

```rust
impl MockComponent for IpAddressInput {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        self.component.view(frame, area);
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.component.query(attr)
    }

    fn attr(&mut self, query: Attribute, attr: AttrValue) {
        self.component.attr(query, attr)
    }

    fn state(&self) -> State {
        self.component.state()
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        self.component.perform(cmd)
    }
}
```

---

## Get started 🏁

In order to get started with **tuirealm_derive** all you need to do is to add [tui-realm](https://github.com/veeso/tui-realm) to your dependencies and enable the `derive` feature if needed.

If you're using the default features:

```toml
[dependencies]
tuirealm = "^2"
```

If you're not using the default features, be sure to enable the **derive** feature:

```toml
[dependencies]
tuirealm = { version = "^2", default-features = false, features = ["derive", "crossterm"] }
```

>⚠️ tuirealm_derive requires tui-realm >= 2.0.0; the old API is not supported

Then you need to include tuirealm in your project using the `macro use` directive:

> src/lib.rs

```rust
#[macro_use]
extern crate tuirealm;
```

and finally derive `MockComponent` on your components:

```rust
#[derive(MockComponent)]
pub struct MyComponent {
  component: MyMockComponentImpl,
}
```

> ❗ In order to work, the procedural macro requires you to name the "inner" mock component as `component` as I did in the example.

And ta-dah, you're ready to go 🎉

### Custom field names

By default a field of name `component` will be used as can be seen in the earlier examples, but this can be customized.

First option is to use a container-level attribute:

```rust
#[derive(MockComponent)]
#[component("radio")]
pub struct MyComponent1 {
  radio: Radio,
}

#[derive(MockComponent)]
#[component = "radio"]
pub struct MyComponent2 {
  radio: Radio,
}
```

Or field-level attribute:

```rust
#[derive(MockComponent)]
pub struct MyComponent {
  #[component]
  radio: Radio,
}
```

> ❗ Only one field can be the component and container- & field-level attributes cannot be used together.

Tuple Structs are also supported:

```rust
#[derive(MockComponent)]
pub struct MyComponent(Radio, SomeOtherType);

#[derive(MockComponent)]
pub struct MyComponent(SomeOtherType, #[component] Radio);
```

---

## Support the developer ☕

If you like tui-realm and you're grateful for the work I've done, please consider a little donation 🥳

You can make a donation with one of these platforms:

[![ko-fi](https://img.shields.io/badge/Ko--fi-F16061?style=for-the-badge&logo=ko-fi&logoColor=white)](https://ko-fi.com/veeso)
[![PayPal](https://img.shields.io/badge/PayPal-00457C?style=for-the-badge&logo=paypal&logoColor=white)](https://www.paypal.me/chrisintin)

---

## Changelog ⏳

View tuirealm_derive's changelog [HERE](CHANGELOG.md)

---

## License 📃

**tuirealm_derive** is licensed under the MIT license.

You can read the entire license [HERE](LICENSE)
