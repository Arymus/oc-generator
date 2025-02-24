mod character_traits; // Imports character_traits.rs
use character_traits::POS_TRAITS; // Imports the POS_TRAITS array from character_traits.rs
use character_traits::NEG_TRAITS; // Imports the NEG_TRAITS array from character_traits.rs
use character_traits::HAIR_COLOR; // Imports the HAIR_COLOR array from character_traits.rs
use rand::Rng; // Use the rand::Rng crate
use std::io; // Use the standard input/output crate

fn get_traits() -> (String, String) {
    let mut rng = rand::rng(); 
    let mut pos_string: Vec<String> = Vec::new(); // Defines an empty vec for containing strings
    let mut neg_string: Vec<String> = Vec::new();

    // For every item in the POS_TRAITS array, convert to a string and add it to the traits_string vector
    for pos in &POS_TRAITS {
        pos_string.push(pos.to_string());
    }

    // For every item in the NEG_TRAITS array, convert to a string and add it to the traits_string vector
    for neg in &NEG_TRAITS {
        neg_string.push(neg.to_string());
    }

    // Select a random number between 0 and 12
    let pos_selection = rng.random_range(0..=12);
    let neg_selection = rng.random_range(0..=12);

    // Return the result of positive and negative traits as a tuple
    return ( pos_string[pos_selection].clone(), neg_string[neg_selection].clone() );
}

fn main() {
    // Print input prompt
    println!("OC Generator");
    println!("What's your OC's name?");

    let mut input: String = String::new(); // Create a new string
    io::stdin().read_line(&mut input).expect("Failure reading string"); // Read the string and panic with a message if there's an error

    println!("{}'s description:\n{:?}, {:?}, {:?}", input, get_traits(), get_traits(), get_traits()); // Print the selected index 3 times
}
