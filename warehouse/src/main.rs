/*
 cargo new - tworzy Rust package 
 package - folder z plikiem Cargo.toml w którym są metadane o package jak nazwa, wersja itp., może 
 zawierać wiele crates
 crate - to najmniejsza jednostka kodu którą Rust kompiluje w danym czasie czego wynikiem jest binarka
 binary crate - to crate który kompiluje się do binarki, ma funkcję main która uruchamia program
 library crate - eksportuje funkcjonalność do wykorzystania w innych programach Rust, nie posiada funkcji main,
 nie tworzy binarki
 W przypadku binarki Rust szuka pliku main.rs a w przypadku biblioteki lib.rs, w projekcie mogą oba pliki jednocześnie
*/

/*
 Module - to kontener służący do organizacji i enkapsulacji powiązanego kodu. Dodatkowo tworzy namesapce
 dla wszystkiego co jest wewnątrz module
*/

mod inventory {
    const FLOOR_SPACE: i32 = 10000;
    pub const MANAGER: &str = "Ivan Inventory"; // musimy użyć pub aby można było uzyskać dostęp z zewnątrz
    
    #[derive(Debug)]
    enum ProductCategory {
        Ladder,
        Hammer
    }
    
    #[derive(Debug)]
    struct Item {
        name: String,
        category: ProductCategory,
        quantity: u32
    }
    
    fn talk_to_manager() {
        println!("Hey, {MANAGER}, how's your coffee?");
    }
}

/*
 Aby dostać się do czegoś w module musimy użyć nazwy modułu a następnie scope resolution operator :: 
 Wszystko domyślnie w module ma scope private.
*/
fn main() {
    println!("The manager of our inventory is {}", inventory::MANAGER)
}
