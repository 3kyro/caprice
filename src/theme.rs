use crossterm::style::Color;

#[derive(Debug)]
/// Theme for the `Caprice` REPL.
pub struct Theme {
    /// Sets the prompt to the desired color.
    pub prompt_color: Color,
}

impl Theme {
    /// Default options.
    pub fn default() -> Self {
        Theme {
            prompt_color: Color::White,
        }
    }
}
