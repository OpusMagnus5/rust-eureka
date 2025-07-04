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

fn main() {
    
}


























