use std::collections::hash_set::{Difference, Union};
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
    let mut years_at_company: HashMap<&str, i32> = HashMap::from(data); // inny sposób inicjalizacji przez array tupli

    let ben: Option<i32> = years_at_company.remove("Ben"); // usuwa i zwraca option wartości dla klucza

}

// ============================================================================================== //

fn ownership() {
    /* Hashmap jest zapisywany na heapie i jako kolekcja jest też ownerem wartości zawartych w sobie */
    let mut coffee_pairings: HashMap<&str, &str> = HashMap::new(); // zmiana na &str, &str
    let drink = String::from("latte");
    let milk = String::from("Oat Milk");
    coffee_pairings.insert(&drink, &milk);
    println!("{coffee_pairings:#?}");

    /*
     println!("{milk}"); panic bo ownership został przejęty przez mape, żeby tego uniknąć
     możemy przekazać clony wartości lub referencje do nich ale wtedy nie możemy przekazywać &str ale
     gdy zadeklarujemy typ mapy z &str wtedy Rust poradzi sobie automatyczną konwersją &str -> &String
    */
}

// ============================================================================================== //

fn access_value_by_key() {
    let mut coffee_pairings: HashMap<&str, &str> = HashMap::new();
    let drink = String::from("latte");
    let milk = String::from("Oat Milk");
    coffee_pairings.insert(&drink, &milk);

    let value = coffee_pairings["latte"]; // dostęp do wartości ale jeśli jej nie ma to błąd
    println!("{value}");
    let option_value = coffee_pairings.get("latte"); // zwraca option z referencją do value
    let value = option_value.copied(); // tu jest kopia wartości ale nadal option
}

// ============================================================================================== //

fn overriding_value() {
    let mut coffee_pairings: HashMap<&str, &str> = HashMap::new();
    let drink = String::from("Latte");
    let milk = String::from("Oat Milk");
    coffee_pairings.insert(&drink, &milk);
    
    coffee_pairings.insert("Latte", "Pistachio Milk"); // nadpisanie wartości pod kluczem
}

// ============================================================================================== //

fn entry_method() {
    let mut coffee_pairings: HashMap<&str, &str> = HashMap::new();
    let drink = String::from("Latte");
    let milk = String::from("Oat Milk");
    coffee_pairings.insert(&drink, &milk);
    
    /*
     zwraca enum Entry który ma dwa warianty Occupied lub Vacant
    */
    let entry = coffee_pairings.entry("Latte");
    entry.or_insert("Some Milk"); // wstawia pod klucz wartość jeśli nie istnieje
}

// ============================================================================================== //

use std::collections::HashSet;
use std::hash::RandomState;

fn hashset() {
    /*
     Działa jak Set w java
    */
    let mut concert_queue = HashSet::<&str>::new();
    concert_queue.insert("Molly");
    concert_queue.insert("Megan");
    
    concert_queue.insert("Molly"); // nie zostanie dodana bo już istnieje, brak duplikatów
    concert_queue.remove("Franny"); // zwraca true lub false jeśli udało się usunąć
    concert_queue.contains("Megan"); // sprawdza czy zawiera element
    concert_queue.get("Molly"); // zwraca Option
    
    let mut movie_queue: HashSet<&str> = HashSet::new();
    movie_queue.insert("Boris");
    movie_queue.insert("Phil");

    let union: Union<&str, RandomState> = concert_queue.union(&movie_queue); // połączenie dwóch setów
    
    // zwraca to czego nie znaleziono w movie_queue a jest w concert_queue
    let difference: Difference<&str, RandomState> = concert_queue.difference(&movie_queue);
    
    concert_queue.symmetric_difference(&movie_queue); // zwraca połączenie setów ale wyrzuca rzeczy które są wspólne
    
    concert_queue.is_disjoint(&movie_queue); // zwraca true jeśli oba sety nie mają wspólnych wartości
    
    concert_queue.is_subset(&movie_queue); // zwraca true jeśli concert_queue zawiera się w movie_queue
    
    concert_queue.is_superset(&movie_queue); // sprawdza czy concert_queue nadzbiorem movie_queue - odwrotność is_subset
}

fn main() {
    ownership();
}






























