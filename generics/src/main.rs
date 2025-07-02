/*
 <T> - definicja typu generycznego
 T - może być to dowolna litera bądź nawet nazwa
 Pod spodem tak naprawdę nie ma żadnego mechanizmu generyczności. W wynikowym pliku dla każdego
 typu użytego w funkcji zostaje rozpisana funkcja przy użyciu danego typu.
*/
fn identity<T>(value: T) -> T {
    value
}

fn intro_to_generics() {
    println!("{}", identity(5));
    println!("{}", identity(5.77));
}

fn main() {

}
