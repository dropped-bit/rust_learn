// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

// * uses a function add two numbers together
fn add_two_numbers(a: u32, b: u32) -> u32 {
    a + b
}
// * Use a function to display the result
fn print_result(result: u32) {
    println!("The result of the addition is: {:?}", result);
}

fn main() {
    let ergebnis = add_two_numbers(10, 20);

    print_result(ergebnis);
}
