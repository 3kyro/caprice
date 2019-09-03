mod terminal_manipulator;

use terminal_manipulator::*;
use crossterm::{KeyEvent};

use std::io::{Error, ErrorKind};

use super::autocomplete::*;

pub(crate) struct Parser {
    terminal: TerminalManipulator,
    functor: fn(String) -> Result<()>,
    buffer: String,
    tokens: Vec<String>,
    commands: Vec<String>
}

type Result<T> = std::result::Result<T, std::io::Error>;

impl Parser {
    pub fn new(functor: fn(String) -> Result<()>) -> Self {
        Parser {
            terminal: TerminalManipulator::new(),
            functor : functor,
            buffer : String::new(),
            tokens : Vec::new(),
            commands : Vec::new(),
        }
    }

    pub(crate) fn parse(&mut self) -> Result<Option<String>> {
        match self.terminal.next_key_event() {
            KeyEvent::Char(c) => {
                if let Some(result_string) = self.parse_char(c)? {
                    (self.functor)(result_string)?;
                }
            },
            KeyEvent::Backspace => {
                self.parse_backspace();
            },
            KeyEvent::Ctrl(c) => {
                self.parse_ctrl_c(c)?;
            },
            _ => {}
        }
        Ok(None)
    }

    fn parse_char(&mut self, c: char) -> Result<Option<String>> {
        match c {
            '\t' => self.parse_tab(),
            '\r' | '\n' => self.parse_enter(),
            _ => self.parse_valid_char(c),

        }
    }

    fn parse_backspace(&mut self) -> Result<()> {
        if !self.buffer.is_empty() {
            self.buffer.pop();
            self.terminal.cursor.move_left(1);
            self.terminal.clear_line()?;
        }
        Ok(())
    }

    pub (crate) fn parse_ctrl_c(&self, c: char) -> Result<()> {
        if c == 'c' {
            return Err(Error::new(ErrorKind::Interrupted, "user aborted"));
        }
        Ok(())
    }

    fn parse_tab(&self) -> Result<Option<String>> {
        unimplemented!()
    }

    fn parse_enter(&self) -> Result<Option<String>> {
        self.terminal.goto_next_line();
        if self.tokens.contains(&self.buffer) {
            return Ok(Some(self.buffer.clone()))
        } else 
        if self.commands.contains(&self.buffer) {
            self.parse_command(&self.buffer);
            Ok(None)
        } else {
            Ok(None)
        }
    }

    fn parse_command(&self, command: &String) {
        unimplemented!()
    }

    fn parse_valid_char(&mut self, c: char) -> Result<Option<String>> {
        if c.is_alphanumeric() || c == '#' || c == '_' {
            // insert new char to self.keyword
            self.buffer.push(c);
            self.buffer = self.buffer.trim_end().to_string();

            print!("{}", c);

            self.print_autocompleted()?

        }

        Ok(None)
    }   

    fn print_autocompleted(&self) -> Result<()> {
        // get autocomplete results
        let (_, common) = autocomplete(&self.buffer, &self.tokens);

        if let Some(result) = common {

            self.terminal.save_cursor()?;

            print_same_line_autocompleted(result, &self.buffer);
            
            self.terminal.restore_cursor()?;

        } else {
            // clear everything left of the cursor
            self.terminal.clear_from_cursor()?;
        }

        Ok(())
    }
}