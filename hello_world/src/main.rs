/*
    main - podstawowa funkcja wywoływana przy starcie programu
    println - to macro (wyróżnione wykrzyknikiem), działa jak funkcja

    Aby ręcznie skompilować kod, wykonujemy rustc main.rs
    rustfmt - formatuje wizualnie kod np. rustfmt main.rs
    cargo fmt - formatuje kod we wszystkich plikach w katalogu i podkatalogach?
    cargo build - buduje cały projekt (default in debug mode)
        --debug - kompilacja szybka i nieoptymalizowana do debugowania, ponieważ dodaje dodatkowe metadane, aby zidentyfikować bugs
        --release - kompilacja wolna i optymalizowana do produkcji, zmniejsza rozmiar wynikowego pliku
    cargo clean - działa podobnie jak mvn clean
    cargo run - buduje i uruchamia projekt domyślnie in debug mode
        --quiet - nie wyświetla informacji o budowie
    cargo check - sprawdza kod pod kątem błędów kompilacji bez budowania projektu
*/
fn main() {
    println!("Hello world!");
}
