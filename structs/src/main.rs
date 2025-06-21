
/*
 Struct - obiekt w Java
 Rust ma 3 typy stucts:
    * named field structs
    * tuple-like structs
    * unit-like structs
 
*/
fn define_struct() {
    // definiowanie typu
    struct Coffee {
        price: f64,
        name: String,
        is_hot: bool,
    }
    
    // tworzenie instancji typu, jest odpowiedzialna za cleanup pól
    let mut mocha = Coffee {
        name: String::from("Mocha"),
        price: 4.99,
        is_hot: true
    };
    
    println!("{}", mocha.name); // sięganie do pól structa
    let favourite_coffee = mocha.name; // przekazanie ownera
    
    /*
     Aby móc zmieniać pola w struct trzeba zadeklarować go jako mut. Nie ma ograniczeń co do mutowalności
     np. tylko jednego pola - albo wszystkie albo wcale.
    */
    mocha.name = String::from("Caramel Macchiato");
}

fn main() {
    
}
































