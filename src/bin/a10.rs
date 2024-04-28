// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print


// * Use a match expression to determine which message
//   to print

fn print_message(gt_100: bool) {
    match  gt_100 {
        true => println!("It's big"),
        false => println!("It's small")
    }
}

fn main() {
    // * Use a boolean variable set to the result of
    //   an if..else expression to store whether the value
    //   is > 100 or <= 100
    let value = 100;
    let is_gt_100 = value > 100; // shortcut instead of doing if..else

    print_message(is_gt_100);
}
