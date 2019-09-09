// lets define three dtata types

/*

bash autocomplete,
find the biggest word in teh list and amke every word that long by appending spaces
the terminal should auto space them correctly
by pressing tab you just change the bg color of one of the items in the list

*/

use crossterm::{Attribute, Color, Colored};

use crate::Result;
use std::io::{Error, ErrorKind};


pub(crate) struct Autocomplete {
    keywords: Vec<String>,
    common: String,
    pub(crate) tabbed: bool,
    tabbed_idx: usize,
}

impl Autocomplete {

    pub fn new() -> Self {
        Autocomplete {
            keywords: Vec::new(),
            common: String::new(),
            tabbed: false,
            tabbed_idx: 0,
        }
    }

    pub(crate) fn get_common(&self) -> &String {
        &self.common
    }

    pub(crate) fn get_keywords(&self) -> &Vec<String> {
        &self.keywords
    }

    pub(crate) fn amortisize(&mut self) {
        // get the length of the biggest word in similar
        if let Some(max_len) = self.keywords.iter().map(|x| x.len()).max() {
            for word in self.keywords.iter_mut() {
                for _ in 0..max_len - word.len() {
                    word.push(' ');
                }
            }
        }
    }
    
    pub(crate) fn reset_tabbed(&mut self) {
        self.tabbed = false;
        if self.keywords.is_empty() {
            self.tabbed_idx = 0;
        } else {
            self.tabbed_idx = self.keywords.len() - 1;
        }
    }

    pub(crate) fn incr_idx(&mut self) -> Result<()> {
        if self.keywords.len() > 0 {
            self.tabbed_idx = (self.tabbed_idx + 1) % self.keywords.len();
            Ok(())
        } else {
            Err(Error::new(ErrorKind::InvalidData, "Invalid AUtocomplete index"))
        }
    }

    pub(crate) fn get_idx(&self) -> usize {
        self.tabbed_idx
    }

}

impl<'a> Autocomplete {
    // takes a word and a slice of keywords and returns the sub set of the collection that starts
    // with the word and the biggest common starting str of this collection (or None if this doesn't exist)
    // UPDATE!!!!!!!
    pub(crate) fn autocomplete(
        &mut self,
        word: &'a String,
        keywords: &'a Vec<String>,
    )  {
        let mut similar: Vec<String>;

        // do not return anything until word is atleast one char long
        if word.is_empty() {
            self.keywords = Vec::with_capacity(0);
            self.common = String::new();
            return;
        }

        similar = keywords
            .iter()
            .filter(|x| x.starts_with(word))
            .cloned()
            .collect();
    
        self.keywords = similar.clone();
        self.common = if let Some(common) = return_common_str_from_sorted_collection(&mut similar) {
            common
        } else {
            String::new()
        };
    }
}

// returns the common str slice of a collection of str slices
// returns None if no common slice can be found
fn return_common_str_from_sorted_collection(collection: &mut Vec<String>) -> Option<String> {
    // take the first element of the sorted list and check if the rest of the elements start with
    // if not remove last character and repeat
    let copied_collection = collection.clone();

    while let Some(first) = collection.first_mut() {
        if copied_collection
            .iter()
            .all(|x| x.starts_with(first.as_str()))
        {
            return Some(first.clone());
        } else {
            // else remove the last character and try again
            first.pop();
        }
    }
    // if we tried all slices, there is no common str
    None
}



pub(crate) fn print_same_line_autocompleted(result: String, buffer: &str) {
    // print in grey the autocompleted part
    print!(
        "{}{}{}",
        Colored::Fg(Color::Rgb {
            r: 125,
            g: 125,
            b: 125
        }),
        result.split_at(buffer.len()).1,
        Attribute::Reset
    );
}

mod tests {

    #[cfg(test)]
    use super::*;

    #[test]
    fn autocomplete_empty_input() {
        let word = "".to_owned();
        let keywords = vec![
            "non".to_owned(),
            "important".to_owned(),
            "for".to_owned(),
            "this".to_owned(),
            "test".to_owned(),
        ];
        let mut autocompleted = Autocomplete::new();
        autocompleted.autocomplete(&word, &keywords);
        assert_eq!(autocompleted.get_keywords(), &Vec::<String>::new());
        assert_eq!(autocompleted.get_common(), &String::new());


        let word = "random_word".to_owned();
        let keywords: Vec<String> = Vec::new();
        autocompleted.autocomplete(&word, &keywords);
        assert_eq!(autocompleted.get_keywords(), &Vec::<String>::new());
        assert_eq!(autocompleted.get_common(), &String::new());

        let word = "".to_owned();
        let keywords: Vec<String> = Vec::new();
        autocompleted.autocomplete(&word, &keywords);
        assert_eq!(autocompleted.get_keywords(), &Vec::<String>::new());
        assert_eq!(autocompleted.get_common(), &String::new());
    }

