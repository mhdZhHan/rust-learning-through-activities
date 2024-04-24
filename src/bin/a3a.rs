// Topic flow control using if..else
//
// Program requirements
// * Display a message based on the value of a boolean variable
// * When the variable is set to true, display "hello"
// * When the variable is set to false, display "goodbye"
//
//
// Notes:
// * Use a variable set either true or false
// * Use an if..else block to determine which message to display
// * Use teh println macro to display message to the terminal

fn main() {
    // * Display a message based on the value of a boolean variable
    let my_bool = true;
    // * When the variable is set to true, display "hello"
    if my_bool {
        println!("Hello");
    }else {
        println!("goodbye");
    }
}
