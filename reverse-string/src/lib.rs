use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    // input.chars().rev().collect()
    // work with graphemes:
    input.graphemes(true).rev().collect()
}
