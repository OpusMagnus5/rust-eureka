/*
 Vector w przeciwieństwie to tablicy może zmieniać rozmiar w czasie działania programu a dane są 
 połączone ze sobą w kolejności zarówno semantycznej jak i w pamięci komputera
*/

fn create_vector() {
    /*
     Tworzenie nowego vectora, trzeba podać typ jeśli tworzymy pusty.
    */
    let pizza_diameters: Vec<i32> = Vec::new();
    let pizza_diameters = Vec::<i32>::new();
    let pizza_diameters = vec![8, 10, 12]; // makro z możliwością inicjalizacji wartościami
}

// ============================================================================================== //

fn adding_removing_elements() {
    /*
     Aby dodać lub usunąć coś z vectora musimy oznaczyć go jako mut
    */
    let mut pizza_diameters = vec![8, 10, 12];
    pizza_diameters.push(14); // dodaje element na koniec
    /*
     dodajemy na konkretną pozycję, reszta wartości się przesuwa w prawo - nie podmienia wartości
    */
    pizza_diameters.insert(0, 6);
    let last_diameter: Option<i32> = pizza_diameters.pop(); // usuwa i zwraca ostatni element
    let removed_diameter: i32 = pizza_diameters.remove(2); // usuwa i zwraca element na danym indeksie
}

fn main() {

}
