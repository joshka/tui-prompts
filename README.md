# tui-prompts

`tui-prompts` is a Rust crate that provides prompt widgets for the Ratatui crate. It allows for easy
creation of interactive command-line interfaces with various types of prompts. Inspired by
<https://www.npmjs.com/package/prompts> and various other prompt libraries.

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
tui-prompts = "0.1.0"
```

## Status

Pre-Alpha

## Examples

```rust
fn draw_ui<B: Backend>(&mut self, frame: &mut Frame<B>) {
    let area = Rect::new(0, 0, frame.size().width, 1);
    TextPrompt::from("Username").draw(frame, area, &mut self.username);

    let area = Rect::new(0, 1, frame.size().width, 1);
    TextPrompt::from("Password")
        .with_render_style(TextRenderStyle::Password)
        .draw(frame, area, &mut self.password);
}
```

![Text Prompt](https://vhs.charm.sh/vhs-4l3c6ufDiij4UQiZW07heP.gif)
