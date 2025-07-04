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

fn main() {
    
}


























