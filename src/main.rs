use std::io;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!(" The value of x is: {x}");

    //Shadowing

    let x = 5;
    let x = x + 1;

    {
        let x = x * 3;
        println!("The value of x in the inner scope is: {x} ");

    }
    println!("The value of x is {x}");

    // Floating-point numbers

    let _x = 2.0;
    let _y: f32 = 3.0;

    // Numeric operations

    let _sum = 5 + 19;
    let _difference = 55.5 - 4.2;
    let _product = 4 * 5;
    let _difference = 70 - 15;
    let _quotient = 65.7 / 30.2;
    let _truncated = -5 /3;
    let _remainder = 49 % 5;

    // Boolean

    let _t = true;
    let _f: bool = false;

    // Character type
    let _c = 'z';
    let _z: char = 'Z'; 
    let _heart_eyed_cat = 'B';

    // Compound_Type => tuple

    let tup = (500, 6.4, 1);
    let (_x, _y, _z) = tup;
    
    println!("The value of y is {_y}");

    let tup = (500, 6.1, 1);
    let (_x, _y, _z) = tup;

    println!("The value of y is: {_y}");

    // Accessing a tuple with (.)

    let x: (i32, f64, u8) = (500, 8.1, 1);
    let _five_hundred = x.0;
    let _six_point_four = x.1;
    let _one = x.2;

    println!("The value of _one is: {_one}");
    println!("The value of _five_hundred is: {_five_hundred}");
    println!("The value of _six_point_four is: {_six_point_four}");


    // Compound_Type = Arrays
    let a = [1, 2, 3, 4, 5];
   
    println!("Please enter an array index");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let _element = a[index];

    println!("The value of the element at index {index} is: {_element}");

   // Functions_in_Rust... Rust Parameters

   {
        println!("Hello Rust Function");
        another_function(6);
    }

   fn another_function(x: i32) {
    println!("The value of x is: {x}");
   }

// Defining multiple variables

   {
    print_labeled_measurement (5, 'h');

   }

   fn print_labeled_measurement(value: i32, unit_label: char) {
        println!("The measurement is: {value}{unit_label}");
   }
   

}
