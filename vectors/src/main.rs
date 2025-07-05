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

// ============================================================================================== //

fn reading_elements() {
    let pizza_diameters = vec![8, 10, 12, 14];
    
    let pepperoni = String::from("Pepperoni");
    let mushroom = String::from("Mushroom");
    let sausage = String::from("Sausage");
    let pizza_toppings: Vec<String> = vec![pepperoni, mushroom, sausage]; // vector przejmuje ownera wartości
    
    let value: i32 = pizza_diameters[2]; // i32 wspiera Copy więc nie przejmuje ownera
    // let topping_value = pizza_toppings[2]; // to nie zadziała bo String nie wspiera Copy
    let reference = &pizza_toppings[2]; // OK
    
    let pizza_slice: &[String] = &pizza_toppings[1..3]; // tworzy referencje do sekcji vectora
}

// ============================================================================================== //

/*
 Metoda get wyciąga pojedynczą wartość po indexie i zwraca ją jako Option zawierającą referencje
*/

fn get_method() {
    let pepperoni = String::from("Pepperoni");
    let mushroom = String::from("Mushroom");
    let sausage = String::from("Sausage");
    let pizza_toppings: Vec<String> = vec![pepperoni, mushroom, sausage];

    let option: Option<&String> = pizza_toppings.get(2);
}

// ============================================================================================== //

fn ownership() {
    let pepperoni = String::from("Pepperoni");
    let mushroom = String::from("Mushroom");
    let sausage = String::from("Sausage");
    let pizza_toppings: Vec<String> = vec![pepperoni, mushroom, sausage];

    let mut delicious_topping = pizza_toppings; // przeniesienie ownera
    
    let topping_reference = &delicious_topping[1];
    delicious_topping.push(String::from("Olives"));
    // println!("Topping is {topping_reference}"); // ERROR dwie referencje do danych vectora w tym jedna mutable
}

fn main() {

    
}




































