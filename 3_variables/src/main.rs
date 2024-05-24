use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

fn mutability() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

fn constants() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
}

fn shadowing() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "   ";
    let spaces = spaces.len();
}

fn data_types() {
    let guess: u32 = "42".parse().expect("Not a number!");

    let signed_int: i32 = -42;
    let unsigned_int: u32 = 42;

    let readable_number: u32 = 42_000;

    let float = 2.0;
    let float: f32 = 3.0;
}

fn numeric_operations() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
}

fn boolean_type() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}

fn chartacter_type() {
    // Note that we specify char literals with single quotes, as opposed to string literals, which use double quotes.
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}

fn compound_types() {
    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // Array
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // You can also initialize an array to contain the same value for each element by specifying the initial value, followed by a semicolon, and then the length of the array in square brackets
    let a = [3; 5];

    let first = a[0];
    let second = a[1];

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn expressions() {
    let y = {
        let x = 3;
        x + 1 // without semicolon at the end because it is an expression
    };

    println!("The value of y is: {y}");
}

fn five() -> i32 {
    5
}

fn plus_one_expression(x: i32) -> i32 {
    x + 1 // expression
}

fn plus_one_statement(x: i32) -> i32 {
    return x + 1; // statement
}
