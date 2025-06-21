
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
    
    // tworzenie instancji typu
    let mocha = Coffee {
        name: String::from("Mocha"),
        price: 4.99,
        is_hot: true
    };
}

fn main() {
    
}
