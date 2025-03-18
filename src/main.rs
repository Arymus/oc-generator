mod character_traits; // Imports character_traits.rs
use character_traits::POS_TRAITS; // Imports the POS_TRAITS array from character_traits.rs
use character_traits::NEG_TRAITS; // Imports the NEG_TRAITS array from character_traits.rs
use character_traits::COLORS; // Imports the HAIR_COLOR array from character_traits.rs
use rand::Rng; // Use the rand::Rng crate
use std::io; // Use the standard input/output crate

fn gen_traits() -> (String, String) {
    let mut rng = rand::rng(); // Apparently this function doesn't exist, but it works so... 
    let mut pos_string: Vec<String> = Vec::new(); // Defines an empty vec for containing strings, specifically negative traits
    let mut neg_string: Vec<String> = Vec::new(); // Defines an empty vec for contaning strings, specifically positive traits

    // For every item in the POS_TRAITS array (character_traits.rs), convert to a string and add it to the traits_string vector
    for pos in &POS_TRAITS {
        pos_string.push(pos.to_string()); // Add the positive traits to the pos_traits vector
    }

    // For every item in the NEG_TRAITS array (character_traits.rs), convert to a string and add it to the traits_string vector
    for neg in &NEG_TRAITS {
        neg_string.push(neg.to_string()); // Add the negative character traits to the neg_traits vector
    }

    // Select a random number between 0 and 12
    let pos_selection: usize= rng.random_range(0..=12); // Random number between 0 and 12, including 12
    let neg_selection: usize = rng.random_range(0..=12); // Random number between 0 and 12, including 12

    // Assign the selected numbers to a variable
    let pos_final: String = pos_string[pos_selection].clone();
    let neg_final: String = neg_string[neg_selection].clone();

    // Return the result of positive and negative traits as a tuple
    return ( pos_final, neg_final );
}

// Initializes a function that returns a tuple of two strings
fn gen_color() -> (String, String) {
    let mut rng = rand::rng();  // Apparently this function doesn't exist, but it works so...
    let mut hair_colors: Vec<String> = Vec::new(); // Defines an empty vector for containing strings, specifically hair colors
    let mut eye_colors: Vec<String> = Vec::new(); // Defines an empty vector for containining strings, specifically eye colors

    // For each color in the COLORS array (character_traits.rs)
    for color in &COLORS {
        hair_colors.push(color.to_string()); // Convert the color to a string and add it to the hair_colors vector
	eye_colors.push(color.to_string()); // Convert the color to a string and add it to the eye_color vector
    }

    // Randomly select hair and eye colors
    let hair_color_selection: usize = rng.random_range(0..=hair_colors.len() - 1); // Pick a random number between 0 and the last index of the hair_colors array
    let eye_color_selection: usize = rng.random_range(0..=eye_colors.len() - 1); // Generates a random number between 0 and the last index of the eye_colors array

    // Define the selected hair and eye colors
    let hair_color_final: String = hair_colors[hair_color_selection].clone();
    let eye_color_final: String = eye_colors[eye_color_selection].clone();
    return ( hair_color_final, eye_color_final  );
}

fn main() {
    // Print input prompt
    println!("OC Generator");
    println!("What's your OC's name?");

    let mut input: String = String::new(); // Create a new string
    io::stdin().read_line(&mut input).expect("Failure reading string"); // Read the string and panic with a message if there's an error

    println!("Name: {}Description:\n{:?}, {:?}, {:?}", input,
    gen_traits().0 + ", " + &gen_traits().1,
    gen_traits().0 + ", " + &gen_traits().1,
    gen_traits().0 + ", " + &gen_traits().1); // Print the selected index 3 times

    println!("Hair color: {:?}\nEye color: {:?}", gen_color().0, gen_color().1); // Print the selected hair color
}
