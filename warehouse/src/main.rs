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

// ============================================================================================== //

/*
 Module - to kontener służący do organizacji i enkapsulacji powiązanego kodu. Dodatkowo tworzy namesapce
 dla wszystkiego co jest wewnątrz module
*/

/*
 funkcja main i plik main.rs to unnamed module od którego Rust zaczyna kompilować program
 Są trzy sposoby na tworzenie module:
    - słowo kluczowe mod i deklaracja rzeczy wewnątrz ciała
    - deklaracja modułu i utworzenie pliku z treścią modułu o takiej samej nazwie i w tym samym katalogu
    - utworzenie folderu o nazwie modułu a w nim pliku mod.rs - patrz orders i zadeklarowanie modułu w pliku
*/
use crate::inventory_file::ProductCategory;

mod inventory_file; // drugi sposób
mod orders; 

mod inventory { // pierwszy sposób
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

fn modules() {
    println!("The manager of our inventory is {}", inventory::MANAGER);
    println!("The manager of our inventory is {}", inventory_file::MANAGER);
    println!("The manager of our orders is {}", orders::MANAGER);
}

// ============================================================================================== //

fn public() {
    println!(
        "Our managers are {} and {}. We have {} square feet of floor space",
        inventory_file::MANAGER,
        orders::MANAGER,
        inventory_file::FLOOR_SPACE
    );
    
    inventory_file::talk_to_manager();
    
    let favourite_category = inventory_file::ProductCategory::Hammer;
    println!("My favourite category of item is {favourite_category:?}");
    
    let tall_ladder = inventory_file::Item { 
        name: String::from("Ladder-o-matic 2000"),
        category: ProductCategory::Ladder,
        quantity: 100
    };
    println!("{tall_ladder:#?}");
}

fn main() {
    
}






































