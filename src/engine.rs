use std::io::stdout;

use crate::autocomplete::Autocomplete;
use crate::error::Result;
use crate::scanner::{Scanner, TokenType};
use crate::terminal::Terminal;
use crate::theme::{Theme, DEFAULT_THEME};
use crossterm::execute;
use crossterm::style::{Attribute, Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use regex::Regex;

#[derive(Debug)]
pub(crate) struct Executor {
    pub(crate) terminal: Terminal,
    pub(crate) scanner: Scanner,
    autocomplete: Autocomplete,
    keywords: Vec<String>,
    commands: Vec<String>,
    pub(crate) prompt: &'static str,
    pub(crate) theme: Theme,
    pub(crate) alternate_screen: bool,
}

impl Executor {
    pub(crate) fn new() -> Self {
        Executor {
            terminal: Terminal::new(),
            scanner: Scanner::new(),
            autocomplete: Autocomplete::new(),
            keywords: Vec::new(),
            commands: vec!["/list".to_owned()],
            prompt: "!:",
            theme: DEFAULT_THEME,
            alternate_screen: false,
        }
    }

    fn print_prompt(&self) -> Result<()> {
        Ok(execute!(
            stdout(),
            SetForegroundColor(self.theme.prompt_color),
            Print(self.prompt),
            Print(" "),
            ResetColor,
        )?)
    }

    // Block until the next key event.
    pub(crate) fn get_next_key_event(&mut self) -> Result<Option<String>> {
        self.terminal.flush()?;
        match self.scanner.scan(self.terminal.next_key_event()?) {
            TokenType::Token(token) => self.exec_token(token),
            TokenType::BackSpace => self.exec_backspace(),
            TokenType::Tab(buffer) => self.exec_tab(buffer),
            TokenType::Continue(buffer) => self.exec_valid_char(buffer),
            TokenType::Exit => self.exec_exit(),
            TokenType::None => Ok(None),
        }
    }

    pub(crate) fn set_keywords(&mut self, keywords: Vec<String>) {
        let mut valid_keywords = get_valid_keywords(keywords);
        valid_keywords.sort();
        self.keywords = valid_keywords
    }

    pub(crate) fn reset_prompt(&mut self) -> Result<()> {
        self.print_prompt()?;
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
                .contains(&token.split(' ').next().unwrap_or_default().to_string())
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
            let mut updated_buffer = buffer;
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

        self.autocomplete.amortize();

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

        let idx = self.autocomplete.tab_idx;

        let mut count: u16 = 0;

        for (i, word) in self.autocomplete.get_keywords().iter().enumerate() {
            // highlight current selection
            if i == idx {
                print!(
                    "{}{}{}  {}",
                    SetBackgroundColor(self.theme.suggestion_bg),
                    SetForegroundColor(self.theme.suggestion_fg),
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
            self.print_prompt()?;
            print!("{}", keyword);
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

        self.autocomplete.print_same_line_autocompleted(
            self.theme.autocomplete_color,
            &buffer,
            &self.terminal,
        )?;

        self.autocomplete.reset_tabbed();
        Ok(None)
    }

    pub fn print_msg(&mut self, msg: &str) -> Result<()> {
        msg.lines().map(|msg| {
            print!("{}", msg);
            self.terminal.goto_next_line()
        }).collect::<Result<()>>()
    }
}

fn get_valid_keywords(keywords: Vec<String>) -> Vec<String> {
    let re = &Regex::new(r"^[_a-zA-Z][A-Za-z_0-9]*$").unwrap();
    keywords
        .into_iter()
        .filter(|keyword| re.is_match(keyword))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_keywords() {
        let empty_keywords: Vec<String> = Vec::new();
        let filtered = get_valid_keywords(empty_keywords);
        assert!(filtered.is_empty());

        let empty_string_keywords: Vec<String> = vec!["".to_owned()];
        let filtered = get_valid_keywords(empty_string_keywords);
        assert!(filtered.is_empty());

        let valid_keywords: Vec<String> =
            vec!["one".to_owned(), "_two".to_owned(), "thr3ee".to_owned()];
        let filtered = get_valid_keywords(valid_keywords.clone());
        assert_eq!(valid_keywords, filtered);

        let mixed_keywords: Vec<String> = vec![
            "9four".to_owned(),
            "".to_owned(),
            "invalid#symbol".to_owned(),
            "one".to_owned(),
            "2".to_owned(),
            "_two".to_owned(),
            "thr3ee".to_owned(),
        ];
        let filtered = get_valid_keywords(mixed_keywords);
        assert_eq!(valid_keywords, filtered);
    }
}
