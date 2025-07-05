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

fn main() {

}
