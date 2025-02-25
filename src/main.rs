mod character_traits; // Imports character_traits.rs
use character_traits::POS_TRAITS; // Imports the POS_TRAITS array from character_traits.rs
use character_traits::NEG_TRAITS; // Imports the NEG_TRAITS array from character_traits.rs
use character_traits::COLORS; // Imports the HAIR_COLOR array from character_traits.rs
use rand::Rng; // Use the rand::Rng crate
use std::io; // Use the standard input/output crate

fn gen_traits() -> (String, String) {
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
    let pos_selection : usize= rng.random_range(0..=12);
    let neg_selection: usize = rng.random_range(0..=12);

    // Return the result of positive and negative traits as a tuple
    return ( pos_string[pos_selection].clone(), neg_string[neg_selection].clone() );
}

fn gen_color() -> (String, String) {
    let mut rng = rand::rng(); 
    let mut hair_colors: Vec<String> = Vec::new(); // Defines an empty vec for containing strings
    let mut eye_colors: Vec<String> = Vec::new();

    // For each color in the HAIR_COLOR array, parse each index to a string an add it to the hair_colors vec
    for color in &COLORS {
        hair_colors.push(color.to_string());
	eye_colors.push(color.to_string());
    }

    let hair_color_selection: usize = rng.random_range(0..=hair_colors.len() - 1); // Pick a random number between 0 and the last index of the hair_colors array
    let eye_color_selection: usize = rng.random_range(0..=eye_colors.len() - 1); // Generates a random number between 0 and the final index of the eye_colors array

    return ( hair_colors[hair_color_selection].clone(), eye_colors[eye_color_selection].clone() ); // Return the randomly selected hair and eye color
}

fn main() {
    // Print input prompt
    println!("OC Generator");
    println!("What's your OC's name?");

    let mut input: String = String::new(); // Create a new string
    io::stdin().read_line(&mut input).expect("Failure reading string"); // Read the string and panic with a message if there's an error

    println!("Name: {}Description:\n{:?}, {:?}, {:?}", input, gen_traits(), gen_traits(), gen_traits()); // Print the selected index 3 times
    println!("Hair color: {:?}", gen_color()); // Print the selected hair color
}
