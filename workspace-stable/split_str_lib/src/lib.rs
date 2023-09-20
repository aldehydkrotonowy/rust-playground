//----------------------------------------------------------- v1
#[derive(Debug)]
pub struct SplitStrV1<'a, 'b> {
    reminder: &'a str,
    delimiter: &'b str,
}
impl<'a, 'b> SplitStrV1<'a, 'b> {
    pub fn new(haystack: &'a str, delimiter: &'b str) -> Self {
        Self {
            reminder: haystack,
            delimiter,
        }
    }
}
impl<'a, 'b> Iterator for SplitStrV1<'a, 'b> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_delimiter) = self.reminder.find(self.delimiter) {
            let until_delimiter = &self.reminder[..next_delimiter];
            self.reminder = &self.reminder[(next_delimiter + self.delimiter.len())..];
            Some(until_delimiter)
        } else if self.reminder.is_empty() {
            None
        } else {
            let rest = self.reminder;
            self.reminder = "";
            Some(rest)
        }
    }
}
#[test]
fn split_str_v1_normal() {
    let haystack = "a b c d e";
    let letters_vec: Vec<&str> = SplitStrV1::new(haystack, " ").collect();
    assert_eq!(letters_vec, vec!["a", "b", "c", "d", "e"]);
}
#[test]
fn split_str_v1_end() {
    let haystack = "a b c d e ";
    let letters_vec: Vec<&str> = SplitStrV1::new(haystack, " ").collect();
    assert_eq!(letters_vec, vec!["a", "b", "c", "d", "e"]);
}
#[test]
fn split_str_v1_start() {
    let haystack = " a b c d e";
    let letters_vec: Vec<&str> = SplitStrV1::new(haystack, " ").collect();
    assert_eq!(letters_vec, vec!["a", "b", "c", "d", "e"]);
}
#[test]
fn split_str_v1_start_and_end() {
    let haystack = " a b c d e ";
    let letters_vec: Vec<&str> = SplitStrV1::new(haystack, " ").collect();
    assert_eq!(letters_vec, vec!["a", "b", "c", "d", "e"]);
}
#[test]
fn split_str_v1_multiple() {
    let haystack = "a b    c d e";
    let letters_vec: Vec<&str> = SplitStrV1::new(haystack, " ").collect();
    assert_eq!(letters_vec, vec!["a", "b", "c", "d", "e"]);
}
