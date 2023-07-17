![](https://user-images.githubusercontent.com/381361/252977280-49b9ff66-f78d-4e16-b5ed-29d771bfcab2.png)

# tui-prompts

[![Crates.io](https://img.shields.io/crates/v/tui-prompts?logo=rust&style=for-the-badge)](https://crates.io/crates/tui-prompts)
[![License](https://img.shields.io/crates/l/tui-prompts?style=for-the-badge)](./LICENSE)
[![Docs.rs](https://img.shields.io/docsrs/tui-prompts?logo=rust&style=for-the-badge)](https://docs.rs/crate/tui-prompts/)  
[![Dependency Status](https://deps.rs/repo/github/joshka/tui-prompts/status.svg?style=for-the-badge)](https://deps.rs/repo/github/joshka/tui-prompts)
[![Codecov](https://img.shields.io/codecov/c/github/joshka/tui-prompts?logo=codecov&style=for-the-badge&token=BAQ8SOKEST)](https://app.codecov.io/gh/joshka/tui-prompts)
[![Discord](https://img.shields.io/discord/1070692720437383208?label=ratatui+discord&logo=discord&style=for-the-badge)](https://discord.gg/pMCEU9hNEj)

`tui-prompts` is a Rust crate that provides prompt widgets for the Ratatui crate. It allows for easy
creation of interactive command-line interfaces with various types of prompts. Inspired by
<https://www.npmjs.com/package/prompts> and various other prompt libraries.

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
ratatui = "0.21.0"
tui-prompts = "0.1.0"
```

## Status

Very much a work in progress

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

![Text Prompt](https://vhs.charm.sh/vhs-1o1m3o1jSCtdacuZwz326V.gif)

## License

Dual-licensed under [Apache 2.0](./LICENSE-APACHE) or [MIT](./LICENSE-MIT).
