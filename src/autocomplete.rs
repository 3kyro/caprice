use crate::error::Result;
use crate::terminal::Terminal;
use crossterm::style::{Attribute, Color, SetForegroundColor};

#[derive(Debug)]
pub(crate) struct Autocomplete {
    keywords: Vec<String>,
    common: String,
    pub(crate) tabbed: bool,
    pub(crate) tab_idx: usize,
}

impl Autocomplete {
    pub fn new() -> Self {
        Autocomplete {
            keywords: Vec::new(),
            common: String::new(),
            tabbed: false,
            tab_idx: 0,
        }
    }

    pub(crate) fn get_common(&self) -> &String {
        &self.common
    }

    pub(crate) fn get_keywords(&self) -> &Vec<String> {
        &self.keywords
    }

    pub(crate) fn amortize(&mut self) {
        // get the length of the longest word in similar
        if let Some(max_len) = self.keywords.iter().map(|x| x.len()).max() {
            // amortise the length of every keyword to the longest one
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
            self.tab_idx = 0;
        } else {
            self.tab_idx = self.keywords.len() - 1;
        }
    }

    // Increments the index pointing to the current active autocomplete suggestion,
    // wrapping around when necessary
    pub(crate) fn incr_idx(&mut self) {
        if !self.keywords.is_empty() {
            self.tab_idx = (self.tab_idx + 1) % self.keywords.len();
        }
    }
}

impl<'a> Autocomplete {
    // takes a word and a slice of keywords and returns the sub set of the collection that starts
    // with the word and the biggest common starting str of this collection (or None if this doesn't exist)
    pub(crate) fn update(&mut self, word: &'a str, keywords: &'a [String]) {
        // do not return anything until word is at least one char long
        if word.is_empty() {
            self.keywords = Vec::with_capacity(0);
            self.common = String::new();
            return;
        }

        let mut similar: Vec<String> = keywords
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

    pub(crate) fn get_current_tabbed_autocomplete(&self) -> Option<String> {
        if self.tabbed {
            self.keywords
                .get(self.tab_idx)
                .map(|keyword| keyword.clone().trim_end().to_string())
        } else {
            None
        }
    }

    // displays an autocomplete suggestion
    pub(crate) fn print_same_line_autocompleted(
        &self,
        color: Color,
        buffer: &str,
        terminal: &Terminal,
    ) -> Result<()> {
        if !self.common.is_empty() {
            terminal.save_cursor()?;

            // print in DarkGreen the autocompleted part
            print!(
                "{}{}{}",
                SetForegroundColor(color),
                self.common.split_at(buffer.len()).1,
                Attribute::Reset
            );

            terminal.restore_cursor()?;
        } else {
            // clear everything left of the cursor
            terminal.clear_from_cursor()?;
        }
        Ok(())
    }
}

