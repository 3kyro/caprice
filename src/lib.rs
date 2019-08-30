use crossterm::{
    cursor, input, terminal, ClearType, Color, Colored, InputEvent, KeyEvent, RawScreen,
    SyncReader, Terminal, TerminalCursor,
};

use std::io::{stdout, Stdout, Write};
use std::process::exit;

use std::collections::BTreeMap;

pub struct Flags {
    pub map: BTreeMap<String, bool>,
    commands: [&'static str; 1],
    stdin: SyncReader,
    stdout: Stdout,
    cursor: TerminalCursor,
    terminal: Terminal,
    keyword: String,
    prompt: String,
}

impl Flags {
    /// Creates a new, empty flags structure.
    /// Flags need to be inserted individually using the insert method
    pub fn new() -> Self {
        Flags {
            map: BTreeMap::new(),
            commands: ["#list"],
            stdin: input().read_sync(),
            stdout: stdout(),
            cursor: cursor(),
            terminal: terminal(),
            keyword: String::new(),
            prompt: String::from("~ "),
        }
    }

    /// Creates a new flags structure using the values
    /// from the provided BTreeMap.
    /// This method allows you to initialise the flags according to your needs
    /// rather than the default false value when using from_vec
    pub fn from_map(map: &BTreeMap<String, bool>) -> Self {
        let mut flags = Flags::new();
        flags.map = map.clone();
        flags
    }

    /// Creates a new flags structure using the tokens found
    /// in the provide vector and initialising all of them to false
    pub fn from_vec(vec: &[&str]) -> Self {
        let mut flags = Flags::new();
        for flag in vec.iter() {
            flags.map.insert(String::from(*flag), false);
        }
        flags
    }

    pub fn insert(&mut self, flag: String) {
        self.map.insert(flag, false);
    }

    /// Runs the caprice prompt once,
    pub fn run(&mut self) {
        // flush the terminal so we see the work previoulsy done
        // TODO: check where best to put it
        self.stdout.flush().unwrap();

        let trimmed = self.keyword.trim_end().to_owned();

        let tokens: Vec<&str> = self.map.keys().map(|x| x.as_str()).collect();

        if let Some(key_event) = self.stdin.next() {
            match key_event {
                InputEvent::Keyboard(KeyEvent::Char(c)) => {
                    match c {
                        '\t' => {
                            // get autocomplete results
                            let (similar, common) = autocomplete(&trimmed, &tokens);

                            // if there is a common str, print it
                            if let Some(common) = common {
                                self.cursor.move_left(self.cursor.pos().0);
                                print!("{}{}", self.prompt, common);
                                self.keyword = common.to_owned().to_string();
                            }

                            // if there are more than one keywords, print them at the bottom of the current line
                            if similar.len() > 1 {
                                // give some space for an extra line
                                if self.cursor.pos().1 == self.terminal.terminal_size().1 - 1 {
                                    self.terminal.scroll_up(1).unwrap();
                                    self.cursor.move_up(1);
                                }

                                // save self.cursor position
                                self.cursor.save_position().unwrap();

                                // goto next line
                                self.cursor.goto(0, self.cursor.pos().1 + 1).unwrap();

                                // print all the similar keywords
                                for word in similar {
                                    print!("{}{} ", Colored::Fg(Color::Green), word);
                                }

                                // erase all after self.cursor
                                self.terminal.clear(ClearType::UntilNewLine).unwrap();

                                // reset position
                                self.cursor.reset_position().unwrap();
                            } else {
                                self.terminal.clear(ClearType::FromCursorDown).unwrap();
                            }
                        }
                        // enter
                        '\r' | '\n' => {
                            // go to next line
                            self.terminal.clear(ClearType::UntilNewLine).unwrap();
                            self.terminal.clear(ClearType::FromCursorDown).unwrap();
                            println!();
                            self.cursor.move_left(self.cursor.pos().0);
                            // check if keyword is part of contents
                            if let Some(value) = self.map.get(&trimmed) {
                                let new_value = !value;
                                self.map.insert(trimmed.clone(), new_value);
                                print!("{} set to {}", trimmed, new_value);
                                println!();
                                self.cursor.move_left(self.cursor.pos().0);
                            } else if self.commands.iter().any(|&x| x == trimmed) {
                                match trimmed.as_str() {
                                    "#list" => {
                                        for token in tokens.iter() {
                                            println!("{}", token);
                                            self.cursor.move_left(self.cursor.pos().0);
                                        }
                                    }
                                    _ => return,
                                }
                            }

                            // clear keyword
                            self.keyword.clear();
                            print!("{}", self.prompt);
                        }
                        _ => {
                            if c.is_alphanumeric() || c == '#' || c == '_' {
                                // insert new char to self.keyword
                                self.keyword.push(c);
                                let trimmed = self.keyword.trim_end();

                                print!("{}", c);

                                self.print_autocompleted(&trimmed.to_owned());
                            }
                        }
                    }
                }
                InputEvent::Keyboard(KeyEvent::Backspace) => {
                    if !self.keyword.is_empty() {
                        self.keyword.pop();
                        self.cursor.move_left(1);
                        self.terminal.clear(ClearType::UntilNewLine).unwrap();
                    }
                }
                InputEvent::Keyboard(KeyEvent::Ctrl(c)) => {
                    if c == 'c' {
                        self.stdout.flush().unwrap();
                        RawScreen::disable_raw_mode().unwrap();
                        exit(exitcode::OK);
                    }
                }
                _ => {}
            }
        }
    }

