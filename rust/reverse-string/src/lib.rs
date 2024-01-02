use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    // split the chcaracters into graphemes, using extended unicode
    let the_chars = input.graphemes(true);
    // reverse the characters and collect them into a string
    let results: String = the_chars.rev().collect();
    results
}
