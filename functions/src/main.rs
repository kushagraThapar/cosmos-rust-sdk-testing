fn main() {
    println!("Hello, world!");
    println!("Calling another function...");
    another_function();
    let x = 5;
    let y = 'a';
    println!("the values are {x} and {y} before calling another function_with_params");
    another_function_with_params(x, y);
    println!("the values are {x} and {y} after calling another function_with_params");
    let y = {
        let x = 3;
        x + 1 // This expression will be the value of `y`
    };
    println!("The value of y is: {y}");
    let return_value = another_function_with_return(10, 'b');
    println!("The value of return_value is: {return_value}");
}

fn another_function() {
    println!("This is another function!");
}

fn another_function_with_params(mut x: i32, mut y: char) {
    println!("The values are {x} and {y} inside another_function_with_params");
    x = x+1;
    y = 'z';
    println!("The values are {x} and {y} after modification inside another_function_with_params");
}

fn another_function_with_return(x: i32, y: char) -> i32 {
    println!("The values are {x} and {y}");
    x + 1 // This will be the return value
}
