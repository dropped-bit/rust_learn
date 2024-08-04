// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple

fn tuple_example() -> (i32, i32) {
    (1, 7)
}

fn main() {
    // * Destructure the return value into two variables
    let (x, y) = tuple_example();
    // * Use an if..else if..else block to determine what to print
    if y > 5 {
        println!("Y is greater than 5, current coordinates: {:?}, {:?}", x, y);
    } else if y == 5 {
        println!("Y is equat to, current coordinates: {:?}, {:?}", x, y);
    } else {
        println!("Y is less than 5, current coordinates: {:?}, {:?}", x, y);
    }
}
