// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
struct ShippingBox {
    dimensions: (i32, i32, i32),
    weight: f32,
    color: BoxColor,
}

// * Use an enum for the box color
enum BoxColor {
    Red,
    Blue,
    Green,
}

// * Implement functionality on the box struct to create a new box
impl ShippingBox {
    fn create_box() -> Self {
        Self {
            dimensions: (50, 20, 30),
            weight: 20.5,
            color: BoxColor::Red,
        }
    }

    fn print_box(&self) {
        // using &self why???
        println!("The shipping box has following characeristics:");
        println!("Dimensions: {:?}", self.dimensions);
        println!("Weight: {:?}kg", self.weight);
        println!("Color: {:?}", self.color);
    }
}
// * Implement functionality on the box struct to print the characteristics

fn main() {
    let the_box = ShippingBox::create_box();
    the_box.print_box();
}