    /// Initialises the terminal.
    pub fn init(&self) {
        let mut screen = RawScreen::into_raw_mode().unwrap();
        screen.disable_drop();
        print!("{}", self.prompt);
    }

    fn print_autocompleted(&self, trimmed: &str) {
        // get autocomplete results
        let tokens: Vec<&str> = self.map.keys().map(|x| x.as_str()).collect();
        let (_, common) = autocomplete(&trimmed, &tokens);

        if let Some(result) = common {
            // save current position so we can return
            self.cursor.save_position().unwrap();

            // print in grey the autocompleted part
            print!(
                "{}{}",
                Colored::Fg(Color::Rgb {
                    r: 125,
                    g: 125,
                    b: 125
                }),
                result.split_at(trimmed.len()).1
            );

            // return the self.cursor for the next loop
            self.cursor.reset_position().unwrap();
        } else {
            // clear everything left of the self.cursor
            self.terminal.clear(ClearType::UntilNewLine).unwrap();
            self.terminal.clear(ClearType::FromCursorDown).unwrap();
        }
    }
}

impl Default for Flags {
    fn default() -> Self {
        Flags::new()
    }
}

// make sure we return  from the raw mode
impl Drop for Flags {
    fn drop(&mut self) {
        RawScreen::disable_raw_mode().unwrap();
        self.terminal.exit();
    }
}

// returns the common str slice of a collection of str slices
// returns None if no common slice can be found
fn return_common_str_from_sorted_collection(collection: Vec<&str>) -> Option<&str> {
    // take the first element of the sorted list and check if the rest of the elements start with
    // if not remove last character and repeat
    if collection.is_empty() {
        // if empty there is nothing to do
        None
    } else {
        // take the first element
        let mut first = collection[0];

        for _ in 0..first.len() {
            // if all others start with it then we have found our str
            if collection.iter().all(|&x| x.starts_with(first)) {
                return Some(&(*first));
            } else {
                // else remove the last character and try again
                first = first.split_at(first.len() - 1).0;
            }
        }
        // if we tried all slices, there is no common str
        None
    }
}

// takes a word and a slice of keywords and returns the sub set of the collection that starts
// with the word and the biggest common starting str of this collection
fn autocomplete<'a>(word: &str, keywords: &'a [&str]) -> (Vec<&'a str>, Option<&'a str>) {
    let similar: Vec<&str>;

    // do not return anything until word is atleast one char long
    if word.is_empty() {
        return (Vec::with_capacity(0), None);
    }

    similar = keywords
        .iter()
        .filter(|&x| x.starts_with(word))
        .copied()
        .collect();

    (
        similar.clone(),
        return_common_str_from_sorted_collection(similar.clone()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn autocomplete_empty_input() {
        let word = "";
        let keywords = vec!["non", "important", "for", "this", "test"];
        assert_eq!(autocomplete(word, &keywords), (Vec::new(), None));

        let word = "random_word";
        let keywords: Vec<&str> = Vec::new();
        assert_eq!(autocomplete(word, &keywords), ((Vec::new(), None)));

        let word = "";
        let keywords: Vec<&str> = Vec::new();
        assert_eq!(autocomplete(word, &keywords), ((Vec::new(), None)));
    }

    #[test]
    fn autocomplete_returns_as_expected() {
        // returns correclty empty sets
        let word = "some_word";
        let keywords = vec!["non", "important", "for", "this", "test"];
        assert_eq!(autocomplete(word, &keywords), (Vec::new(), None));

        // returns correclty full sets with full word
        let word = "some_word";
        let keywords = vec![
            "some_word",
            "some_word",
            "some_word",
            "some_word",
            "some_word",
        ];
        assert_eq!(
            autocomplete(word, &keywords),
            (
                vec![
                    "some_word",
                    "some_word",
                    "some_word",
                    "some_word",
                    "some_word"
                ],
                Some("some_word")
            )
        );

        // returns correclty full sets with one or more char
        let word = "s";
        let keywords = vec![
            "some_word",
            "some_word",
            "some_word",
            "some_word",
            "some_word",
        ];
        assert_eq!(
            autocomplete(word, &keywords),
            (
                vec![
                    "some_word",
                    "some_word",
                    "some_word",
                    "some_word",
                    "some_word"
                ],
                Some("some_word")
            )
        );

        // returns correclty sets
        let word = "s";
        let keywords = vec!["some_word", "some_other_word", "none"];
        assert_eq!(
            autocomplete(word, &keywords),
            (vec!["some_word", "some_other_word"], Some("some_"))
        );

        // returns correclty sets
        let word = "some_w";
        let keywords = vec!["some_word", "some_other_word", "none"];
        assert_eq!(
            autocomplete(word, &keywords),
            (vec!["some_word"], Some("some_word"))
        );
    }
}
