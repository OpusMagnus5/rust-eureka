use traits::lodging::{ Hotel, AirBnB, Accommodation, Description };
use traits::utils;

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

fn traits_for_function_parameter_constraints() {
    let mut hotel = Hotel::new("The Luxe");
    hotel.book("Piers", 5);
    utils::book_for_one_night(&mut hotel, "Damian");
    println!("{hotel:#?}");
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

/* ============================================================================================== */

/*
 Associated constant jest to stała zadeklarowana w trait;
*/

trait Taxable: Investment { // rozszerzenie Investment - jak dziedziczenie
    const TAX_RATE: f64 = 0.25;
    
    fn tax_bill(&self) -> f64 {
        self.amount() * Self::TAX_RATE // sięganie po constant
    }
}

#[derive(Debug)]
struct Income {
    amount: f64
}

impl Investment for Income {
    fn amount(&self) -> f64 {
        self.amount
    }

    fn set_amount(&mut self, new_amount: f64) {
        self.amount = new_amount;
    }
}

impl Taxable for Income { }

#[derive(Debug)]
struct Bonus {
    value: f64
}

impl Investment for Bonus {
    fn amount(&self) -> f64 {
        self.value
    }

    fn set_amount(&mut self, new_amount: f64) {
        self.value = new_amount;
    }
}

impl Taxable for Bonus {
    const TAX_RATE: f64 = 0.50; // możemy nadpisywać stałe w implementacjach
}

fn associated_constants_in_trait() {
    let mut income = Income { amount: 50000.50 };
    println!("Total tax owed: ${:.2}", income.tax_bill());
    income.double_amount();
    println!("Total tax owed: ${:.2}", income.tax_bill());
    
    let mut bonus = Bonus { value: 10000.23 };
    println!("Bonus tax owed: ${:.2}", bonus.tax_bill());
    bonus.double_amount();
    println!("Bonus tax owed: ${:.2}", bonus.tax_bill());
    
    let weekend = QualityTime { minutes: 120.0 };
    println!("Relaxation time: ${:.2}", weekend.amount());
    
}

/* ============================================================================================== */

trait Investment {
    fn double_amount(&mut self) {
        self.set_amount(self.amount() * 2.0)
    }
    fn amount(&self) -> f64; // dodajemy getera żeby zrobić default implementacje tax_bill

    fn set_amount(&mut self, new_amount: f64); // dodajemy setera żeby zrobić default implementacje double_amount
}

struct QualityTime {
    minutes: f64
}

impl Investment for QualityTime {
    fn amount(&self) -> f64 {
        self.minutes
    }

    fn set_amount(&mut self, new_amount: f64) {
        self.minutes = new_amount
    }
}

fn main() {
    associated_constants_in_trait();
}



































