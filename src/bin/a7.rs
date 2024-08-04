// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Colours {
    Red,
    Green,
    Blue
}

fn output(colour: Colours) {
    match colour {
        Colours::Red => println!("The colour is red"),
        Colours::Green => println!("The colour is green"),
        Colours::Blue => println!("The colour is blue"),
    };
}

fn main() {
    output(Colours::Green);
}
