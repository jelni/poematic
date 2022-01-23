use deunicode::deunicode;
use rand::prelude::*;

pub trait EqUnicodeInsensitive<T: ?Sized> {
    fn eq_unicode_insensitive(&self, rhs: &T) -> bool;
}

impl<T> EqUnicodeInsensitive<T> for str
where
    T: ?Sized + AsRef<str>,
{
    /// Compares two strings ignoring letter case and permitting ASCII equivalents
    /// for unicode characters.
    ///
    /// ```
    /// use poematic::EqUnicodeInsensitive;
    /// assert!("Zęby".eq_unicode_insensitive("żeby"));
    /// ```
    fn eq_unicode_insensitive(&self, rhs: &T) -> bool {
        deunicode(self.to_lowercase().as_ref()) == deunicode(rhs.as_ref().to_lowercase().as_ref())
    }
}

pub trait SplitHuman<'a> {
    type Item: 'a;
    fn split_human(&'a self) -> Box<dyn Iterator<Item = Self::Item> + 'a>;
}

impl<'a> SplitHuman<'a> for str {
    type Item = &'a str;

    /// Splits a string on whitespace and trims non-alphabetic chars off of words.
    ///
    /// ```
    /// use poematic::SplitHuman;
    /// let words = "Lorem ipsum? It's dolor, sit amet!"
    ///     .split_human()
    ///     .collect::<Vec<_>>();
    /// assert_eq!(&words, &["Lorem", "ipsum", "It's", "dolor", "sit", "amet"]);
    /// ```
    fn split_human(&'a self) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
        Box::new(
            self.split_whitespace()
                .map(|s| s.trim_matches(|ch: char| !ch.is_alphabetic())),
        )
    }
}

/// Replaces `n` random words in the given string with a blank `___`.
/// Returns the resulting string and the selected words.
pub fn hide_words<'a>(sentence: &'a str, n: usize) -> (String, Vec<&'a str>) {
    let mut rng = rand::thread_rng();
    let mut result = sentence.to_string();
    let mut hidden_words = vec![];

    let mut words_to_hide = sentence
        .split_human()
        .enumerate()
        .choose_multiple(&mut rng, n);

    words_to_hide.sort_by_key(|(i, _)| *i);

    for (_, word) in words_to_hide.into_iter().rev() {
        // Safe because `word` always points to a subslice of `sentence`
        let byte_offset = unsafe { word.as_ptr().offset_from(sentence.as_ptr()) as usize };
        let blank = "_".repeat(word.chars().count());
        result.replace_range(byte_offset..(byte_offset + word.len()), blank.as_str());

        hidden_words.push(word);
    }

    hidden_words.reverse();
    (result, hidden_words)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hide_words() {
        let (_, words) = hide_words("Lorem ipsum? Dolor sit amet!", 3);
        assert_eq!(words.len(), 3);
    }
}
