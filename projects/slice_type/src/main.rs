// Slices let you reference a contiguous sequence of elements in a collection
// rather than the whole collection. A slice is a kind of reference, so it does
// not have ownership.
fn main() {
    let s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    // mutable code
    // s.clear(); // this empties the String, making it equal to ""

    println!("Value of word: {}", word);
}

fn first_word_index(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// &str is the "string slice" type
fn first_word(s: &String) -> &str {
    let index = first_word_index(s);

    &s[..index]
}
