/*
 <T> - definicja typu generycznego
 T - może być to dowolna litera bądź nawet nazwa
 Pod spodem tak naprawdę nie ma żadnego mechanizmu generyczności. W wynikowym pliku dla każdego
 typu użytego w funkcji zostaje rozpisana funkcja przy użyciu danego typu.
 
 Turbofish operator pozwala nadpisać defaultowy typ
*/
fn identity<T>(value: T) -> T {
    value
}

fn intro_to_generics() {
    println!("{}", identity::<i64>(5)); // nadpisujemy i32 przez i64
    println!("{}", identity(5.77));
}

// ============================================================================================== //

fn multiple_generics() {
    make_tuple("hello", 5);
}

/*
 <T, U> deklaracja dwóch różnych typów
*/
fn make_tuple<T, U>(first: T, second: U) -> (T, U) {
    (first, second)
}

// ============================================================================================== //

fn generics_in_structs() {
    let gold_chest: TreasureChest<&str> = TreasureChest {
        captain: String::from("Firebeard"),
        treasure: "Gold"
    };
}

#[derive(Debug)]
struct TreasureChest<T> {
    captain: String,
    treasure: T
}

impl TreasureChest<String> {
    fn clean_treasure(&mut self) {
        self.treasure = self.treasure.trim().to_string();
    }    
}

impl TreasureChest<[&str; 3]> {
    fn amount_of_treasure(&self) -> usize {
        self.treasure.len()
    }
}

impl<T> TreasureChest<T> { // Generyczna implementacja
    fn capital_captain(&self) -> String {
        self.captain.to_uppercase()
    }
}

fn generics_iand_impl() {
    let mut silver_chest: TreasureChest<String> = TreasureChest {
        captain: String::from("Firebeard"),
        treasure: String::from(" Silver ")
    };
    silver_chest.clean_treasure();

    let special_chest: TreasureChest<[&str; 3]> = TreasureChest {
        captain: String::from("Firebeard"),
        treasure: ["GOld", "Silver", "Platinium"]
    };
    special_chest.amount_of_treasure();
    
    silver_chest.capital_captain();
    special_chest.capital_captain();
}

// ============================================================================================== //

enum Cheesesteak<T> {
    Plain, // nie każdy enum musi wykorzystywać T
    Topic(T)
}

fn generics_and_enums() {
    let mushroom: Cheesesteak<&str> = Cheesesteak::Topic("mushroom");
    // mimo że nie wykorzystuje T musimy zadeklarować konkretny typ generyczny
    let cheesesteak: Cheesesteak<String> = Cheesesteak::Plain; 
}

fn main() {

}




























