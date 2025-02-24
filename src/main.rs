mod character_traits; // Imports character_traits.rs
use character_traits::POS_TRAITS; // Imports the POS_TRAITS array from character_traits.rs
use character_traits::NEG_TRAITS; // Imports the NEG_TRAITS arrat from character_traits.rs
use rand::Rng; // Use the rand::Rng crate

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

    // Select a random number between 0 and the last index of traits_string
    let selection = rng.random_range(0..=12);
    return ( pos_string[selection].clone(), neg_string[selection].clone() );
}

fn main() {
    println!("Character description: {:?}", get_traits()); // Print the selected index
}
