use crossterm::input::{InputEvent, KeyEvent};

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
    pub(crate) enable_ctrl_c: bool,
}

impl Scanner {
    pub(crate) fn new() -> Self {
        Scanner {
            buffer: String::new(),
            enable_ctrl_c: false,
        }
    }

    pub(crate) fn scan(&mut self, input_event: InputEvent) -> TokenType {
        match input_event {
            InputEvent::Keyboard(KeyEvent::Tab) => self.scan_tab(),
            InputEvent::Keyboard(KeyEvent::Enter) => self.scan_enter(),
            InputEvent::Keyboard(KeyEvent::Char(c)) => self.scan_char(c),
            InputEvent::Keyboard(KeyEvent::Backspace) => self.scan_backspace(),
            InputEvent::Keyboard(KeyEvent::Ctrl(c)) => self.scan_ctrl(c),
            _ => TokenType::None,
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
        // continue parsing if ctrl_C exit is disabled
        if !self.enable_ctrl_c {
            return TokenType::None;
        }

        if c == 'c' {
            TokenType::Exit
        } else {
            TokenType::None
        }
    }

    pub(crate) fn scan_tab(&mut self) -> TokenType {
        TokenType::Tab(self.buffer.clone())
    }

    pub(crate) fn scan_enter(&mut self) -> TokenType {
        let token = self.buffer.clone();
        self.buffer.clear();
        TokenType::Token(token)
    }

    pub(crate) fn scan_char(&mut self, c: char) -> TokenType {
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
