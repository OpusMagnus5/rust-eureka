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

fn main() {

}
