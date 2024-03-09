use std::io;
use indexed_string::indexed_string::IndexedString;

fn main() {
    println!("****MONOGRAM PROGRAM****");

    println!("Enter your first name:");

    //Handling user input
    let mut firstName = String::new();
    io::stdin().read_line(&mut firstName).expect("Failed to read the line");

    println!("Enter your last name:");
    let mut lastName = String::new();
    io::stdin().read_line(&mut lastName).expect("Failed to read the line");

    //Output

    let fullName = firstName.to_string() + "" + &lastName;

        //Displaying the full name
    println!("Your full name: {}", fullName);

        //Creating the monogram
    let indexed_firstName = IndexedString::from(firstName);
    let indexed_lastName = IndexedString::from(lastName);

    println!("Your monogram: {}{}", indexed_firstName[0], indexed_lastName[0]); 
}
