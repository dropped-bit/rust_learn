// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks

enum Flavours {
    Strawberry,
    Cola,
    Vanilla,
}

// * Use a struct to store drink flavor and fluid ounce information
struct Drink {
    flavour: Flavours,
    fluid_ounce: f64
}

// * Use a function to print out the drink flavor and ounces
fn print_drink(drink: Drink) {

// * Use a match expression to print the drink flavor
    match drink.flavour {
        Flavours::Strawberry => println!("Strawberry"),
        Flavours::Cola => println!("Cola"),
        Flavours::Vanilla => println!("Vanilla"),
    };

    println!("Fluid ounces: {:?}", drink.fluid_ounce);
}

fn main() {
    let my_drink = Drink {
        flavour: Flavours::Strawberry,
        fluid_ounce: 5.99
    };
    print_drink(my_drink);

    let my_second_drink = Drink {
        flavour: Flavours::Cola,
        fluid_ounce: 41.99
    };
    print_drink(my_second_drink);
}
