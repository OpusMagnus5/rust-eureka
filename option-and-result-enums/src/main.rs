/*
 Option enum - to typ który działa jak Optional w Java, zawiera valid type or absent
 Option::None - absent value
 Option::Some(T) - present value
*/

fn option_enum() {
    let a: Option<i32> = Some(5);
    let b: Option<i32> = Option::<i32>::Some(5);
    
    let c = Option::<i32>::None;
    let c: Option<i32> = None;
}

// ============================================================================================== //

fn option_real_example() {
    let musical_instruments = [
        String::from("Guitar"),
        String::from("Drums"),
        String::from("Bass")
    ];
    
    /*
     Bezpieczniejsze sięganie po element tablicy, plus zwraca referencje
    */
    let bass: Option<&String> = musical_instruments.get(2);
    println!("{:?}", bass); // Some("Bass")
    
    let invalid_instrument: Option<&String> = musical_instruments.get(100);
    println!("{:?}", invalid_instrument); // None
}

// ============================================================================================== //

fn unwrap_and_expext_method() {
    /*
     extract - wyciąga wartość w przypadku Some a w przypadku None leci błąd
     expect - robi to co extract ale można dorzucić swój własny error message
    */

    let musical_instruments = [
        String::from("Guitar"),
        String::from("Drums"),
        String::from("Bass")
    ];
    
    let bass: Option<&String> = musical_instruments.get(2);
    let valid_instrument: &String = bass.unwrap();

    let x: &String = bass.expect("Unable to retrieve element");
}

// ============================================================================================== //

fn match_and_option() {
    let musical_instruments = [
        String::from("Guitar"),
        String::from("Drums"),
        String::from("Bass")
    ];
    
    let bass: Option<&String> = musical_instruments.get(2);
    play(bass);
    println!("{bass:?}"); // Option wspiera Copy trait więc nie transferu ownera
}

fn play(instrument: Option<&String>) {
    match instrument {
        Some(instrument) => println!("Playing the {instrument}"),
        None => println!("I have not instrument!")
    }
}

// ============================================================================================== //

fn returning_option() {
    let availability = is_item_in_stock(true, false);
}

fn is_item_in_stock(item_is_in_system: bool, item_is_in_stock: bool) -> Option<bool> {
    if item_is_in_system && item_is_in_stock { 
        Some(true)
    } else if item_is_in_system { 
        Some(false)
    } else { 
        None
    }
}

// ============================================================================================== //

/*
 Rust prelude to typy, funkcje, makro dostępne do użycia w każdym programie dlatego zamiast 
 Option::None możemy użyć po prostu None
*/

// ============================================================================================== //

/*
 W przypadku None unwrap_or zwraca przez nas zdefiniowaną wartość domyślną
*/

fn unwrap_or_method() {
    let missing_value: Option<i32> = None;
    println!("{}", missing_value.unwrap_or(0));
}

// ============================================================================================== //

#[derive(Debug, Copy, Clone)]
enum MyOption {
    Some(i32),
    None
}

impl MyOption {
    fn unwrap(self) -> i32 { // nie boimy się o ownera bo mamy Clone i Copy
        match self { 
            MyOption::Some(value) => value,
            MyOption::None => panic!("Uh oh") // wywołujemy błąd
        }
    }
    
    fn unwrap_or(self, fallback_value: i32) -> i32 {
        match self {
            MyOption::Some(value) => value,
            MyOption::None => fallback_value
        }
    }
}

fn building_option_from_scratch() {
    let some_option = MyOption::Some(100);
    println!("{}", some_option.unwrap());
    
    let none_option = MyOption::None;
    println!("{}", some_option.unwrap()); // error
}

// ============================================================================================== //

/*
 Result enum -> ma dwi wartości OK(T) i Err(E)
*/

fn result_enum() {
    let ok: Result<i32, &str> = Ok(5); // musimy podać typ również dla Err
    let disaster: Result<i32, &str> = Err("Something went wrong");
}

// ============================================================================================== //

fn result_enum_real_example() {
    let text = "50";
    let text_as_number = text.parse::<i32>(); // Dostarczamy typ bo Rust musi wiedzieć jaki 
    println!("{text_as_number:?}");
}

// ============================================================================================== //

fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 { 
        Err("Cannot divide by zero.".to_string())
    } else { 
        Ok(numerator / denominator)
    }
}

fn return_result_enum() {
    let result = divide(10.0, 2.0);
    
    match result { 
        Ok(calculation) => println!("Result: {calculation}"),
        Err(message) => println!("Error: {message}")
    }
}

// ============================================================================================== //

fn result_enum_methods() {
    let result = divide(10.0, 2.0);

    let unwrapped_result = result.unwrap(); // Ok zwraca value, Err rzuca wyjtek
    // result.expect("Some error");
    // result.unwrap_or(9.0);
    // result.is_ok();
    // result.is_err();
}

// ============================================================================================== //

fn operation(great_success: bool) -> Result<String, String> {
    if great_success { 
        Ok("Success".to_string())
    } else { 
        Err("Error".to_string())
    }
}

fn operation2(great_success: bool) -> Result<String, &'static str> { // &'static str - o tym będzie później
    if great_success {
        Ok("Success".to_string())
    } else {
        Err("Error")
    }
}

