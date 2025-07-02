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

fn main() {

}
