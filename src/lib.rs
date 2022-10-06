/// `wordsplit` is a library that takes two arguments `haystack` & `delimiter` and splits the `haystack` with `delimiter`.
///
/// ```rust
/// use wordsplit::WordSplit;
///
/// let haystack = "wordsplit is awesome";
/// let delimiter = " ";
/// let letters = WordSplit::new(haystack, delimiter);
///
/// assert!(letters.eq(vec!["wordsplit", "is", "awesome"].into_iter()));
/// ```

#[derive(Debug)]
pub struct WordSplit<'haystack, D> {
    remainder: Option<&'haystack str>,
    delimiter: D,
}

pub trait Delimiter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

impl<'haystack, D> WordSplit<'haystack, D> {
    pub fn new(haystack: &'haystack str, delimiter: D) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<'haystack, D> Iterator for WordSplit<'haystack, D>
where
    D: Delimiter,
{
    type Item = &'haystack str;
    fn next(&mut self) -> Option<Self::Item> {
        // impl<T> Option<T> { fn as_mut(&mut self) -> Option< &mut T>}
        let remainder = self.remainder.as_mut()?;
        if let Some((delimiter_start, delimiter_end)) = self.delimiter.find_next(remainder) {
            let until_delimiter = &remainder[..delimiter_start];
            *remainder = &remainder[delimiter_end..];
            Some(until_delimiter)
        } else {
            // impl<T> Option<T> { fn take(&mut self) -> Option<T> }
            self.remainder.take()
        }
    }
}

impl Delimiter for &str {
    // `self` is a reference to `&str`
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))
    }
}

impl Delimiter for char {
    // `self` is a reference to `char`
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices()
            .find(|(_, c)| c == self)
            .map(|(start, _)| (start, self.len_utf8()))
    }
}

pub fn until_char(s: &str, c: char) -> &'_ str {
    WordSplit::new(s, c)
        .next()
        .expect("WordSplit always gives alteast one result")
}
