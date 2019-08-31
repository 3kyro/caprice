// lets define three dtata types


pub(crate) struct TabWord {
    content: String,
    selected: bool,
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
pub(crate) fn autocomplete<'a>(word: &str, keywords: &'a [&str]) -> (Vec<&'a str>, Option<&'a str>) {
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

mod tests {

    #[cfg(test)]
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