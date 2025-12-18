fn main() {
    println!("Hello, world!");
    let s = "hello";

    {
        let s = s.to_string();
        println!("The value of s in the inner scope is: {}", s);
    }

    let s = String::from("hello");
    println!("The value of s is: {}", s);

    let s1 = String::from("hello");
    let s2 = s1;
    // The below line won't compile because s1 is moved to s2.
    // println!("The value of s1 is: {}", s1);

    println!("The value of s2 is: {}", s2);

    take_ownership(s2);

    // The below line won't compile because s2 is moved to take_ownership function.
    // println!("The value of s2 is: {}", s2);

    let x = 5;
    make_copy(x);
    println!("The value of x is: {}", x);

    let s1 = gives_ownership();
    println!("The value of s1 is: {}", s1);

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    // The below line won't compile because s2 is moved to takes_and_gives_back function.
    // println!("The value of s2 is: {}", s2);

    println!("The value of s3 is: {}", s3);

}

fn take_ownership(some_string: String) {
    println!("The value of some_string is: {}", some_string);
}

fn make_copy(some_integer: i32) {
    println!("The value of some_integer is: {}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
