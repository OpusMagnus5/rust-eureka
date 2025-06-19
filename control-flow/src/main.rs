fn main() {
    let season: &str = "summer";

    // if statement
    if season == "summer" {
        println!("School's out!");
    } else if season == "winter" { 
        println!("Brr, so cold!");
    } else { 
        println!("Lots of rain")
    }
    
    even_or_odd(12);
}

// przypisywanie wartości z ifa
fn even_or_odd(number: i32) {
    let result: &str = if number % 2 == 0 { "even" } else { "odd" }; // ternary operation nie istnieje, jedynie takie coś
    println!("The number is {result}");
}
