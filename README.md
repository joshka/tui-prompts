# tui-prompts

`tui-prompts` is a Rust crate that provides prompt widgets for the Ratatui crate. It allows for easy
creation of interactive command-line interfaces with various types of prompts. Inspired by
<https://www.npmjs.com/package/prompts> and various other prompt libraries.

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
ratatui = "0.21.0"
tui-prompts = "0.1.0"

[patch.crates-io]
ratatui = { git = "https://github.com/joshka/ratatui.git", branch = "feat-stylize-all-the-things" }
tui-prompts = { git = "https://github.com/joshka/tui-prompts.git" }
```

## Status

This relies on some ratatui features not yet released, and is in a very alpha state.

## Examples

```rust
struct App<'a> {
    username: TextState<'a>,
    password: TextState<'a>,
}

impl<'a> App<'a> {
    fn draw_ui<B: Backend>(&mut self, frame: &mut Frame<B>) {
        let area = Rect::new(0, 0, frame.size().width, 1);
        TextPrompt::from("Username").draw(frame, area, &mut self.username);

        let area = Rect::new(0, 1, frame.size().width, 1);
        TextPrompt::from("Password")
            .with_render_style(TextRenderStyle::Password)
            .draw(frame, area, &mut self.password);
    }
}
```

![Text Prompt](https://vhs.charm.sh/vhs-7wYCLtdxtWqUEtNatBO77h.gif)

## License

Dual-licensed under [Apache 2.0](./LICENSE-APACHE) or [MIT](./LICENSE-MIT).
