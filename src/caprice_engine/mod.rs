use crate::caprice_autocomplete::Autocomplete;
use crate::caprice_error::Result;
use crate::caprice_scanner::{Scanner, TokenType};
use crate::caprice_terminal::TerminalManipulator;
use crossterm::style::{Attribute, Color, SetBackgroundColor, SetForegroundColor};

#[derive(Debug)]
pub(crate) struct Executor {
    pub(crate) terminal: TerminalManipulator,
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
            commands: vec!["/list".to_owned()],
            prompt: "!:".to_owned(),
        }
    }

    pub(crate) fn poll(&mut self) -> Result<Option<String>> {
        self.terminal.flush()?;
        if let Some(input_event) = self.terminal.next_key_event()? {
            match self.scanner.scan(input_event) {
                TokenType::Token(token) => self.exec_token(token),
                TokenType::BackSpace => self.exec_backspace(),
                TokenType::Tab(buffer) => self.exec_tab(buffer),
                TokenType::Continue(buffer) => self.exec_valid_char(buffer),
                TokenType::Exit => self.exec_exit(),
                TokenType::None => Ok(None),
            }
        } else {
            Ok(None)
        }
    }

    fn get_alphabetic_keywords(keywords: &[String]) -> Vec<String> {
        keywords
            .iter()
            .filter(|keyword| {
                if let Some(head) = keyword.chars().take(1).next() {
                    head.is_alphabetic()
                } else {
                    false
                }
            })
            .map(|s| s.clone())
            .collect()
    }

    pub(crate) fn set_keywords(&mut self, keywords: &[String]) {
        self.keywords = Executor::get_alphabetic_keywords(keywords);
        self.keywords.sort();
    }

    pub(crate) fn set_prompt(&mut self, prompt: &str) {
        self.prompt = prompt.to_owned();
    }

    pub(crate) fn reset_prompt(&mut self) -> Result<()> {
        print!("{} ", self.prompt);
        self.clear_prompt()
    }

    pub(crate) fn clear_prompt(&mut self) -> Result<()> {
        self.terminal.clear_from_cursor()?;
        self.autocomplete.reset_tabbed();
        Ok(())
    }

    fn exec_token(&mut self, token: String) -> Result<Option<String>> {
        // if tab suggestions are active then we stop it
        if let Some(buffer) = self.autocomplete.get_current_tabbed_autocomplete() {
            self.terminal.clear_from_cursor()?;
            self.autocomplete.reset_tabbed();
            self.scanner.update_buffer(buffer);
            return Ok(None);
        }

        // We've committed to this input, clear the scanner
        self.scanner.clear_buffer();

        if self.keywords.contains(&token)
            || self
                .keywords
                .contains(&token.split(" ").next().unwrap_or_default().to_string())
        {
            self.terminal.goto_next_line()?;
            self.clear_prompt()?;
            return Ok(Some(token));
        } else if self.commands.contains(&token) {
            self.exec_command(token)?;
            self.terminal.goto_beginning_of_line()?;
            self.reset_prompt()?;
        } else {
            self.terminal.goto_next_line()?;
            self.reset_prompt()?;
        }

        Ok(None)
    }

    fn exec_command(&mut self, command: String) -> Result<()> {
        if command == "/list" {
            self.terminal.goto_next_line()?;
            for token in self.keywords.iter() {
                print!("{}", token);
                self.terminal.goto_next_line()?;
            }
        }

        self.autocomplete.reset_tabbed();
        Ok(())
    }

    pub(crate) fn exec_exit(&mut self) -> Result<Option<String>> {
        self.terminal.clear_from_cursor()?;
        self.terminal.flush()?;
        self.terminal.exit();
        Ok(None)
    }

    fn exec_backspace(&mut self) -> Result<Option<String>> {
        if let Some(buffer) = self.autocomplete.get_current_tabbed_autocomplete() {
            let mut updated_buffer = buffer.clone();
            updated_buffer.pop();
            self.scanner.update_buffer(updated_buffer);
        }

        self.terminal.backspace()?;

        self.autocomplete.reset_tabbed();
        self.terminal.clear_from_cursor()?;
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

        self.terminal.goto_beginning_of_line()?;
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
        self.terminal.goto_beginning_of_line()?;

        if let Some(keyword) = self.autocomplete.get_keywords().get(idx) {
            let keyword = keyword.trim_end();
            print!("{} {}", self.prompt, keyword);
        };
        Ok(())
    }

    fn exec_valid_char(&mut self, buffer: String) -> Result<Option<String>> {
        let origin_buffer_char = buffer.clone().pop();

        if let Some(buffer) = self.autocomplete.get_current_tabbed_autocomplete() {
            if origin_buffer_char.is_some() {
                self.scanner
                    .update_buffer(format!("{}{}", buffer, origin_buffer_char.unwrap()));
            } else {
                self.scanner.update_buffer(buffer);
            }
        }

        if origin_buffer_char.is_some() {
            print!("{}", origin_buffer_char.unwrap());
        }

        self.autocomplete.update(&buffer, &self.keywords);

        self.autocomplete
            .print_same_line_autocompleted(&buffer, &self.terminal)?;

        self.autocomplete.reset_tabbed();
        Ok(None)
    }

    pub fn print_msg(&mut self, msg: String) -> Result<()> {
        print!("{}", msg);
        self.terminal.goto_next_line()?;
        self.reset_prompt()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_keywords() {
        let empty_keywords: Vec<String> = Vec::new();
        let filtered = Executor::get_alphabetic_keywords(&empty_keywords);
        assert!(filtered.is_empty());

        let empty_string_keywords: Vec<String> = vec!["".to_owned()];
        let filtered = Executor::get_alphabetic_keywords(&empty_string_keywords);
        assert!(filtered.is_empty());

        let alphabetic_keywords: Vec<String> =
            vec!["one".to_owned(), "two".to_owned(), "three".to_owned()];
        let filtered = Executor::get_alphabetic_keywords(&alphabetic_keywords);
        assert_eq!(alphabetic_keywords, filtered);

        let mixed_keywords: Vec<String> = vec![
            "".to_owned(),
            "one".to_owned(),
            "2".to_owned(),
            "two".to_owned(),
            "three".to_owned(),
            "_four".to_owned(),
        ];
        let filtered = Executor::get_alphabetic_keywords(&mixed_keywords);
        assert_eq!(alphabetic_keywords, filtered);
    }
}
