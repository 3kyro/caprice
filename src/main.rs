use crossterm::{
    input, ClearType, Colored, Color, InputEvent, KeyEvent
}; 

use crossterm::RawScreen;
use crossterm::cursor;
use crossterm::terminal;

use std::io::{stdout, Write};  
use std::process::exit;

use std::collections::BTreeMap;


fn main() {

    let mut map: BTreeMap<String, bool> = BTreeMap::new();

    map.insert("simulate_bms".to_owned(), false);
    map.insert("simulate_battery".to_owned(), false);
    map.insert("three".to_owned(), false);
    

    let input = input();
    let mut stdout = stdout();
    let mut cursor = cursor();
    let terminal = terminal();

    let mut stdin = input.read_sync();

    // let mut tokens = vec!["simulate_bms", "simulate_battery", "ahree"];
    let commands = vec!["#list"];
    

    println!("Welcome to caprice runtime.");
    println!("Autocomplete by pressing tab,");
    println!("type #list to get a list of availiable options,");
    println!("ctrl+c to exit.");

    
    let _screen = RawScreen::into_raw_mode().unwrap();
    
    print!(":");

    let mut keyword : String = "".to_owned();
    // tokens.sort();
    loop {
        // flush the terminal so we see the work previoulsy done
        // TODO: check where best to put it
        stdout.flush().unwrap();

        let trimmed = keyword.trim_end().to_owned();

        let tokens : Vec<&str> = map.keys().map(|x| x.as_str()).collect();

        if let Some(key_event) = stdin.next() {
            match key_event {
                InputEvent::Keyboard(KeyEvent::Char(c)) => {
                    process_char(c, trimmed, tokens, &commands, &mut keyword, &mut map)
                },
                InputEvent::Keyboard(KeyEvent::Backspace) => {
                    if !keyword.is_empty() {
                        keyword.pop();
                        cursor.move_left(1);
                        terminal.clear(ClearType::UntilNewLine).unwrap();
                    }
                },
                _ => std::process::exit(1),
            }
        }
    }
}

fn process_char(c: char, trimmed: String, tokens: Vec<&str>, commands : &Vec<&str>, keyword : &mut String, map:&mut BTreeMap<String, bool>) {
    let mut cursor = cursor();
    let terminal = terminal();

    match c {
        // ctrl + c to exit
        '\u{3}' => {
            RawScreen::disable_raw_mode().unwrap();
            exit(exitcode::OK);
        },
        '\t' => {
            // get autocomplete results
            let (similar, common) = autocomplete(&trimmed, &tokens);

            // if there is a common str, print it
            if let Some(common) = common {
                cursor.move_left(cursor.pos().0);
                print!(":{}", common);
                *keyword = common.to_owned().to_string();

            }

            // if there are more than one keywords, print them at the bottom of the current line
            if similar.len() > 1 {

                // give some space for an extra line
                if cursor.pos().1 == terminal.terminal_size().1 - 1  {
                    terminal.scroll_up(1).unwrap();
                    cursor.move_up(1);
                }

                // save cursor position
                cursor.save_position().unwrap();

                // goto next line
                cursor.goto(0, cursor.pos().1 + 1).unwrap();

                // print all the similar keywords
                for word in similar {
                    print!("{}{} ", Colored::Fg(Color::Green), word);
                }

                // erase all after cursor
                terminal.clear(ClearType::UntilNewLine).unwrap();

                // reset position
                cursor.reset_position().unwrap();
            } else {
                terminal.clear(ClearType::FromCursorDown).unwrap();
            }
        },
        '\r' | '\n' => {
            // go to next line
            terminal.clear(ClearType::UntilNewLine).unwrap();
            terminal.clear(ClearType::FromCursorDown).unwrap();
            println!("");
            cursor.move_left(cursor.pos().0);
            // check if keyword is part of contents
            if  let Some(value) = map.get(&trimmed) {
                let new_value = !value;
                map.insert(trimmed.clone(), new_value);
                print!("{} set to {}", trimmed, new_value);
                println!("");
                cursor.move_left(cursor.pos().0);
            } else 
            if commands.iter().any(|&x| x == trimmed) {
                match trimmed.as_str() {
                    "#list" => {
                        
                        for token in tokens.iter() {
                            println!("{}", token);
                            cursor.move_left(cursor.pos().0);
                        }
                    }
                    _ => return,
                }
            }

            // clear keyword
            keyword.clear();
            print!(":");
        }
        _ => {
            if c.is_alphanumeric() {
                // insert new char to keyword
                keyword.push(c);
                let trimmed = keyword.trim_end();

                print!("{}", c);

                // get autocomplete results
                let tokens : Vec<&str> = map.keys().map(|x| x.as_str()).collect();
                let (_, common) = autocomplete(&trimmed, &tokens);

                if let Some(result) = common {
                    // save current position so we can return
                    cursor.save_position().unwrap();

                    // print in grey the autocompleted part
                    print!("{}{}", Colored::Fg(Color::Rgb {r: 125, g: 125, b: 125}), result.split_at(trimmed.len()).1);
                    
                    // return the cursor for the next loop
                    cursor.reset_position().unwrap();
                } else {
                    // clear everything left of the cursor
                    terminal.clear(ClearType::UntilNewLine).unwrap();
                    terminal.clear(ClearType::FromCursorDown).unwrap();
                }

            }
        }
    
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
