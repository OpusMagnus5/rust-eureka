fn main() {
    open_store("Brooklyn", 32);
    println!("Square 5 is {}",  square(5));
    println!("Square 5 is {}", implicit_square(5));
}

fn open_store(neighborhood: &str, building_number: i32) {
    println!("Opening my pizza store in {neighborhood} {building_number}");
}

fn square(number: i32) -> i32 {
    return number * number;
}

// jeśli ostatnia linia zwraca wartości, możemy pominąć return i musimy usunąć ; na końcu
fn implicit_square(number: i32) -> i32 {
    number * number
}

// unit jest to pusty tuple bez wartości
fn mystery() {
    
}

// explicit 
fn mystery_2() -> () {
}

fn blocks() {
    let multiplier: i32 = 3;

    //scope, izoluje zasięg
    let calculation: i32 = {
        let value: i32 = 5 + 4;
        value * multiplier //działa podobnie jak funkcja i zwraca ostatni wynik linii, ale nie za pomocą return
    };
}