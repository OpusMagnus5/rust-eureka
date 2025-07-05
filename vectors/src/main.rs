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

// ============================================================================================== //

fn writing_elements() {
    let pepperoni = String::from("Pepperoni");
    let mushroom = String::from("Mushroom");
    let sausage = String::from("Sausage");
    let mut pizza_toppings: Vec<String> = vec![pepperoni, mushroom, sausage];
    
    pizza_toppings[1] = String::from("Olives"); // nadpisanie na danej pozycji
    
    // let target_topping = pizza_toppings[2] // nie zadziała bo nie możemy pożyczyć częściowo danych
    let target_topping = &mut pizza_toppings[2];
    // let another_topping = &pizza_toppings[2]; // nie można mieć dwóch referencji gdy jest jedna mut ale ⬇️
    target_topping.push_str(" and Meatballs"); // concatenate strings
    /*
     OK bo Rust ma coś takiego jak Lifetime i widzi że ostatnie użyciu mut referencji jest przed użyciem nowej
    */
    let another_topping = &pizza_toppings[2];
    println!("{pizza_toppings:#?}")
}

// ============================================================================================== //

fn vector_capacity() {
    /*
     Jeśli zacznie brakować miejsca w przydzielonej pamięci, Rust przenosi cały vector w nowe miejsce,
     dlatego też Rust zabrania kilku mutable references ponieważ dodając coś do vectora może mu się zmienić
     adres w pamięci.
    */

    let mut seasons: Vec<&str> = Vec::with_capacity(4); // tworzy nowy wektor o zadanej pojemności
    println!("Length: {}. Capacity {}", seasons.len(), seasons.capacity()); // 0, 4
    
    seasons.push("Summer");
    seasons.push("Fall");
    seasons.push("Winter");
    seasons.push("Spring");
    println!("Length: {}. Capacity {}", seasons.len(), seasons.capacity()); // 4, 4

    seasons.push("Summer");
    println!("Length: {}. Capacity {}", seasons.len(), seasons.capacity()); // 5, 8 Rust musiał przenieść do większej pamięci
}

// ============================================================================================== //

/*
Let's model a file system on a computer.
 
Define a File struct with a `name` field set to a
String. Derive a Debug implementation.
 
Define a Folder struct with a `name` field set to
a String and a `contents` field set to a vector of
File structs. Derive a Debug implementation.
 
On the Folder struct...
 
Define a `new` constructor function that accepts a
`name` String. The method should create and return
a new Folder with that name. For the `contents` field,
provide a hardcoded empty vector.
 
Define a `create_file` method that accepts a `name`
String. The method should create a new File with that
name and add it to the end of the `contents` vector.
 
Define a `delete_file` method that accepts an `index`
parameter of type `usize`. The method should remove the
File at the specified index position from the `contents`
vector. It should also return the File.
 
Define a `get_file` method that accepts an `index`
parameter of type `usize`. The method should return
an Option containing a reference to the File at
that index position.
 
In the `main` function, use the `new` function to
create a Folder instance with a `name` of your choosing.
 
Call the `create_file` method two times. Print out
the Folder in Debug format.
 
Delete one of the two files using the `delete_file`
method. Print out the Folder in Debug format.
 
Call the `get_file` method. Use a match statement
to react to both Option variants. For the Some variant,
print out the File in Debug format. For the None variant,
print out the text "There was no file".
*/

#[derive(Debug)]
struct File {
    name: String
}

#[derive(Debug)]
struct Folder {
    name: String,
    contents: Vec<File>
}

impl Folder {
    fn new(name: String) -> Folder {
        Folder { name, contents: Vec::new() }
    }
    
    fn create_file(&mut self, name: String) {
        self.contents.push(File { name });
    }
    
    fn delete_file(&mut self, index: usize) {
        self.contents.remove(index);
    }
    
    fn get_file(&self, index: usize) -> Option<&File> {
        self.contents.get(index)
    }
}

fn test() {
    let mut games_folder = Folder::new("games".to_string());
    
    games_folder.create_file("Plague Tale".to_string());
    games_folder.create_file("Ghost of Tsushima".to_string());
    println!("Folder: {games_folder:?}");
    
    games_folder.delete_file(0);
    println!("Folder: {games_folder:?}");
    
    match games_folder.get_file(0) {
        Some(file) => println!("File 0: {file:?}"),
        None => println!("There was no file index 0")
    }

    match games_folder.get_file(1) {
        Some(file) => println!("File 1: {file:?}"),
        None => println!("There was no file index 1")
    }
}

fn main() {
    test();
}




































