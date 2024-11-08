// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use an enum for the box color
enum Color {
    Red,
    Blue,
    Green,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Red => printl("The color is RED."),
            Color::Blue => println!("The color is BLUE."),
            Color::Green => println!("The color is GREEN."),
        }
    }
}
struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

impl Dimensions {
    fn print(&self) {
        println!("width: {:?}", self.width);
        println!("height: {:?}", self.height);
        println!("depth: {:?}", self.depth);
    }
}

// * Use a struct to encapsulate the box characteristics
struct ShippingBox {
    color: Color,
    weight: f64,
    dimensions: Dimensions,
}

// * Implement functionality on the box struct to create a new box
impl ShippingBox {
    fn new(weight: f64, color: Color, dimensions: Dimensions) -> Self {
        Self {
            weight,
            color,
            dimensions,
        }
    }

    // * Implement functionality on the box struct to print the characteristics
    fn print(&self) {
        println!("Weight: {:?}", self.weight);
        self.color.print();
        self.dimensions.print();
    }
}

fn main() {
    let my_dimensions = Dimensions {
        width: 10.2,
        height: 44.1,
        depth: 2.1,
    };
    let the_box = ShippingBox::new(10.6, Color::Red, my_dimensions);
    the_box.print()
}
