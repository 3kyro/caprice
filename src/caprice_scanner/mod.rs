use crossterm::{InputEvent, KeyEvent};

pub(crate) enum TokenType {
    Token(String),
    BackSpace,
    Continue(String),
    Tab(String),
    Exit,
    None,
}

pub(crate) struct Scanner {
    buffer: String,
}

impl Scanner {
    pub(crate) fn new() -> Self {
        Scanner {
            buffer: String::new(),
        }
    }

    pub(crate) fn scan(&mut self, input_event: InputEvent) -> TokenType {
        match input_event {
            InputEvent::Keyboard(KeyEvent::Char(c)) => self.scan_char(c),
            InputEvent::Keyboard(KeyEvent::Backspace) => self.scan_backspace(),
            InputEvent::Keyboard(KeyEvent::Ctrl(c)) => self.scan_ctrl(c),
            _ => TokenType::None,
        }
    }

    pub(crate) fn scan_char(&mut self, c: char) -> TokenType {
        match c {
            '\t' => self.scan_tab(),
            '\r' | '\n' => self.scan_enter(),
            _ => self.scan_valid_char(c),
        }
    }

    pub(crate) fn scan_backspace(&mut self) -> TokenType {
        if !self.buffer.is_empty() {
            self.buffer.pop();
            TokenType::BackSpace
        } else {
            TokenType::None
        }
    }

    pub(crate) fn scan_ctrl(&mut self, c: char) -> TokenType {
        #[cfg(windows)]
        let exit_char = 'q';
        #[cfg(unix)]
        let exit_char = 'c';

        if c == exit_char {
            return TokenType::Exit;
        }
        TokenType::None
    }

    pub(crate) fn scan_tab(&mut self) -> TokenType {
        TokenType::Tab(self.buffer.clone())
    }

    pub(crate) fn scan_enter(&mut self) -> TokenType {
        let token = self.buffer.clone();
        self.buffer.clear();
        TokenType::Token(token)
    }

    pub(crate) fn scan_valid_char(&mut self, c: char) -> TokenType {
        if c.is_alphanumeric() || c == '#' || c == '_' {
            self.buffer.push(c);
            TokenType::Continue(self.buffer.clone())
        } else {
            TokenType::None
        }
    }

    pub(crate) fn update_buffer(&mut self, new_buffer: String) {
        self.buffer = new_buffer;
    }
}
