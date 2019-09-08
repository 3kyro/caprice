mod terminal_manipulator;

use super::autocomplete::*;
use crossterm::{InputEvent, KeyEvent};
use terminal_manipulator::*;

pub struct Parser {
    terminal: TerminalManipulator,
    functor: fn(String) -> Result<()>,
    buffer: String,
    tokens: Vec<String>,
    commands: Vec<String>,
    prompt: String,
    autocompleted: Autocomplete,
}

type Result<T> = std::result::Result<T, std::io::Error>;

impl Parser {
    pub fn new(functor: fn(String) -> Result<()>) -> Self {
        Parser {
            terminal: TerminalManipulator::new(),
            functor,
            buffer: String::new(),
            tokens: vec![
                "some_token".to_owned(),
                "some_other_token".to_owned(),
                "some_other_token".to_owned(),
                "some_other_token".to_owned(),
                "some_other_token".to_owned(),
                "some_other_token".to_owned(),
                "some_other_token".to_owned(),
                "some_other_token".to_owned(),
                "some_other_token".to_owned(),
                "some_other_token".to_owned(),
                "some_other_token".to_owned(),
                "some_other_token".to_owned(),
                "some_other_token".to_owned(),
                "some_other_token".to_owned(),
                "some_other_token".to_owned(),
                "some_other_token".to_owned(),
                "some_other_token".to_owned(),
                "some_other_token".to_owned(),
                "some_other_token".to_owned(),
                "some_other_token".to_owned(),
                "some_other_token".to_owned(),
                "some_other_token".to_owned(),
                "some_other_token".to_owned(),
                "some_other_token".to_owned(),
                "some_other_token".to_owned(),
                "some_other_token".to_owned(),
                "some_other_token".to_owned(),
                "some_other_token".to_owned(),
                "some_other_token".to_owned(),
                "some_other_token".to_owned(),
                "none".to_owned(),
            ],
            commands: vec!["#list".to_owned()],
            prompt: "~".to_owned(),
            autocompleted: Autocomplete::new(),

        }
    }

    pub fn parse(&mut self) -> Result<()> {
        self.terminal.flush()?;

        if let Some(input_event) = self.terminal.next_key_event() {
            match input_event {
                InputEvent::Keyboard(KeyEvent::Char(c)) => {
                    self.parse_char(c)?
                }
                InputEvent::Keyboard(KeyEvent::Backspace) => {
                    self.parse_backspace()?;
                }
                InputEvent::Keyboard(KeyEvent::Ctrl(c)) => {
                    self.parse_ctrl_c(c)?;
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn parse_char(&mut self, c: char) -> Result<()> {
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

    pub(crate) fn parse_ctrl_c(&self, c: char) -> Result<()> {
        if c == 'c' {
            self.terminal.exit()?;
        }
        Ok(())
    }

    fn parse_tab(&mut self) -> Result<()> {
        let word_margin = 2;
        self.autocompleted.autocomplete(&self.buffer, &self.tokens);

        if self.autocompleted.get_common().len() == 0 {
            return Ok(());
        }
        
        // print common string
        self.terminal.goto_begining_of_line();
        self.buffer = self.autocompleted.get_common().clone();
        print!("{}{}",self.prompt, self.buffer);

        // print other suggestions below the cursor
        self.autocompleted.amortisize();

        // get num of words that fit in one line
        let num_per_line;
        if let Some(first) = self.autocompleted.get_keywords().get(0) {
            // +2 for the number of spaces seperating each word
            // -2 to leave some space free at the edges of the terminam 
            num_per_line = (self.terminal.size().0 / (first.len() as u16 + 2)) - word_margin;
        } 
        else {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid autocompleted result"));
        }

        self.terminal.cursor_to_last_line()?;

        self.terminal.save_cursor()?;
        // begin from next line
        self.terminal.goto_next_line()?;

        let mut count: u16 = 0;
        for (_,word) in self.autocompleted.get_keywords().iter().enumerate() {
            print!("{}  ", word);
            count += 1;
            if count == num_per_line {
                self.terminal.goto_next_line()?;
                count = 0;
            }
        }
        self.terminal.restore_cursor()?;
        self.terminal.move_cursor_up(
            (num_per_line - word_margin) as i16 % self.autocompleted.get_keywords().len() as i16
        );
        Ok(())
    }

    fn parse_enter(&mut self) -> Result<()> {
        if self.tokens.contains(&self.buffer) {
            (self.functor)(self.buffer.clone())?;
            self.terminal.goto_begining_of_line();
        } else if self.commands.contains(&self.buffer) {
            self.parse_command(&self.buffer.clone())?;
            self.terminal.goto_begining_of_line();
        } else {
            self.terminal.goto_next_line()?;
        }
        self.buffer.clear();
        print!("{}", self.prompt);
        self.terminal.clear_from_cursor()?;

        Ok(())
    }

    fn next_line(&mut self) -> Result<()> {
        self.terminal.goto_begining_of_line();
        print!("{}", self.prompt);

        Ok(())
    }

    fn parse_command(&mut self, command: &String) -> Result<()> {
        if command == "#list" {
            self.terminal.goto_next_line()?;
            for token in self.tokens.iter() {
                println!("{}", token);
                self.terminal.goto_begining_of_line();
            }
        }

        Ok(())
    }

    fn parse_valid_char(&mut self, c: char) -> Result<()> {
        if c.is_alphanumeric() || c == '#' || c == '_' {
            // insert new char to self.keyword
            self.buffer.push(c);
            self.buffer = self.buffer.trim_end().to_string();

            print!("{}", c);

            self.print_autocompleted()?
        }

        Ok(())
    }

    pub fn init(&mut self) -> Result<()> {
        self.terminal.enable_raw_screen()?;

        print!("{}", self.prompt);

        Ok(())
    }

    fn print_autocompleted(&mut self) -> Result<()> {
        // get autocomplete results
        self.autocompleted.autocomplete(&self.buffer, &self.tokens);

        if !self.autocompleted.get_common().is_empty() {
            self.terminal.save_cursor()?;

            print_same_line_autocompleted(self.autocompleted.get_common().to_owned(), &self.buffer);

            self.terminal.restore_cursor()?;
        } else {
            // clear everything left of the cursor
            self.terminal.clear_from_cursor()?;
        }

        Ok(())
    }
}

impl Drop for Parser {
    fn drop(&mut self) {
        self.terminal.exit().unwrap();
    }
}
