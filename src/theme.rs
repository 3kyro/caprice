use crossterm::style::Color;

#[derive(Debug, Clone, Copy)]
/// Theme for the `Caprice` REPL.
pub struct Theme {
    /// Sets the prompt to the desired color.
    pub prompt_color: Color,
    pub autocomplete_color: Color,
    pub suggestion_fg: Color,
    pub suggestion_bg: Color,
}

pub static DEFAULT_THEME: Theme = Theme {
    prompt_color: Color::White,
    autocomplete_color: Color::DarkGreen,
    suggestion_fg: Color::Black,
    suggestion_bg: Color::Grey,
};

pub static DARK_BLUE: Theme = Theme {
    prompt_color: Color::DarkBlue,
    autocomplete_color: Color::DarkGrey,
    suggestion_fg: Color::DarkGrey,
    suggestion_bg: Color::DarkBlue,
};
