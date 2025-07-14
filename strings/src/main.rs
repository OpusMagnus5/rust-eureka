
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

fn main() {
    format_macro();
}
