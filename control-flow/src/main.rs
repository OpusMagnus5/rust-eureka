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
    
    //match statement jest podobny do switch
    let evaluation: bool = true;
    
    match evaluation { 
        true => { println!("Yes"); }
        false => { println!("No"); }
    }
    
    // zwracanie wartości z match i krótszy zapis
    let result = match season {
        "summer" => println!("School's out!"),
        "winter" => println!("Brr's out!"),
        "autumn" | "spring" => println!("Lots of rain"), // przypisanie kilku wartości do jednej ścieżki
        _ => println!("Invalid season"), // _ działa jak default w java
    };

    // pattern match
    let number: i32 = 8;
    let number_result = match number {
        value if value % 2 == 0 => println!("{value} is an even number"),
        x if x % 2 != 0 => println!("{number} is an odd number"),
        _ => unreachable!(), // to jest makro które symbolizuje, że ten kod jest nieosiągalny
    };
    
    // loop - pętla bez końca
    let mut seconds: i32 = 0;
    loop {
        if seconds <= 0 { 
            println!("Blastoff!");
            break; 
        }
        
        if seconds % 2 == 0 { 
            println!("Skipping 3 seconds...");
            seconds -=3;
            continue;
        }
        
        println!("{seconds} seconds to blastoff...");
        seconds -= 1;
    }
}

// przypisywanie wartości z ifa
fn even_or_odd(number: i32) {
    let result: &str = if number % 2 == 0 { "even" } else { "odd" }; // ternary operation nie istnieje, jedynie takie coś
    println!("The number is {result}");
}






















