fn main() {
    println!("Hello, world!");

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s2 = String::from("hello");
    //  Mutable references
    let new_len = change_and_calculate_length(&mut s2);
    println!("The new length of '{}' is {}.", s2, new_len);

    let r1 = &s2;
    let r2 = &s2;

    println!("The values of r1 and r2 are: '{}' and '{}'.", r1, r2);
}

fn calculate_length(s: &String) -> usize {
    //  s cannot be modified here because it is a reference
    // s.push_str(", world"); // This line would cause a compile-time errors
    s.len()
}

fn change_and_calculate_length (some_string: &mut String) -> usize {
    some_string.push_str(", world");
    some_string.len()
}
