use std::fmt;

#[derive(Debug)]
struct Rectangle {
    length: u64,
    width: u64,
}

impl Rectangle {
    fn area(&self) -> u64 {
        self.length * self.width
    }

    fn width(&self) -> u64 {
        self.width
    }

    fn length(&self) -> u64 {
        self.length
    }

    fn has_width(&self) -> bool {
        self.width > 0
    }

    fn has_length(&self) -> bool {
        self.length > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }

    fn square(size: u64) -> Rectangle {
        Self {
            length: size,
            width: size,
        }
    }
}

// Implement Display trait for Rectangle to print area when passed to println!
impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The area of the rectangle is: {}", self.length * self.width)
    }
}

fn main() {
    println!("Hello, world!");

    let length1 = 10;
    let width1 = 5;
    let rect1_area = area(length1, width1);
    println!("The area of rectangle 1 is: {}", rect1_area);

    let rect2 = Rectangle {
        length: 15,
        width: 10,
    };

    //  Using the area_rectangle function
    let rect2_area_function = area_rectangle(&rect2);
    println!("The area of rectangle 2 (using function) is: {}", rect2_area_function);

    // Using the instance method
    let rect2_area_method = rect2.area();
    println!("The area of rectangle 2 (using method) is: {}", rect2_area_method);

    //  Using the println method
    println!("The area of rectangle 2 (using calculate area println) is: {}", rect2);

    // Using the debug method
    println!("The values of rectangle 2 (using debug println) is: {:?}", rect2);

    //  Using the debug macro to print rectangle 2
    dbg!(&rect2);

    let scale = 2;
    let rect3 = Rectangle {
        length: dbg!(rect2.length * scale),
        width: dbg!(rect2.width * scale),
    };
    dbg!(&rect3);
    println!("The area of rectangle 3 is: {}", rect3);

    println!("Does rectangle 3 have non-zero width? {}", rect3.has_width());
    println!("Does rectangle 3 have non-zero length? {}", rect3.has_length());

    println!("Rectangle 3 length: {}", &rect3.length());
    println!("Rectangle 3 width: {}", &rect3.width());

    let rect4 = Rectangle {
        length: 20,
        width: 15,
    };

    println!(
        "Can rectangle 4 hold rectangle 3? {}",
        rect4.can_hold(&rect3)
    );

    println!("Can rectangle 3 hold rectangle 4? {}",
        rect3.can_hold(&rect4)
    );

    let square = Rectangle::square(4);
    println!("Square is: {:?}", square);
    println!("The area of the square is: {}", square);
}

fn area(length: u64, width: u64) -> u64 {
    length * width
}

fn area_rectangle(rect: &Rectangle) -> u64 {
    rect.length * rect.width
}