mod terminal_manipulator;

use crate::Result;
use super::autocomplete::*;
use crossterm::{InputEvent, KeyEvent, Attribute, Colored, Color};
use terminal_manipulator::*;
use std::io::{Error, ErrorKind};

pub struct Caprice<'a> {
    terminal: TerminalManipulator,
    callback: Option<Box<dyn 'a + FnMut(String)>>,
    buffer: String,
    tokens: Vec<String>,
    commands: Vec<String>,
    prompt: String,
    autocompleted: Autocomplete,
}


impl<'a> Caprice<'a> {
    
    pub fn set_callback<CB: 'a + FnMut(String)>(&mut self, functor: CB) {
        self.callback = Some(Box::new(functor));
    }
    /// Creates a new Caprice object
    pub fn new() -> Self {
        Caprice {
            terminal: TerminalManipulator::new(),
            callback: None,
            buffer: String::new(),
            tokens: Vec::with_capacity(0),
            commands: vec!["#list".to_owned()],
            prompt: "➜".to_owned(),
            autocompleted: Autocomplete::new(),
        }
    }

    /// Sets the current active tokens for the parser
    /// 
    /// # Example
    /// ```
    /// use caprice::Caprice;
    /// let mut caprice = Caprice::new(functor);
    /// 
    /// // set some tokens 
    /// caprice.set(&vec![
    ///    "some_token".to_owned(),
    ///    "some_other_token".to_owned(),
    ///    "none".to_owned(),
    /// ]);
    pub fn set_tokens(&mut self, tokens: &Vec<String>) {
        self.tokens = tokens.clone();
    }

 

    /// Prepares the terminal for parsing initilaizing it either in RawMode or AlternateMode
    pub fn init(&mut self, alternate: bool) -> Result<()> {
        if alternate {
            self.terminal.enable_alternate_screen()?;
        } else {
            self.terminal.enable_raw_screen()?;
        }

        self.reset_prompt()?;

        Ok(())
    }

    /// Sets the prompt displayed while the caprice parser is running
    /// 
    /// ## Note
    /// This method __will not__ check for the length of the provided prompt,
    /// nor if this prompt can be correctly displayed in all supported
    /// terminals.
    /// 
    ///  # Example
    /// caprice.set_prompt("λ:");
    pub fn set_prompt(&mut self, prompt: &str) {
        self.prompt = prompt.to_owned();
    }

    /// Caprice internally is using Crossterms Rawmode for terminal manipulation.
    /// In order for the process to exit correcktly, cleaning up all changes made
    /// to the current terminal, a standard process::exit() procedure cannot be used.
    /// Instead parse will return a Error::new(ErrorKind::Interrupted, "Program Exit"),
    /// which the calling funxtion should interpret as a stop command
    /// 
    /// # Example
    /// ```
    /// loop {
    ///     // ignoring possible token return
    ///     if let Ok(_) = caprice_instance.parse() {}
    ///     else { 
    ///         break 
    ///     }
    /// }
    pub fn parse(&mut self) -> Result<Option<String>> {
        self.terminal.flush()?;

        if let Some(input_event) = self.terminal.next_key_event() {
            match input_event {
                InputEvent::Keyboard(KeyEvent::Char(c)) => {
                    return self.parse_char(c)
                }
                InputEvent::Keyboard(KeyEvent::Backspace) => {
                    self.parse_backspace()?
                }
                InputEvent::Keyboard(KeyEvent::Ctrl(c)) => {
                    self.parse_ctrl_c(c)?;
                }
                _ => { return Ok(None)}
            }
        }
        Ok(None)
    }

    
    fn parse_char(&mut self, c: char) -> Result<Option<String>> {
        match c {
            '\t' => self.parse_tab()?,
            '\r' | '\n' => return self.parse_enter(),
            _ => self.parse_valid_char(c)?,
        };
        Ok(None)
    }

    fn parse_backspace(&mut self) -> Result<()> {
        self.autocompleted.set_buffer(&mut self.buffer);
        
        if !self.buffer.is_empty() {
            self.buffer.pop();
            self.terminal.backspace()?;
        }
        self.autocompleted.reset_tabbed();
        Ok(())
    }

    // Returns an std::io::Error to signal user exit command 
    // since windows handles the ctrl+c combination indepedently
    // the exit signal will be sent with ctrl+q on windows  
    fn parse_ctrl_c(&mut self, c: char) -> Result<()> {
        #[cfg(windows)]
        let exit_char = 'q';
        #[cfg(unix)]
        let exit_char = 'c';
        
        if c == exit_char {
            self.terminal.clear_from_cursor().unwrap();
            self.terminal.flush().unwrap();
            self.terminal.disable_raw_screen().unwrap();
            return Err(Error::new(ErrorKind::Interrupted, "Program Exit")); 
        }

        // reset autocompleted state
        self.autocompleted.reset_tabbed();
        
        Ok(())
    }

    fn parse_tab(&mut self) -> Result<()> {
        // set autocompleted state
        self.autocompleted.tabbed = true;

        // a margin left on the right of the terminal
        let word_margin = 1;
        
        // update the autocompleted state
        self.autocompleted.autocomplete(&self.buffer, &self.tokens);


        // return if there are no autocmplete suggestions
        if self.autocompleted.get_common().is_empty() {
            return Ok(());
        }

        self.autocompleted.amortisize();

        self.autocompleted.incr_idx()?;
        

        let mut num_per_line: u16;
        let word_separation: u16 = 2;


        // get num of words that fit in one line
        if let Some(first) = self.autocompleted.get_keywords().get(0) {
            num_per_line = self.terminal.size().0 / (first.len() as u16 + word_separation);
            if num_per_line > word_margin {
                num_per_line -= word_margin;
            }
        } 
        else {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid autocompleted result"));
        }

        // get vertical distance of current line to end of terminal
        let ydiff = (self.terminal.size().1 - (self.terminal.get_cursor_pos().1 + 1)) as i16;

        // get required number of lines to print autocomplete suggestions
        let needed_lines = (self.autocompleted.get_keywords().len() as f32 / (num_per_line) as f32).ceil() as i16;

        // if we need space to display the suggestions, scroll the terminal upwards
        if ydiff < needed_lines {
            self.terminal.scroll_up(needed_lines - ydiff)?;
        }

        // print autocomplete suggestions
        self.terminal.goto_begining_of_line();
        self.print_autocomplete_suggestions(num_per_line, self.autocompleted.get_idx())?;
        self.terminal.goto_begining_of_line();
        if let Some(keyword) = self.autocompleted.get_keywords().get(self.autocompleted.get_idx()) {
            print!("{} {}", self.prompt, keyword.clone().trim_end());
        }

        Ok(())
    }

    fn print_autocomplete_suggestions(&self,num_per_line: u16, idx: usize) -> Result<()> {
        self.terminal.save_cursor()?;
        self.terminal.goto_next_line()?;

        let mut count: u16 = 0;
        for (i,word) in self.autocompleted.get_keywords().iter().enumerate() {
            if i == idx {
                print!("{}{}  {}", Colored::Bg(Color::Cyan), word, Attribute::Reset);
            } else {
                print!("{}  ", word);
            }
            count += 1;
            if count == num_per_line {
                self.terminal.goto_next_line()?;
                count = 0;
            }
        }
        self.terminal.restore_cursor()?;
        Ok(())
    }

    fn parse_enter(&mut self) -> Result<Option<String>> {
        
        self.autocompleted.set_buffer(&mut self.buffer);

        if self.tokens.contains(&self.buffer) {

            if let Some(functor) = &mut self.callback {
                (functor)(self.buffer.clone());
            }
            
            self.terminal.goto_begining_of_line();
            let rtn = self.buffer.clone();

            self.reset_prompt()?;
            
            return Ok(Some(rtn))
            
        } else if self.commands.contains(&self.buffer) {
            self.parse_command(&self.buffer.clone())?;
            self.terminal.goto_begining_of_line();
            self.reset_prompt()?;

        } else {
            self.terminal.goto_next_line()?;
            self.reset_prompt()?;

        }

        Ok(None)
    }

    fn reset_prompt(&mut self) -> Result<()> {
        self.buffer.clear();
        print!("{} ", self.prompt);
        self.terminal.clear_from_cursor()?;
        self.autocompleted.reset_tabbed();

        Ok(())
    }

    fn parse_command(&mut self, command: &String) -> Result<()> {
        
        self.autocompleted.set_buffer(&mut self.buffer);

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

        self.autocompleted.set_buffer(&mut self.buffer);

        if c.is_alphanumeric() || c == '#' {
            // insert new char to self.keyword
            self.buffer.push(c);
            self.buffer = self.buffer.trim_end().to_string();

            print!("{}", c);

            self.print_autocompleted()?
        }

        self.autocompleted.reset_tabbed();
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

/// Ensures the process exits gracefully, returning the terminal to its 
/// original state
impl<'a> Drop for Caprice<'a> {
    fn drop(&mut self) {
        self.terminal.clear_from_cursor().unwrap();
        self.terminal.flush().unwrap();
        self.terminal.disable_raw_screen().unwrap();
        // reset terminal attributes
        println!("{}", Attribute::Reset);
    }
}
