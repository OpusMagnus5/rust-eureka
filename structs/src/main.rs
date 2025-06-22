
/*
 Struct - obiekt w Java
 Rust ma 3 typy stucts:
    * named field structs
    * tuple-like structs
    * unit-like structs
 
*/
use std::time::Duration;

fn define_struct() {
    // definiowanie typu
    struct Coffee {
        price: f64,
        name: String,
        is_hot: bool,
    }
    
    // tworzenie instancji typu, jest odpowiedzialna za cleanup pól
    let mut mocha = Coffee {
        name: String::from("Mocha"),
        price: 4.99,
        is_hot: true
    };
    
    println!("{}", mocha.name); // sięganie do pól structa
    let favourite_coffee = mocha.name; // przekazanie ownera
    
    /*
     Aby móc zmieniać pola w struct trzeba zadeklarować go jako mut. Nie ma ograniczeń co do mutowalności
     np. tylko jednego pola - albo wszystkie albo wcale.
    */
    mocha.name = String::from("Caramel Macchiato");
}

/*
 Struct można deklarować poza funkcją wtedy jest dostępny w całym pliku
*/
#[derive(Debug)] // dostarczenie domyślnej implementacji Debug trait
struct Coffee {
    price: f64,
    name: String,
    is_hot: bool,
}

// ============================================================================================== //

fn create_struct_in_function() {
    let name = String::from("Latte");
    let coffee: Coffee = make_coffee(name, 4.99, true); // przeniesienie ownera name do struct
}

fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
    Coffee {
        price: price,
        name: name,
        is_hot: is_hot,
    }
}

// ============================================================================================== //

/*
 Jeśli nazwy parametrów lub zmiennych pasują do nazwy pól w struct, możemy użyć nazwy pola bez :
*/
fn short_syntax(name: String, price: f64, is_hot: bool) -> Coffee {
    Coffee {
        price,
        name,
        is_hot,
    }
}

// ============================================================================================== //

fn struct_update_syntax() {
    let coffee: Coffee = make_coffee(String::from("Latte"), 4.99, true);
    /*
     Kopiowanie wartości zapisywanych na stacku jest OK
    */
    let caramel_macchiato = Coffee {
        name: String::from("Caramel Macchiato"),
        price: coffee.price,
        is_hot: coffee.is_hot,
    };

    /*
     spread syntax, rozrzuca pozostałe brakujące wartości structa z innego structa
     !trzeba uważać zmienne z heap które nie są kopiowane tylko przenoszony owner!
    */
    let caramel_macchiato_2 = Coffee {
        name: coffee.name.clone(), // używamy clone aby nie przenosić ownera
        ..coffee
    };
}

// ============================================================================================== //

fn passing_struct_into_function() {
    let coffee: Coffee = make_coffee(String::from("Latte"), 4.99, true);
    drink_coffee(coffee); // przekazanie ownera

    let mut coffee2: Coffee = make_coffee(String::from("Latte"), 4.99, true);
    drink_coffee_ref(&mut coffee2);
}

fn drink_coffee(mut coffee: Coffee) { // domyślnie jest immutable ale możemy dodac mut
    println!("Drinking my delicious {}", coffee.name);
    coffee.is_hot = false;
}

fn drink_coffee_ref(coffee: &mut Coffee) { // przekazanie jako referencje, możemy równiez użyć mut
    println!("Drinking my delicious {}", coffee.name);
    coffee.is_hot = false;
}

// ============================================================================================== //

fn debug_trait_for_structs() {
    let coffee: Coffee = make_coffee(String::from("Latte"), 4.99, true);
    println!("{coffee:?}");
}

// ============================================================================================== //

#[derive(Debug)]
struct TaylorSwiftSong {
    title: String,
    release_year: u32,
    duration_secs: u32
}

