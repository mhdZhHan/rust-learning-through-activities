// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

// * Use an enum to create different flavors of drinks
enum Flavor {
    Sweet,
    Fruity,
    Spice,
}

// * Use a struct to store drink flavor and fluid ounce information
struct Drink {
    flavor: Flavor,
    fluid_ounce: f64,
}

// * Use a function to print out the drink flavor and ounces
fn display_drink(drink: Drink) {
    // * Use a match expression to print the drink flavor
    match drink.flavor {
        Flavor::Sweet => println!("Flavor: Sweet"),
        Flavor::Fruity => println!("Flavor: Fruite"),
        Flavor::Spice => println!("Flavor: Spice"),
    }

    println!("Ounce: {:?}", drink.fluid_ounce);
}

fn main() {
    let spice_drink = Drink {
        flavor: Flavor::Spice,
        fluid_ounce: 5.7
    };

    let fruity_drink = Drink {
        flavor: Flavor::Fruity,
        fluid_ounce: 10.0,
    };

    display_drink(spice_drink);

    display_drink(fruity_drink);
}
