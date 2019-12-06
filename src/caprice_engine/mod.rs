use crate::caprice_autocomplete::Autocomplete;
use crate::caprice_scanner::{Scanner, TokenType};
use crate::caprice_terminal::TerminalManipulator;
use anyhow::Result;

use crossterm::style::{Attribute, Color, SetBackgroundColor, SetForegroundColor};
pub(crate) struct Executor {
    terminal: TerminalManipulator,
    pub(crate) scanner: Scanner,
    autocomplete: Autocomplete,
    keywords: Vec<String>,
    commands: Vec<String>,
    prompt: String,
}

impl Executor {
    pub(crate) fn new() -> Self {
        Executor {
            terminal: TerminalManipulator::new(),
            scanner: Scanner::new(),
            autocomplete: Autocomplete::new(),
            keywords: Vec::new(),
            commands: vec!["#list".to_owned()],
            prompt: "!:".to_owned(),
        }
    }

    pub(crate) fn poll(&mut self) -> Result<Option<String>> {
        self.terminal.flush()?;

        if let Some(input_event) = self.terminal.next_key_event() {
            match self.scanner.scan(input_event) {
                TokenType::Token(token) => return self.exec_token(token),
                TokenType::BackSpace => return self.exec_backspace(),
                TokenType::Tab(buffer) => return self.exec_tab(buffer),
                TokenType::Continue(buffer) => return self.exec_valid_char(buffer),
                TokenType::Exit => self.exec_exit(),
                TokenType::None => return Ok(None),
            }
        } else {
            Ok(None)
        }
    }

    pub(crate) fn set_keywords(&mut self, keywords: &Vec<String>) {
        self.keywords = keywords.clone();
        self.keywords.sort();
    }

    pub(crate) fn set_prompt(&mut self, prompt: &str) {
        self.prompt = prompt.to_owned();
    }

    pub(crate) fn reset_prompt(&mut self) -> Result<()> {
        print!("{} ", self.prompt);
        self.terminal.clear_from_cursor()?;
        self.autocomplete.reset_tabbed();
        Ok(())
    }

    fn exec_token(&mut self, mut token: String) -> Result<Option<String>> {
        // if tab suggestions are active, ignore the scanner's token
        // and use the autocompleted one
        if let Some(buffer) = self.autocomplete.get_current_tabbed_autocomplete() {
            token = buffer;
        }

        if self.keywords.contains(&token) {
            self.terminal.goto_next_line()?;
            self.reset_prompt()?;
            return Ok(Some(token));
        } else if self.commands.contains(&token) {
            self.exec_command(token)?;
            self.terminal.goto_begining_of_line()?;
            self.reset_prompt()?;
        } else {
            self.terminal.goto_next_line()?;
            self.reset_prompt()?;
        }

        Ok(None)
    }

    fn exec_command(&mut self, command: String) -> Result<()> {
        if let Some(buffer) = self.autocomplete.get_current_tabbed_autocomplete() {
            self.scanner.update_buffer(buffer);
        }

        if command == "#list" {
            self.terminal.goto_next_line()?;
            for token in self.keywords.iter() {
                println!("{}", token);
                self.terminal.goto_begining_of_line()?;
            }
        }

        self.autocomplete.reset_tabbed();
        Ok(())
    }

    pub(crate) fn exec_exit(&mut self) -> Result<Option<String>> {
        self.terminal.clear_from_cursor()?;
        self.terminal.flush()?;
        self.terminal.disable_raw_screen()?;
        self.terminal.exit();
        Ok(None)
    }

    fn exec_backspace(&mut self) -> Result<Option<String>> {
        if let Some(buffer) = self.autocomplete.get_current_tabbed_autocomplete() {
            self.scanner.update_buffer(buffer);
        }

        self.terminal.backspace()?;

        self.autocomplete.reset_tabbed();
        Ok(None)
    }

    fn exec_tab(&mut self, buffer: String) -> Result<Option<String>> {
        // set autocompleted state
        self.autocomplete.tabbed = true;

        // update the autocompleted state
        self.autocomplete.update(&buffer, &self.keywords);

        // return if there are no autocomplete suggestions
        if self.autocomplete.get_common().is_empty() {
            Ok(None)
        } else {
            // print autocomplete suggestions
            self.print_autocomplete_suggestions()?;
            Ok(None)
        }
    }

    fn print_autocomplete_suggestions(&mut self) -> Result<()> {
        // a margin left on the right of the terminal
        let word_margin = 1;
        // spaces between each printed suggestion
        let word_separation: u16 = 2;

        let mut num_per_line: u16;

        self.autocomplete.amortisize();

        self.autocomplete.incr_idx();

        // get num of words that fit in one line
        if let Some(first) = self.autocomplete.get_keywords().get(0) {
            num_per_line = self.terminal.size().0 / (first.len() as u16 + word_separation);
            if num_per_line > word_margin {
                num_per_line -= word_margin;
            }
        } else {
            num_per_line = 0;
        }

        // get vertical distance of current line to end of terminal
        let distance_to_end = self.terminal.size().1 - (self.terminal.get_cursor_pos().1 + 1);

        // get required number of lines to print autocomplete suggestions
        let needed_lines =
            (self.autocomplete.get_keywords().len() as f32 / (num_per_line) as f32).ceil() as u16;

        // if we need space to display the suggestions, scroll the terminal up
        if distance_to_end < needed_lines {
            self.terminal.scroll_up(needed_lines - distance_to_end)?;
        }

        self.terminal.goto_begining_of_line()?;
        self.terminal.save_cursor()?;
        self.terminal.goto_next_line()?;

        let idx = self.autocomplete.get_idx();

        let mut count: u16 = 0;

        for (i, word) in self.autocomplete.get_keywords().iter().enumerate() {
            // highlight current selection
            if i == idx {
                print!(
                    "{}{}{}  {}",
                    SetBackgroundColor(Color::Grey),
                    SetForegroundColor(Color::Black),
                    word,
                    Attribute::Reset
                );
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
        self.terminal.goto_begining_of_line()?;

        if let Some(keyword) = self.autocomplete.get_keywords().get(idx) {
            let keyword = keyword.trim_end();
            print!("{} {}", self.prompt, keyword);
        };
        Ok(())
    }

    fn exec_valid_char(&mut self, buffer: String) -> Result<Option<String>> {
        if let Some(buffer) = self.autocomplete.get_current_tabbed_autocomplete() {
            self.scanner.update_buffer(buffer);
        }

        if let Some(c) = buffer.clone().pop() {
            print!("{}", c);
        }

        self.autocomplete.update(&buffer, &self.keywords);

        self.autocomplete
            .print_same_line_autocompleted(&buffer, &self.terminal)?;

        self.autocomplete.reset_tabbed();
        Ok(None)
    }

    pub fn print_msg(&self, msg: String) -> Result<()> {
        self.terminal.goto_next_line()?;
        print!("{}", msg);
        self.terminal.goto_next_line()?;
        Ok(())
    }
}
