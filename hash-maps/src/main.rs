use std::collections::HashMap;

fn create_hashmap() {
    let mut menu: HashMap<String, f64> = HashMap::new();
    menu.insert(String::from("Steak"), 29.99); // dodawanie elementu
    menu.insert(String::from("Tuna"), 14.99); // Przy zdublowaniu klucza poprzedni jest zastępowany
    
    println!("{menu:#?}");
    
    let mut country_capitals: HashMap<&str, &str> = HashMap::new();
    let mut country_capitals = HashMap::<&str, &str>::new(); //turbofish operator
    country_capitals.insert("France", "Paris");
    country_capitals.insert("Germany", "Berlin");
    
    println!("{country_capitals:?}")
}

// ============================================================================================== //

fn remove_method() {
    let data = [
        ("Bobby", 7),
        ("Grand", 4),
        ("Ben", 6)
    ];
    let years_at_company: HashMap<&str, i32> = HashMap::from(data); // inny sposób inicjalizacji przez array tupli

    let ben: Option<i32> = years_at_company.remove("Ben"); // usuwa i zwraca option wartości dla klucza

}

fn main() {
    create_hashmap();
}
