
fn concatenation() {
    let mut full_name = String::from("Sylvester");
    let last_name = "Stallone";
    full_name.push(' '); // pojedynczy charakter
    full_name.push_str(last_name); // tylko do String można dodawać
    println!("{}", full_name);

    let mut full_name = String::from("Sylvester");
    let last_name = String::from("Stallone");
    full_name.push(' '); // pojedynczy charakter
    full_name.push_str(&last_name); // nie można dodawać Stringa, ale można użyć referencji a Rust zamieni to sobie na &str
    println!("{}", full_name);

    /*
     W przypadku + przekazujemy ownera first name do full_name, pod spodem jest wykonywana metoda add ze String
    */
    let first_name = String::from("Sylvester");
    let last_name = String::from("Stallone");
    let full_name = first_name + &last_name; // drugi argument musi być referencją lub &str
    println!("{}", full_name);
}

// ============================================================================================== //

fn format_macro() {
    let first_name = String::from("Sylvester");
    let last_name = String::from("Stallone");
    // nie przekazujemy ownera tylko referencje tak jak w println
    let full_name = format!("{} {}", first_name, last_name);
    println!("{}", full_name);
}

// ============================================================================================== //

fn string_methods() {
   let mut music_genres = "   Rock, Metal, Country, Rap    ";
    println!("{}", music_genres.trim());
    println!("{}", music_genres.trim_start());
    println!("{}", music_genres.trim_end());

    music_genres = music_genres.trim();
    println!("{}", music_genres.to_uppercase());
    println!("{}", music_genres.to_lowercase());
    println!("{}", music_genres.replace("Rock", "Jazz"));

    let genres: Vec<&str> = music_genres.split(", ").collect();
    println!("{:?}", genres);
}

// ============================================================================================== //

use std::io;

fn user_input() {
    let mut name = String::new();
    println!("What is your name?");
    // zwraca enum Result z liczbą bajtów wczytanych
    io::stdin().read_line(&mut name).expect("Failed to collect input"); // wczytuje również Enter z zatwierdzenia
    println!("Hello, {}", name);
}

fn main() {
    user_input();
}




































