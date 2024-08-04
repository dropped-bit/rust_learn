// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
struct GroceryItem {
    id: i32,
    quantity: i32,
}
// * Create a function to display the quantity, with the struct as a parameter
fn print_quantity(grocery: &GroceryItem) {
    println!("The quantity of this item is: {:?}", grocery.quantity)
}

// * Create a function to display the id number, with the struct as a parameter
fn print_id(grocery: &GroceryItem) {
    println!("The id of this item is: {:?}", grocery.id)
}

fn main() {
    let my_grocery_item = GroceryItem {
        id: 512,
        quantity: 3,
    };

    print_id(&my_grocery_item);
    print_quantity(&my_grocery_item);
}
