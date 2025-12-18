fn main() {
    println!("Hello, world!");

    let number = 5;

    if number < 5 {
        println!("The number is less than 5");
    } else if number == 5 {
        println!("The number is equal to 5");
    } else {
        println!("The number is greater than 5");
    }

    let condition = true;
    let number = if condition { 8 } else { 6 };
    println!("The value of number is: {number}");

    let mut counter = 0;

    loop {
        counter += 1;
        println!("Counter: {counter}");
        if counter == 5 {
            break;
        }
    }
    println!("The value of counter is: {counter}");

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // This will be the value of `result`
        }
    };
    println!("The value of result is: {result}");

    'counting_loop: loop {
        println!("Counting...");
        let mut count = 0;

        loop {
            if count == 5 {
                break 'counting_loop; // Breaks out of the outer loop
            }
            println!("Count: {count}");
            count += 1;
        }
    }

    let mut array = [10, 20, 30, 40, 50];
    counter = 0;
    while counter < array.len() {
        println!("Array element at index {counter} is: {}", array[counter]);
        counter += 1;
    }

    for element in array.iter_mut() {
        println!("Element: {element}");
        *element = 10;
        println!("Modified element: {element}");
    }
    
    for element in array {
        println!("Element: {element}");
    }
}
