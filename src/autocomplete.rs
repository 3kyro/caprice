// lets define three dtata types

/*

bash autocomplete,
find the biggest word in teh list and amke every word that long by appending spaces
the terminal should auto space them correctly
by pressing tab you just change the bg color of one of the items in the list

*/

use crossterm::{Attribute, Color, Colored};

pub(crate) struct Autocomplete {
    keywords: Vec<String>,
    common: String,
}

impl Autocomplete {

    pub fn new() -> Self {
        Autocomplete {
            keywords: Vec::new(),
            common: String::new(),
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
/*
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
        assert_eq!(autocomplete(&word, &keywords), (Vec::new(), None));

        let word = "random_word".to_owned();
        let keywords: Vec<String> = Vec::new();
        assert_eq!(autocomplete(&word, &keywords), ((Vec::new(), None)));

        let word = "".to_owned();
        let keywords: Vec<String> = Vec::new();
        assert_eq!(autocomplete(&word, &keywords), ((Vec::new(), None)));
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
        assert_eq!(autocomplete(&word, &keywords), (Vec::new(), None));

        // returns correclty full sets with full word
        let word = "some_word".to_owned();
        let keywords = vec![
            "some_word".to_owned(),
            "some_word".to_owned(),
            "some_word".to_owned(),
            "some_word".to_owned(),
            "some_word".to_owned(),
        ];
        assert_eq!(
            autocomplete(&word, &keywords),
            (
                vec![
                    "some_word".to_owned(),
                    "some_word".to_owned(),
                    "some_word".to_owned(),
                    "some_word".to_owned(),
                    "some_word".to_owned()
                ],
                Some("some_word".to_owned())
            )
        );

        // returns correclty full sets with one or more char
        let word = "s".to_owned();
        let keywords = vec![
            "some_word".to_owned(),
            "some_word".to_owned(),
            "some_word".to_owned(),
            "some_word".to_owned(),
            "some_word".to_owned(),
        ];
        assert_eq!(
            autocomplete(&word, &keywords),
            (
                vec![
                    "some_word".to_owned(),
                    "some_word".to_owned(),
                    "some_word".to_owned(),
                    "some_word".to_owned(),
                    "some_word".to_owned()
                ],
                Some("some_word".to_owned())
            )
        );

        // returns correclty sets
        let word = "s".to_owned();
        let keywords = vec![
            "some_word".to_owned(),
            "some_other_word".to_owned(),
            "none".to_owned(),
        ];
        assert_eq!(
            autocomplete(&word, &keywords),
            (
                vec!["some_word".to_owned(), "some_other_word".to_owned()],
                Some("some_".to_owned())
            )
        );

        // returns correclty sets
        let word = "some_w".to_owned();
        let keywords = vec![
            "some_word".to_owned(),
            "some_other_word".to_owned(),
            "none".to_owned(),
        ];
        assert_eq!(
            autocomplete(&word, &keywords),
            (vec!["some_word".to_owned()], Some("some_word".to_owned()))
        );
    }

    #[test]
    fn amortised() {
        // normal consditions
        let mut vec = vec!["a".to_owned(), "ab".to_owned(), "abc".to_owned()];
        assert_eq!(
            Autocomplete::get_amortisized_array(&mut vec),
            &vec!["a  ", "ab ", "abc"]
        );

        // similar length
        let mut vec = vec!["aa".to_owned(), "bb".to_owned(), "cc".to_owned()];
        assert_eq!(
            Autocomplete::get_amortisized_array(&mut vec),
            &vec!["aa", "bb", "cc"]
        );

        // empty vec
        let mut vec = Vec::with_capacity(0);
        let return_vec: Vec<String> = Vec::with_capacity(0);
        assert_eq!(Autocomplete::get_amortisized_array(&mut vec), &return_vec);
    }
}
*/