// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
struct Student {
    name: String,
    locker: Option<i32>,
}
// * The locker assignment should use an Option<i32>

fn main() {
    let liyana = Student {
        name: "Liyana".to_owned(),
        locker: Some(3)
    };

    print!("Student: {:?}", liyana.name);

    match liyana.locker {
        Some(num) => println!("locker number: {:?}", num),
        None => println!("No locker assigned")
    }

}
