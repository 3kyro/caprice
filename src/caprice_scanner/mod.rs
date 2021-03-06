use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

pub(crate) enum TokenType {
    Token(String),
    BackSpace,
    Continue(String),
    Tab(String),
    Exit,
    None,
}

#[derive(Debug)]
pub(crate) struct Scanner {
    buffer: String,
    pub(crate) enable_ctrl_c: bool,
}

impl Scanner {
    pub(crate) fn new() -> Self {
        Scanner {
            buffer: String::new(),
            enable_ctrl_c: true,
        }
    }

    pub(crate) fn scan(&mut self, input_event: Event) -> TokenType {
        match input_event {
            Event::Key(KeyEvent {
                code: KeyCode::Tab, ..
            }) => self.scan_tab(),
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) => self.scan_enter(),
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
            }) => self.scan_ctrl_c(),
            Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                ..
            }) => self.scan_char(c),
            Event::Key(KeyEvent {
                code: KeyCode::Backspace,
                ..
            }) => self.scan_backspace(),
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

    pub(crate) fn scan_ctrl_c(&self) -> TokenType {
        if self.enable_ctrl_c {
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
        if c.is_alphanumeric() || c == '/' || c == '_' {
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