fn operation3(great_success: bool) -> Result<&'static str, &'static str> { // &'static str - o tym będzie później
    if great_success {
        Ok("Success")
    } else {
        Err("Error")
    }
}

fn result_enum_nuances() {
    let my_result = operation(true);
    let content = match my_result { // przniesienia ownera wrapowanych danych, zarówno Ok i Err
        Ok(message) => message,
        Err(message) => message
    };
    // println!("{}", my_result.unwrap()); // error, owner wcześniej przeniesiony
    
    let my_result2 = operation(true);
    let content2 = match &my_result2 { // nie przenosimy ownera wartości
        Ok(message) => message,
        Err(message) => message
    };
    // println!("{}", my_result.unwrap()); // error, owner wcześniej przeniesiony
    
    let my_result3 = operation2(true);
    let content = match my_result3 {
        Ok(message) => message,
        Err(message) => message.to_string() // musimy sprowadzić do tej samego typu
    };
    // println!("{}", my_result.unwrap()); // error, owner wcześniej przeniesiony

    let my_result3 = operation3(true);
    let content = match my_result3 { // &str wspiera Copy trait więc nie przenosimy ownera
        Ok(message) => message,
        Err(message) => message
    };
    println!("{}", my_result3.unwrap()); // nie ma błędu
}

// ============================================================================================== //

fn while_let_construct() {
    let mut sauces = vec!["Mayonnaise", "Ketchup", "Ranch"];
    
    while let Some(sauce) = sauces.pop() { // pętla dopóki warunek z dynamiczną wartością jest spełniony
        println!("The next sauce is {sauce}");
    }
}

// ============================================================================================== //

/*
Define a Food struct with a single `name` field
set to a String. Derive a Debug implementation.
 
Define a Restaurant struct with a `reservations` field
set to a u32 and a `has_mice_infestation` field set to
a bool. Derive a Debug implementation.
 
Define a `chef_special` method on the Restaurant.
The method will return the restaurant's famous
dish. It should return an Option containing a Food
struct.
 
If the restaurant has a mice infestation, return the
None variant. There is no chef special!
 
If the restaurant has less than 12 reservations, return
a Food instance with a name of "Uni Sashimi" wrapped in
the Some variant. If it has 12 or more reservations,
return a Food instance with a name of "Strip Steak"
instead, also wrapped in the Some variant.
 
Define a `deliver_burger` method on the Restaurant.
It should accept an `address` string slice; it will
represent where to deliver the order. It should
return a Result type where the Ok variant holds a Food
struct and the Err variant holds a String.
 
If the restaurant has a mice infestation, return the
Err variant containing a String of "Sorry, we have a
mice problem".
 
If the user's address is an empty string, return the Err
variant with a String of "No delivery address specified".
HINT: You can use the `is_empty` method on a string to check
if it has 0 characters.
https://doc.rust-lang.org/std/string/struct.String.html#method.is_empty
 
Otherwise, the delivery is good to go! Return the Ok
variant containing a Food struct with a `name` of "Burger".
 
In the `main` function, create a `Restaurant` instance
with 11 reservations and a mice infestation.
 
Invoke the `chef_special` method and print out its return
value. It should be the None variant.
 
Invoke the `deliver_burger` method with an argument of "123
Elm Street" and print out its return value. It should be
the Err variant.
 
Create another `Restaurant` instance with 15 reservations
and no mice infestation.
 
Invoke the `chef_special` method and print out its return
 value. It should be the Some variant with a "Strip Steak".
 
Invoke the `deliver_burger` method with an argument of an
empty address. Print out its return value. It should be the
Err variant.
 
Invoke the `deliver_burger` method again with an argument
of a valid address. Print out its return value. It should
be the Ok variant nesting a Food struct with a `name` of
"Burger".
*/

#[derive(Debug)]
struct Food {
    name: String
}

#[derive(Debug)]
struct Restaurant {
    reservations: u32,
    has_mice_infestation: bool
}

impl Restaurant {
    fn chef_special(&self) -> Option<Food> {
        if self.has_mice_infestation { 
            None
        } else if self.reservations < 12 { 
            Some(Food { name: String::from("Uni Sashimi") })
        } else { 
            Some(Food { name: String::from("Strip Steak") })
        }
    }
    
    fn deliver_burger(&self, address: &str) -> Result<Food, String> {
        if self.has_mice_infestation { 
            Err(String::from("Sorry, we have a mice problem"))
        } else if address.is_empty() { 
            Err(String::from("No delivery address specified"))
        } else { 
            Ok(Food { name: String::from("Burger") })
        }
    }
}

fn test() {
    let restaurant_with_mice = Restaurant { reservations: 11, has_mice_infestation: true };
    let chef_special = restaurant_with_mice.chef_special();
    println!("Chef special: {chef_special:?}");
    let burger_delivery = restaurant_with_mice.deliver_burger("123 Elm Street");
    println!("Burger delivery: {burger_delivery:?}");
    
    let fine_restaurant = Restaurant { reservations: 15, has_mice_infestation: false };
    let chef_special = fine_restaurant.chef_special();
    println!("Chef special: {chef_special:?}");
    let burger_delivery = fine_restaurant.deliver_burger("");
    println!("Burger delivery: {burger_delivery:?}");
}

fn main() {
    test();
}


























