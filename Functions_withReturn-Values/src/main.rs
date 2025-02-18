
    fn five() -> i32 {
        5
    }
    fn main() {
        let x = five();

        println!("The value of x is: {x}");
    

   
// Functions with Return Values must declare their TYPE fter an arrow (->)..No macros, no function call, even let statements in the five function - just the number 5 by itself.
// The Function 's return type is specified as -> i32.
// The 5 in the five is the function's return value, which is why the return type is i32. On examination - there are two important bits -
// First : the line let x = five(); shows that there we're using the return value of a function to initialize a variable.   
// Because the function five returns a 5, that line is the same as this = let x = 5;
// Second, the five function has no parameters and defines the type of the return value, but the bodyof the function is a loney 5 with no semi-colon becasue it's an EXPRESSION 
// whose value we want to return


// Another example;

// fn main() {
        let x = plus_one(5);
        println!("The value of x is: {x}");

// }
    fn plus_one(x: i32) -> i32 {
        x + 1
    }


}