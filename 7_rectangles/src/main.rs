fn main() {
    let width = 30;
    let height = 50;
    println!(
        "The area of the rectangle is {} square pixels",
        area(width, height)
    );

    let rectangle = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels",
        area_tuple(rectangle)
    );

    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels",
        area_struct(&rectangle)
    );
    println!("rectangle is {:#?}", rectangle);

    // dbg! takes (and returns) ownership of an expression, as opposed to println!, which takes a reference
    // dbg -> stderr
    // println -> stdout
    dbg!(&rectangle);

    println!(
        "The area of the rectangle is {} square pixels",
        rectangle.area()
    );
    if rectangle.width() {
        println!(
            "The rectangle has a nonzero width; it is {}",
            rectangle.width
        );
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(3);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// associated functions
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
