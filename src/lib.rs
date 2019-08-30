
use crossterm::{
    input, ClearType, Colored, Color, InputEvent, KeyEvent, RawScreen, cursor, terminal, Terminal, SyncReader, TerminalCursor
}; 

use std::io::{stdout, Write, Stdout};  
use std::process::exit;

use std::collections::BTreeMap;


pub struct Flags {
    pub map: BTreeMap<String, bool>,
    commands: [&'static str;1],
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
            map:  BTreeMap::new(),
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
    pub fn from_vec(vec: &Vec<&str>) -> Self {
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

        let tokens : Vec<&str> = self.map.keys().map(|x| x.as_str()).collect();

        if let Some(key_event) = self.stdin.next() {
            match key_event {
                InputEvent::Keyboard(KeyEvent::Char(c)) => {
                    match c {
                        // ctrl + c to exit
                        '\u{0003}' => {

                        },
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
                                if self.cursor.pos().1 == self.terminal.terminal_size().1 - 1  {
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
                        },
                        // enter
                        '\r' | '\n' => {
                            // go to next line
                            self.terminal.clear(ClearType::UntilNewLine).unwrap();
                            self.terminal.clear(ClearType::FromCursorDown).unwrap();
                            println!("");
                            self.cursor.move_left(self.cursor.pos().0);
                            // check if keyword is part of contents
                            if  let Some(value) = self.map.get(&trimmed) {
                                let new_value = !value;
                                self.map.insert(trimmed.clone(), new_value);
                                print!("{} set to {}", trimmed, new_value);
                                println!("");
                                self.cursor.move_left(self.cursor.pos().0);
                            } else 
                            if self.commands.iter().any(|&x| x == trimmed) {
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

                                // get autocomplete results
                                let tokens : Vec<&str> = self.map.keys().map(|x| x.as_str()).collect();
                                let (_, common) = autocomplete(&trimmed, &tokens);

                                if let Some(result) = common {
                                    // save current position so we can return
                                    self.cursor.save_position().unwrap();

                                    // print in grey the autocompleted part
                                    print!("{}{}", Colored::Fg(Color::Rgb {r: 125, g: 125, b: 125}), result.split_at(trimmed.len()).1);
                                    
                                    // return the self.cursor for the next loop
                                    self.cursor.reset_position().unwrap();
                                } else {
                                    // clear everything left of the self.cursor
                                    self.terminal.clear(ClearType::UntilNewLine).unwrap();
                                    self.terminal.clear(ClearType::FromCursorDown).unwrap();
                                }

                            }
                        }
                    }                
                },
                InputEvent::Keyboard(KeyEvent::Backspace) => {
                    if !self.keyword.is_empty() {
                        self.keyword.pop();
                        self.cursor.move_left(1);
                        self.terminal.clear(ClearType::UntilNewLine).unwrap();
                    }
                },
                InputEvent::Keyboard(KeyEvent::Ctrl(c)) => {
                    if c == 'c' {
                        self.stdout.flush().unwrap();
                        RawScreen::disable_raw_mode().unwrap();
                        exit(exitcode::OK);
                    }
                }
                _ => {
                }
            }
        }
    }

    /// Initialises the terminal.
    pub fn init(&self) {
        let mut  screen = RawScreen::into_raw_mode().unwrap();
        screen.disable_drop();
        print!("{}", self.prompt);
    }
} 

// make sure we return  from the raw mode 
impl Drop for Flags {
    fn drop(&mut self) {
        RawScreen::disable_raw_mode().unwrap();
        self.terminal.exit();
    }
}

// returns the common str slice of a collection od strs
// returns None if no commin slice can be found
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
                return Some(first.clone())
                
            } else {
                // else remove the last character and try again
                first = first.split_at(first.len() - 1).0; 
            }
        }
        // if we tried all slices, there is no common str
        None
    }
    
}

// takes a word and a list of keywords and returns the sub set of the collection that starts
// with the word and the biggest common starting str of this collection
fn autocomplete<'a>(word: &str, keywords: &'a Vec<&str>) -> (Vec<&'a str>, Option<&'a str>) {

    let similar: Vec<&str>;

    similar = keywords.iter().filter(|&x| x.starts_with(word)).map(|x| *x).collect();

    (similar.clone(), return_common_str_from_sorted_collection(similar.clone()))

}