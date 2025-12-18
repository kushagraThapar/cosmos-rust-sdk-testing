fn main() {
    println!("Hello, world!");

    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {MAX_POINTS}");

    let x = x + 1;

    println!("The value of x is: {x}");

    {
        let x = 10;
        println!("The value of x in the inner scope is: {x}");

        let x = x.to_string() + "-something";
        println!("The value of x is: {}", x);
        println!("The type of x is: {}", &x);
    }

    println!("The value of x is: {x}");

    let mut tuple = (1, 2.0, "3");
    println!("The value of tuple is: {:?}", tuple);
    let (x, y, z) = tuple;
    println!("The value of x is: {x}, y is: {y}, z is: {z}");
    // tuple needs to be mutable to change its values.
    tuple.0 = 10;
    tuple.1 = 20.0;
    tuple.2 = "30";
    println!("The value of tuple is: {:?}", tuple);
    println!("The value of tuple 3rd entry is: {:?}", tuple.2);
    // The below won't compile, cannot change types.
    // tuple.2 = 30;
    println!("The value of x is: {x}, y is: {y}, z is: {z}");

    let array = [1, 2, 3, 4, 5];
    println!("The value of array is: {:?}", array);
    println!("The value of array 3rd entry is: {:?}", array[2]);
    // The below won't compile, cannot access index out of bound during compile time.
    //  println!("The value of array 5th entry is: {:?}", array[5]);
    println!("Please enter an index to access the array: ");
    let mut index = String::new();
    std::io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = match index.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input, using default index 0.");
            0
        },
    };
    println!("The element at the {index} is: {}", array[index])
}
