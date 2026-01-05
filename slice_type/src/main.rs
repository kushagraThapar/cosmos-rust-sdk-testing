fn main() {
    println!("Hello, world!");

    // Using string slicesHere’s a small programming problem:
    // Write a function that takes a string of words separated by spaces and returns the first word it finds in that string.
    // If the function doesn’t find a space in the string, the whole string must be one word, so the entire string should be returned.

    let my_string = String::from("hello world");
    let first = first_word(&my_string);
    println!("The first word ends at index: {}", first);
    let my_string_literal = "hello world";
    let first_word_literal = find_first_word(my_string_literal);
    println!("The first word is: {}", first_word_literal);
    let word1 = find_first_word(&my_string[0..6]);
    let word2 = find_first_word(&my_string[..]);
    println!("The first word is: {}", word1);
    println!("The first word is: {}", word2);

    let word = String::from("hello");
    let slice = &word[0..2];
    println!("The slice is: {}", slice);
}

fn find_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word(string: &String) -> usize {
    let bytes = string.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    string.len()
}
