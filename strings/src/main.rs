
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

// ============================================================================================== //

/*
Define a `make_money` function that accepts a mutable
String reference. The function should concatenate
the characters "$$$" to the end of the String.
Invoke the function in `main`.

Define a `trim_and_capitalize` function that accepts
a string slice. It should return a String with
all whitespace removed and all characters in uppercase.
Invoke the function in `main`.

Define an `elements` function that accepts a string
slice. It should split the string by all occurrences
of the `!` symbol and return a vector of the string
slices. Invoke the function in `main`.

Example:
elements("Gold!Silver!Platinum")
=> Vector of ["Gold", "Silver", "Platinum"]

Define a `get_identity` function. The function should
ask the user for their first and last name in TWO
steps (i.e., collect user input twice). Make sure
to communicate the instructions to the user.
For each Result enum you receive, call the `expect`
method and provide a custom error message (like
"Failed to collect first name"). Return a String
with the first and last names combined. Invoke
the `get_identity` function in `main`, and output the
returned String.

Example:
fn main() {
  let name = get_identity();
   println!("{name}"); // Bill Murray
}
*/

fn test() {
    let mut money = String::from("1000");
    make_money(&mut money);
    println!("{}", money);

    println!("{}", trim_and_capitalize("   hello world   "));

    println!("{:?}", elements("Gold!Silver!Platinum"));

    println!("{}", get_identity());
}

fn make_money(money: &mut String) {
    money.push_str("$$$");
}

fn trim_and_capitalize(text: &str) -> String {
    text.trim().to_uppercase()
}

fn elements(text: &str) -> Vec<&str> {
    text.split("!").collect()
}

fn get_identity() -> String {
    println!("Please enter your first name");
    let mut first_name = String::new();
    io::stdin().read_line(&mut first_name).expect("Failed to collect input");
    println!("Please enter your last name");
    let mut last_name = String::new();
    io::stdin().read_line(&mut last_name).expect("Failed to collect input");

    format!("{} {}", first_name.trim(), last_name.trim())
}

fn main() {
    test();
}




































