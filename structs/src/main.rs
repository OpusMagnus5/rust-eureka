
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

/*
 Struct można deklarować poza funkcją wtedy jest dostępny w całym pliku
*/
struct Coffee {
    price: f64,
    name: String,
    is_hot: bool,
}

fn create_struct_in_function() {
    let name = String::from("Latte");
    let coffee: Coffee = make_coffee(name, 4.99, true); // przeniesienie ownera name do struct
}

fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
    Coffee {
        price: price,
        name: name,
        is_hot: is_hot,
    }
}

fn main() {
    
}
































