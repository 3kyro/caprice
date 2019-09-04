mod terminal_manipulator;

use terminal_manipulator::*;
use crossterm::{KeyEvent, InputEvent};
use super::autocomplete::*;

pub struct Parser {
    terminal: TerminalManipulator,
    functor: fn(String) -> Result<()>,
    buffer: String,
    tokens: Vec<String>,
    commands: Vec<String>,
    prompt: String,
}

type Result<T> = std::result::Result<T, std::io::Error>;

impl Parser {
    pub fn new(functor: fn(String) -> Result<()>) -> Self {
        Parser {
            terminal: TerminalManipulator::new(),
            functor : functor,
            buffer : String::new(),
            tokens : vec!["some_token".to_owned(), "some_other_token".to_owned(), "none".to_owned()],
            commands: vec!["#list".to_owned()],
            prompt: "~".to_owned(),
        }
    }

    pub fn parse(&mut self) -> Result<()> {
        
        self.terminal.flush()?;

        if let Some(input_event) = self.terminal.next_key_event() {
            match input_event  {
                InputEvent::Keyboard(KeyEvent::Char(c))  => {
                    // if let Some(result_string) = self.parse_char(c)? {
                    //     (self.functor)(result_string)?;
                    // }
                    self.parse_char(c)?
                },
                InputEvent::Keyboard(KeyEvent::Backspace) => {
                    self.parse_backspace()?;
                },
                InputEvent::Keyboard(KeyEvent::Ctrl(c)) => {
                    self.parse_ctrl_c(c)?;
                },
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

    pub (crate) fn parse_ctrl_c(&self, c: char) -> Result<()> {
        if c == 'c' {
            self.terminal.exit()?;
        }
        Ok(())
    }

    fn parse_tab(&self) -> Result<()> {
        unimplemented!()
    }

    fn parse_enter(&mut self) -> Result<()> {
        if self.tokens.contains(&self.buffer) {
            (self.functor)(self.buffer.clone())?;
        } else 
        if self.commands.contains(&self.buffer) {
            self.parse_command(&self.buffer.clone())?;
        } 
        self.buffer.clear();
        self.next_line()?;

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

impl Drop for Parser {
    fn drop(&mut self) {
        self.terminal.exit().unwrap();
    }
}