// returns the common str slice of a collection of str slices
// returns None if no common slice can be found
fn return_common_str_from_sorted_collection(collection: &mut [String]) -> Option<String> {
    // take the first element of the sorted list and check if the rest of the elements start with
    // if not remove last character and repeat
    let copied_collection = collection.to_owned();

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
        autocompleted.update(&word, &keywords);
        assert_eq!(autocompleted.get_keywords(), &Vec::<String>::new());
        assert_eq!(autocompleted.get_common(), &String::new());

        let word = "random_word".to_owned();
        let keywords: Vec<String> = Vec::new();
        autocompleted.update(&word, &keywords);
        assert_eq!(autocompleted.get_keywords(), &Vec::<String>::new());
        assert_eq!(autocompleted.get_common(), &String::new());

        let word = "".to_owned();
        let keywords: Vec<String> = Vec::new();
        autocompleted.update(&word, &keywords);
        assert_eq!(autocompleted.get_keywords(), &Vec::<String>::new());
        assert_eq!(autocompleted.get_common(), &String::new());
    }

    #[test]
    fn autocomplete_returns_as_expected() {
        // correctly returns empty sets
        let word = "some_word".to_owned();
        let keywords = vec![
            "non".to_owned(),
            "important".to_owned(),
            "for".to_owned(),
            "this".to_owned(),
            "test".to_owned(),
        ];
        let mut autocompleted = Autocomplete::new();
        autocompleted.update(&word, &keywords);
        assert_eq!(autocompleted.get_keywords(), &Vec::<String>::new());
        assert_eq!(autocompleted.get_common(), &String::new());

        // returns correctly full sets with full word
        let word = "some_word".to_owned();
        let keywords = vec![
            "some_word".to_owned(),
            "some_word".to_owned(),
            "some_word".to_owned(),
            "some_word".to_owned(),
            "some_word".to_owned(),
        ];
        autocompleted.update(&word, &keywords);
        assert_eq!(
            autocompleted.get_keywords(),
            &vec![
                "some_word".to_owned(),
                "some_word".to_owned(),
                "some_word".to_owned(),
                "some_word".to_owned(),
                "some_word".to_owned()
            ]
        );
        assert_eq!(autocompleted.get_common(), &"some_word".to_owned());

        // returns correctly full sets with one or more char
        let word = "s".to_owned();
        let keywords = vec![
            "some_word".to_owned(),
            "some_word".to_owned(),
            "some_word".to_owned(),
            "some_word".to_owned(),
            "some_word".to_owned(),
        ];
        autocompleted.update(&word, &keywords);
        assert_eq!(
            autocompleted.get_keywords(),
            &vec![
                "some_word".to_owned(),
                "some_word".to_owned(),
                "some_word".to_owned(),
                "some_word".to_owned(),
                "some_word".to_owned()
            ]
        );
        assert_eq!(autocompleted.get_common(), &"some_word".to_owned());

        // returns correctly sets
        let word = "s".to_owned();
        let keywords = vec![
            "some_word".to_owned(),
            "some_other_word".to_owned(),
            "none".to_owned(),
        ];
        autocompleted.update(&word, &keywords);
        assert_eq!(
            autocompleted.get_keywords(),
            &vec!["some_word".to_owned(), "some_other_word".to_owned(),]
        );
        assert_eq!(autocompleted.get_common(), &"some_".to_owned());

        // returns correctly sets
        let word = "some_w".to_owned();
        let keywords = vec![
            "some_word".to_owned(),
            "some_other_word".to_owned(),
            "none".to_owned(),
        ];
        autocompleted.update(&word, &keywords);
        assert_eq!(autocompleted.get_keywords(), &vec!["some_word".to_owned(),]);
        assert_eq!(autocompleted.get_common(), &"some_word".to_owned());
    }

    #[test]
    fn amortized() {
        // normal conditions
        let vec = vec!["_a".to_owned(), "_ab".to_owned(), "_abc".to_owned()];
        let word = "_".to_owned();
        let mut autocomplete = Autocomplete::new();
        autocomplete.update(&word, &vec);
        autocomplete.amortize();
        assert_eq!(autocomplete.get_keywords(), &vec!["_a  ", "_ab ", "_abc"]);

        // similar length
        let vec = vec!["_aa".to_owned(), "_bb".to_owned(), "_cc".to_owned()];
        autocomplete.update(&word, &vec);
        autocomplete.amortize();
        assert_eq!(autocomplete.get_keywords(), &vec!["_aa", "_bb", "_cc"]);

        // empty vec
        let vec = Vec::with_capacity(0);
        autocomplete.update(&word, &vec);
        autocomplete.amortize();
        let return_vec: Vec<String> = Vec::with_capacity(0);
        assert_eq!(autocomplete.get_keywords(), &return_vec);
    }

    #[test]
    fn increment_index_wraps_around() {
        let mut autocomplete = Autocomplete::new();

        let vec = vec!["_a".to_owned(), "_ab".to_owned(), "_abc".to_owned()];
        let word = "_".to_owned();
        autocomplete.update(&word, &vec);
        autocomplete.incr_idx();
        assert_eq!(autocomplete.tab_idx, 1);
        autocomplete.incr_idx();
        assert_eq!(autocomplete.tab_idx, 2);
        autocomplete.incr_idx();
        assert_eq!(autocomplete.tab_idx, 0);
        autocomplete.incr_idx();
        assert_eq!(autocomplete.tab_idx, 1);
    }
}
