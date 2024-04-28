// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

// * Use an enum for the box color
enum Color {
    Black,
    Brown,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Black => println!("black"),
            Color::Brown => println!("brown"),
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
        println!("Width: {:?}", self.width);
        println!("Height: {:?}", self.height);
        println!("Depth: {:?}", self.depth);
    }
}

// * Use a struct to encapsulate the box characteristics
// * Must include dimensions, weight, and color

struct ShippingBox {
    color: Color,
    weight: f64,
    dimensions: Dimensions,
}

// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics
impl ShippingBox {
    fn new(weight: f64, color: Color, dimensions: Dimensions) -> Self {
        Self {
            weight: weight,
            color,
            dimensions
        }
    }

    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("Weight: {:?}", self.weight);
    }
}

fn main() {
    let dimensions_for_small_box = Dimensions {
        width: 1.0,
        height: 2.0,
        depth: 3.0
    };

    let small_box = ShippingBox::new(3.6, Color::Black, dimensions_for_small_box);

    small_box.print();
}
