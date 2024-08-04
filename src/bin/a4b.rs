// Topic: Decision making with match
//
// Program requirements:
// * Display "one", "two", "three", or "other" based on whether
//   the value of a variable is 1, 2, 3, or some other number,
//   respectively
//
// Notes:
// * Use a variable set to any integer
// * Use a match expression to determine which message to display
// * Use an underscore (_) to match on any value

fn main() {
    let some_number = 7;

    match some_number {
        1 => println!("{:?}", some_number),
        2 => println!("{:?}", some_number),
        3 => println!("{:?}", some_number),
        _ => println!("It's some other number {:?}", some_number),
    }
}
