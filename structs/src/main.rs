
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

// ============================================================================================== //

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

// ============================================================================================== //

/*
 Jeśli nazwy parametrów lub zmiennych pasują do nazwy pól w struct, możemy użyć nazwy pola bez :
*/
fn short_syntax(name: String, price: f64, is_hot: bool) -> Coffee {
    Coffee {
        price,
        name,
        is_hot,
    }
}

// ============================================================================================== //

fn struct_update_syntax() {
    let coffee: Coffee = make_coffee(String::from("Latte"), 4.99, true);
    /*
     Kopiowanie wartości zapisywanych na stacku jest OK
    */
    let caramel_macchiato = Coffee {
        name: String::from("Caramel Macchiato"),
        price: coffee.price,
        is_hot: coffee.is_hot,
    };

    /*
     spread syntax, rozrzuca pozostałe brakujące wartości structa z innego structa
     !trzeba uważać zmienne z heap które nie są kopiowane tylko przenoszony owner!
    */
    let caramel_macchiato_2 = Coffee {
        name: coffee.name.clone(), // używamy clone aby nie przenosić ownera
        ..coffee
    };
}

fn main() {
    
}
































