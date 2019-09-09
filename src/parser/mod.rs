mod terminal_manipulator;

use crate::Result;
use super::autocomplete::*;
use crossterm::{InputEvent, KeyEvent, Attribute, Colored, Color};
use terminal_manipulator::*;

pub struct Caprice {
    terminal: TerminalManipulator,
    functor: fn(String) -> Result<()>,
    buffer: String,
    tokens: Vec<String>,
    commands: Vec<String>,
    prompt: String,
    autocompleted: Autocomplete,
}


impl Caprice {
    pub fn new(functor: fn(String) -> Result<()>) -> Self {
        Caprice {
            terminal: TerminalManipulator::new(),
            functor,
            buffer: String::new(),
            tokens: vec![
                "some_token".to_owned(),
                "some_token".to_owned(),
                "some_other_token".to_owned(),
                "some_other_token".to_owned(),
                "some_other_token".to_owned(),
                "some_third_thing".to_owned(),
                "some_third_thing".to_owned(),
                "some_third_thing".to_owned(),
                "simulatred".to_owned(),
                "simulatred".to_owned(),
                "sunshine".to_owned(),
                "sunshine".to_owned(),
                "sunshine".to_owned(),
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
        self.autocompleted.reset_tabbed();
        Ok(())
    }

    pub(crate) fn parse_ctrl_c(&mut self, c: char) -> Result<()> {
        if c == 'c' {
            self.terminal.exit()?;
        }
        self.autocompleted.reset_tabbed();
        Ok(())
    }

    fn parse_tab(&mut self) -> Result<()> {

        self.autocompleted.tabbed = true;

        let word_margin = 2;
        self.autocompleted.autocomplete(&self.buffer, &self.tokens);

        if self.autocompleted.get_common().len() == 0 {
            return Ok(());
        }
        self.autocompleted.incr_idx()?;
        
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

        let ydiff = (self.terminal.terminal.terminal_size().1 - self.terminal.cursor.pos().1 - 1) as i16;
        let needed_lines =   self.autocompleted.get_keywords().len()as i16 % (num_per_line - word_margin) as i16;

        if needed_lines > ydiff {
            self.terminal.terminal.scroll_up(needed_lines - ydiff - 1)?;
            self.terminal.cursor.move_up((needed_lines - ydiff - 1) as u16);
        }


        self.terminal.goto_begining_of_line();
        self.print_autocomplete_suggestions(num_per_line, Some(self.autocompleted.get_idx()))?;
        self.terminal.goto_begining_of_line();
        if let Some(keyword) = self.autocompleted.get_keywords().get(self.autocompleted.get_idx()) {
            print!("{} {}", self.prompt, keyword.clone().trim_end());
        }

        Ok(())
    }

    fn print_autocomplete_suggestions(&self,num_per_line: u16, idx: Option<usize>) -> Result<()> {
        self.terminal.save_cursor()?;
        self.terminal.goto_next_line()?;

        let mut count: u16 = 0;
        if let Some(idx) = idx {
            for (i,word) in self.autocompleted.get_keywords().iter().enumerate() {
                if i == idx {
                    print!("{}{}  ", Colored::Bg(Color::White), word);
                    print!("{}", Attribute::Reset);
                } else {
                    print!("{}  ", word);
                }
                count += 1;
                if count == num_per_line {
                    self.terminal.goto_next_line()?;
                    count = 0;
                }
            }
        } else {
            for (_,word) in self.autocompleted.get_keywords().iter().enumerate() {
                print!("{}  ", word);
                count += 1;
                if count == num_per_line {
                    self.terminal.goto_next_line()?;
                    count = 0;
                }
            }
        }
        self.terminal.restore_cursor()?;
        Ok(())
    }

    fn parse_enter(&mut self) -> Result<()> {
        if self.autocompleted.tabbed {
            if let Some(keyword) = self.autocompleted.get_keywords().get(self.autocompleted.get_idx()) {
                self.buffer = keyword.to_owned().trim_end().to_string();
            }
        }
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

        self.autocompleted.reset_tabbed();
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

        self.autocompleted.reset_tabbed();
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

        self.autocompleted.reset_tabbed();
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

impl Drop for Caprice {
    fn drop(&mut self) {
        self.terminal.flush().unwrap();
        self.terminal.disable_raw_screen().unwrap();
        // reset any possible changes to the terminal's output
        println!("{}", Attribute::Reset);
        self.terminal.exit().unwrap();
    }
}
