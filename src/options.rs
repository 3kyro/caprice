use crossterm::style::Color;

#[derive(Debug)]
/// Options for the `Caprice` REPL.
pub struct Options {
    /// Sets the prompt to the desired color.
    pub prompt_color: Color,
}

impl Options {
    /// Default options.
    pub fn default() -> Self {
        Options {
            prompt_color: Color::White,
        }
    }
}
