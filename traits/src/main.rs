use std::collections::HashMap;
use std::fmt::Display;
/*
 Train jest interfejsem jak w Java. Definiuje metody jakie trzeba zaimplementować aby spełnić kontrakt.
*/

trait Accommodation {
    fn book(&mut self, name: &str, nights: u32);
}

/* ============================================================================================== */

#[derive(Debug)]
struct Hotel<T> {
    name: T,
    reservations: HashMap<String, u32>
}

impl<T> Hotel<T> {
    fn new(name: T) -> Self {
        Self {
            name,
            reservations: HashMap::new()
        }
    }
}

impl<T: Display> Hotel<T> {
    fn summarize(&self) -> String {
        format!("{}: {}", self.name, self.get_description())
    }
}

// implementacja dla Hotel
impl<T> Accommodation for Hotel<T> {
    fn book(&mut self, name: &str, nights: u32) {
        self.reservations.insert(name.to_string(), nights);
    }
}

/* ============================================================================================== */

#[derive(Debug)]
struct AirBnB {
    host: String,
    guest: Vec<(String, u32)>
}

impl AirBnB {
    fn new(host: &str) -> AirBnB {
        Self {
            host: host.to_string(),
            guest: vec![]
        }
    }
}

impl Accommodation for AirBnB {
    fn book(&mut self, name: &str, nights: u32) {
        self.guest.push((name.to_string(), nights));
    }
}

fn implementing_trait() {
    let mut hotel = Hotel::new("The Luxe");
    println!("{}", hotel.summarize());
    hotel.book("Piers", 5);
    println!("{hotel:#?}");
    
    let mut airbnb = AirBnB::new("Peter");
    println!("{}", airbnb.get_description());
    airbnb.book("Piers", 3);
    println!("{airbnb:#?}")
}

/* ============================================================================================== */

fn book_for_one_night(entity: &mut impl Accommodation, guest: &str) {
    entity.book(guest, 1);
}

fn traits_for_function_parameter_constraints() {
    let mut hotel = Hotel::new("The Luxe");
    hotel.book("Piers", 5);
    book_for_one_night(&mut hotel, "Damian");
    println!("{hotel:#?}");
}

/* ============================================================================================== */

fn book_for_one_night_2<T: Accommodation>(entity: &mut T, guest: &str) {
    entity.book(guest, 1);
}

// tu możemy przekazać dwie różne implementacje Accommodation
fn mix_and_match(first: &mut impl Accommodation, second: &mut impl Accommodation, guest: &str) {
    first.book(guest, 1);
    second.book(guest, 2);
}

// tutaj muszą być to dwie takie same implementacje Accommodation
fn mix_and_match_2<T: Accommodation>(first: &mut T, second: &mut T, guest: &str) {
    first.book(guest, 1);
    second.book(guest, 2);
}

// a tu wspieramy dwa rożne typy Accommodation
fn mix_and_match_3<T: Accommodation, U: Accommodation>(first: &mut T, second: &mut U, guest: &str) {
    first.book(guest, 1);
    second.book(guest, 2);
}

/* ============================================================================================== */

trait Description {
    fn get_description(&self) -> String {
        String::from("A wonderful place to stay!")
    }
}

impl<T:  Display> Description for Hotel<T> {
    fn get_description(&self) -> String {
        format!("{} is the pinnacle of the luxury", self.name)
    }
}

impl Description for AirBnB {
    fn get_description(&self) -> String {
        format!("Please enjoy {}'s apartment", self.host)
    }
}

fn mix_and_match_4(
    first: &mut (impl Accommodation + Description), // musi również wspierać Description
    second: &mut impl Accommodation, 
    guest: &str
) {
    first.book(guest, 1);
    second.book(guest, 2);
}

// przykład z generics
fn book_for_one_night_3<T: Accommodation + Description>(entity: &mut T, guest: &str) {
    entity.book(guest, 1);
}

/* ============================================================================================== */

/*
 za where możemy zdefiniować generyki zamiast w < >
*/
fn mix_and_match_5<T, U>(
    first: &mut T, // musi również wspierać Description
    second: &mut U,
    guest: &str
) where
    T: Accommodation + Description,
    U: Accommodation
{
    first.book(guest, 1);
    second.book(guest, 2);
}

/* ============================================================================================== */

fn choose_best_place_to_stay() -> impl Accommodation + Description {
    Hotel::new("The Lux")

    /* nie możemy zwracać różnych typów implementacji na podstawie logiki
    if true {
        Hotel::new("The Lux")
    } else {
        AirBnB::new("AirBnB")
    }*/
}

/* ============================================================================================== */

fn trait_bounds_to_conditionally_implement_methods() {
    let hotel1 = Hotel::new(String::from("The Lux"));
    println!("{}", hotel1.summarize());

    let hotel2 = Hotel::new("The Golden Standards");
    println!("{}", hotel2.summarize());

    let hotel3 = Hotel::new(vec!["The", "Sweet", "Escape"]);
    // println!("{}", hotel3.summarize()); nie zadziała bo vec nie implementuje Display
}

/* ============================================================================================== */

fn trait_object() {
    let hotel = Hotel::new("The Luxe");
    let airbnb = AirBnB::new("Peter");

    //let stays = vec![hotel, airbnb]; nie zadziała bo tą różne typy mimo wspólnego interfejsu
    //let stays: Vec<impl Description> = vec![hotel, airbnb]; to też nie zadziała

    /*
      dyn - oznacza że typy będą runtime inwestygowane, będzie to również wolniejsze działanie
      bo nie zostanie zoptymalizowane przez kompilator
      dynamic dispatch współpracuje jedynie z referencjami
    */
    let stays: Vec<&dyn Description> = vec![&hotel, &airbnb];
    println!("{}", stays[0].get_description());
}

/* ============================================================================================== */

/*
 Jeśli używamy jakiejś metody z traita musimy zaimportować trait do obecnego pliku
*/

use std::ops::Add;
use std::str::FromStr;
fn trait_must_be_in_scope_to_use_its_definitions() {
    let a = 5;
    let b = 10;
    let sum = a.add(b); // std::ops::Add
    
    let numeric_count = u64::from_str("5"); // std::str::FromStr
}

fn main() {
    trait_object();
}



































