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
struct Person {
    name: String,
    age: i32,
    color: String,
}


// * The name and colors should be printed using a function
fn print_details(name: &str, color: &str) {
    println!("Name: {:?}", name);
    println!("Favorite Color: {:?}", color);
}

fn main() {
    // * Create and store at least 3 people in a vector
    let peoples = vec![
        Person{
            age: 7,
            name: String::from("Liyana"),
            color: "Green".to_owned(),
        },
        Person{
            age: 8,
            name: String::from("Liviya"),
            color: "Red".to_owned(),
        },
        Person{
            age: 14,
            name: String::from("Ali"),
            color: "Black".to_owned(),
        }
    ];

    // * Iterate through the vector using a for..in loop
    for person in peoples {
        // * Use an if expression to determine which person's info should be printed
        if person.age <= 10 {
            print_details(&person.name, &person.color);
        }
    }
}