/*
 W tym bloku deklarujemy funkcje powiązane ze structem.
 self jest tak jakby this w Java, może to być zarówno sam struct jak i referencja, mutable jak i immutable
    * Immutable struct value - self: TaylorSwiftSong / self: Self / self, przekazuje ownera
    * Mutable struct value - mut self: TaylorSwiftSong / mut self: Self / mut self, przekazuje ownera
    * Immutable struct reference - self: &TaylorSwiftSong / self: &Self / &self, nie przekazujemy ownera
    * Mutable struct reference - self: &mut TaylorSwiftSong / self: &mut Self / &mut self, przekazuje ownera
*/
impl TaylorSwiftSong {

    // Immutable struct value - self: TaylorSwiftSong / self: Self / self, przekazuje ownera
    fn display_song_info(self) {
        println!("Title: {}", self.title);
        println!("Release Year: {}", self.release_year);
        println!("Duration: {} seconds", self.duration_secs);
    }

    // Mutable struct value - self: mut TaylorSwiftSong / self: mut Self / mut self, przekazuje ownera
    fn double_length(mut self) {
        self.duration_secs *= 2;
        self.display_song_info();
    }

    // Immutable struct reference - self: &TaylorSwiftSong / self: &Self / &self, nie przekazujemy ownera
    fn display_song_info_ref(&self) {
        println!("Title: {}", self.title);
        println!("Years since Release: {}", self.years_since_release());
        println!("Duration: {} seconds", self.duration_secs);
    }

    // Mutable struct reference - self: &mut TaylorSwiftSong / self: &mut Self / &mut self, przekazuje ownera
    fn double_length_ref(&mut self) {
        self.duration_secs *= 2;
    }
    
    fn is_longer_than(&self, other: &Self) -> bool {
        self.duration_secs > other.duration_secs
    }

    fn years_since_release(&self) -> u32 {
        2025 - self.release_year
    }
}

/*
 Rust pozwala na kilka bloków impl dla typu które w ostateczności zostaną połączone.
*/

impl TaylorSwiftSong {
    /*
     Associated functions to nie są metody structa ale funkcje powiązane z jego namespacem np. String::from()
     Jeśli nie ma parametru self to funkcja jest uznawana jako associated.
    */
    fn new(title: String, release_year: u32, duration_secs: u32) -> Self {
        Self {
            title,
            release_year,
            duration_secs
        }
    }
}

fn struct_methods() {
    let mut song = TaylorSwiftSong {
        title: String::from("Blank Space"),
        release_year: 2014,
        duration_secs: 231
    };

    // song.display_song_info(); // Rust automatycznie przekazuje instancje do metody, ten sposób przekazuje ownera
    // song.double_length(); // Rust automatycznie przekazuje instancje do metody, ten sposób przekazuje ownera
    song.double_length_ref();
    song.display_song_info_ref();
    
    let all_to_well = TaylorSwiftSong {
        title: String::from("All to well"),
        release_year: 2012,
        duration_secs: 327
    };
    
    song.is_longer_than(&all_to_well); // przekazujemy tylko kolejne parametry, self jest ogarnięty przez Rust

    let blank_space = TaylorSwiftSong::new(String::from("Blank Space"), 2014, 231);
}

// ============================================================================================== //

/*
 Builder pattern
*/

#[derive(Debug)]
struct Computer {
    cpu: String,
    memory: u32,
    hard_drive_capacity: u32
}

impl Computer {
    fn new(cpu: String, memory: u32, hard_drive_capacity: u32) -> Self {
        Self {
            cpu,
            memory,
            hard_drive_capacity
        }
    }

    fn upgarde_cpu(&mut self, new_cpu: String) -> &mut Self {
        self.cpu = new_cpu;
        self
    }

    fn upgarde_memory(&mut self, new_memory: u32) -> &mut Self {
        self.memory = new_memory;
        self
    }

    fn upgarde_hard_drive_capacity(&mut self, new_hard_drive_capacity: u32) -> &mut Self {
        self.hard_drive_capacity = new_hard_drive_capacity;
        self
    }
}

fn builder_pattern() {
    let mut computer = Computer::new(String::from("M3 Max"), 64, 2);

    computer.upgarde_cpu(String::from("M4 Max"))
        .upgarde_memory(128)
        .upgarde_hard_drive_capacity(4);

}

fn main() {

}
