    #[test]
    fn autocomplete_returns_as_expected() {
        // returns correclty empty sets
        let word = "some_word".to_owned();
        let keywords = vec![
            "non".to_owned(),
            "important".to_owned(),
            "for".to_owned(),
            "this".to_owned(),
            "test".to_owned(),
        ];
        let mut autocompleted = Autocomplete::new();
        autocompleted.autocomplete(&word, &keywords);
        assert_eq!(autocompleted.get_keywords(), &Vec::<String>::new());
        assert_eq!(autocompleted.get_common(), &String::new());
        

        // returns correclty full sets with full word
        let word = "some_word".to_owned();
        let keywords = vec![
            "some_word".to_owned(),
            "some_word".to_owned(),
            "some_word".to_owned(),
            "some_word".to_owned(),
            "some_word".to_owned(),
        ];
        autocompleted.autocomplete(&word, &keywords);
        assert_eq!(autocompleted.get_keywords(), 
            &vec![
                "some_word".to_owned(),
                "some_word".to_owned(),
                "some_word".to_owned(),
                "some_word".to_owned(),
                "some_word".to_owned()
            ]
        );
        assert_eq!(autocompleted.get_common(),&"some_word".to_owned());

        // returns correclty full sets with one or more char
        let word = "s".to_owned();
        let keywords = vec![
            "some_word".to_owned(),
            "some_word".to_owned(),
            "some_word".to_owned(),
            "some_word".to_owned(),
            "some_word".to_owned(),
        ];
        autocompleted.autocomplete(&word, &keywords);
        assert_eq!(autocompleted.get_keywords(), 
            &vec![
                "some_word".to_owned(),
                "some_word".to_owned(),
                "some_word".to_owned(),
                "some_word".to_owned(),
                "some_word".to_owned()
            ]
        );
        assert_eq!(autocompleted.get_common(),&"some_word".to_owned());

        // returns correclty sets
        let word = "s".to_owned();
        let keywords = vec![
            "some_word".to_owned(),
            "some_other_word".to_owned(),
            "none".to_owned(),
        ];
        autocompleted.autocomplete(&word, &keywords);
        assert_eq!(autocompleted.get_keywords(), 
            &vec![
                "some_word".to_owned(),
                "some_other_word".to_owned(),
            ]
        );
        assert_eq!(autocompleted.get_common(),&"some_".to_owned());

        // returns correclty sets
        let word = "some_w".to_owned();
        let keywords = vec![
            "some_word".to_owned(),
            "some_other_word".to_owned(),
            "none".to_owned(),
        ];
        autocompleted.autocomplete(&word, &keywords);
        assert_eq!(autocompleted.get_keywords(), 
            &vec![
                "some_word".to_owned(),
            ]
        );
        assert_eq!(autocompleted.get_common(),&"some_word".to_owned());
    }

    #[test]
    fn amortised() {
        // normal consditions
        let vec = vec!["_a".to_owned(), "_ab".to_owned(), "_abc".to_owned()];
        let word = "_".to_owned();
        let mut autocomplete = Autocomplete::new();
        autocomplete.autocomplete(&word,&vec);
        autocomplete.amortisize();  
        assert_eq!(
            autocomplete.get_keywords(),
            &vec!["_a  ", "_ab ", "_abc"]
        );

        // similar length
        let vec = vec!["_aa".to_owned(), "_bb".to_owned(), "_cc".to_owned()];
        autocomplete.autocomplete(&word,&vec);
        autocomplete.amortisize();  
        assert_eq!(
            autocomplete.get_keywords(),
            &vec!["_aa", "_bb", "_cc"]
        );

        // empty vec
        let vec = Vec::with_capacity(0);
        autocomplete.autocomplete(&word,&vec);
        autocomplete.amortisize();  
        let return_vec: Vec<String> = Vec::with_capacity(0);
        assert_eq!(autocomplete.get_keywords(), &return_vec);
    }

    #[test]
    #[should_panic]
    fn increment_index_emty() {
        let mut autocomplete = Autocomplete::new();
        // panics with empty lists
        assert_eq!(autocomplete.incr_idx().unwrap(), ());
    }

    #[test]
    fn increment_index_wraps_around() {
        let mut autocomplete = Autocomplete::new();

        let vec = vec!["_a".to_owned(), "_ab".to_owned(), "_abc".to_owned()];
        let word = "_".to_owned();
        autocomplete.autocomplete(&word, &vec);
        autocomplete.incr_idx().unwrap();
        assert_eq!(autocomplete.get_idx(), 1);
        autocomplete.incr_idx().unwrap();
        assert_eq!(autocomplete.get_idx(), 2);
        autocomplete.incr_idx().unwrap();
        assert_eq!(autocomplete.get_idx(), 0);
        autocomplete.incr_idx().unwrap();
        assert_eq!(autocomplete.get_idx(), 1);
    }
}
