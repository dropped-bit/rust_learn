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
fn check(a: bool) {
    match a {
        true => println!("It's small"),
        false => println!("It's big"),
    };
}

fn main() {
    // * Use a boolean variable set to the result of
    //   an if..else expression to store whether the value
    //   is > 100 or <= 100
    //
    let value_to_check = 92;
    let if_lt_100 = value_to_check < 100;
    // * Use a function to print the messages
    check(if_lt_100)
}
