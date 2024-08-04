// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
enum Color {
    Red,
    Green,
    Blue,
    Purple,
    Black,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Red => println!("Red"),
            Color::Green => println!("Green"),
            Color::Blue => println!("Blue"),
            Color::Purple => println!("Purple"),
            Color::Black => println!("Black"),
        }
    }
}
struct Profile {
    name: String,
    age: u32,
    color: Color,
}

fn print_name(name: &str) {
    println!("name: {:?}", name);
}

// * NOTE: MAIN FUNCTION HERE
fn main() {

    // * Create and store at least 3 people in a vector
    let people = vec![
        Profile {
            name: "Bobby".to_owned(),
            age: 12,
            color: Color::Purple,
        },
        Profile {
            name: "Chrissy".to_owned(),
            age: 5,
            color: Color::Black,
        },
        Profile {
            name: "Diana".to_owned(),
            age: 19,
            color: Color::Blue,
        },
    ];

    // * Iterate through the vector using a for..in loop

    for i in people {
        // * Use an if expression to determine which person's info should be printed
        if i.age <= 10 {
            print_name(&i.name);
            i.color.print();
        }
    }
    // * The name and colors should be printed using a function
}
