/*
    Typy skalarne to takie, które mają tylko jedną wartość w sobie
    Signed zaczyna się od i, a Unsigned od u
*/
fn main() {
    // można używać zapisu z _ dla liczb, dla zmiennoprzecinkowych nie ma unsigned
    let eight_bit: u8 = 255;
    let sixteen_bit: u16 = 65_535;
    let thirty_two_bit: u32 = 4_294_967_295;
    let sixty_four_bit: u64 = 18_446_744_073_709_551_615;
    let one_byte: i8 = -128;
    let two_byte: i16 = -32_768;
    let four_byte: i32 = -2_147_483_648;
    let eight_byte: i64 = -9_223_372_036_854_775_808;
    
    // Inny sposób na deklarowanie typu po wartości zmiennej
    let some_value = 255u8;
    
    // usize i isize to są aliasy dla unsigned i signed, ale wielkość bitów zależy od architektury, na jakim komputerze się uruchamia
    let days: usize = 55;
    
    // string literal jest, wtedy gdy kompilator wie już, w trakcie jaki to będzie
    let string_literal = "Hello, world!";
    // raw string literal nie analizuje specjalnych znaków, tylko wstawia to, co piszemy, poprzedzamy literą r
    let raw_string = r"C:\\MyDoc\new";
    
    // formatowanie liczb
    // .2 - 2 cyfry po przecinku
    println!("The current value of pi is {eight_bit:.2}");
    println!("The current value of pi is {:.2}", eight_bit);
    
    //casting
    let miles_away: i32 = 50;
    let kilometers_away: i8 = miles_away as i8;
    
    // mathematical operations
    let five = 5;
    let six = 6;
    let sum = five + six;
    let difference = five - six;
    let product = five * six;
    let quotient = five / six;
    let remainder = five % six;
    
    // assignment operators
    let mut year = 2025;
    year = year + 1;
    year += 1;

    // boolean
    let is_handsome: bool = true;
    let inverse_is_handsome = !is_handsome;
    five.is_positive();
    
    // equality
    let is_equal = five == six;
    let is_not_equal = five != six;
    
    // logical operators
    let is_true = is_equal && is_not_equal;
    let is_false = is_equal || is_not_equal;
    
    // character type
    let z: char = 'z';
    let emoij: char = '🤣';
    emoij.is_alphabetic();
    
    // array nie zmienia wielkości
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    array[0];
    // nadal nie może zmieniać wielkości, ale można zmieniać wartości w tablicy
    let mut mut_array: [i32; 5] = [1, 2, 3, 4, 5];
    mut_array[0] = 10;
}

// Metoda to funkcja, którą można wykonać na obiekcie / typie

fn methods() {
    let abs_value = 10i32.abs();
}

// Trait
fn display_trait() {
    // trait to kontrakt, że typ wspiera jedną lub więcej metod (type implementuje trait)
    // Typ może implementować kilka trait.
    // Display trait wymaga, aby typ mógł być reprezentowany przez string (user-friendly).
    // Display trait udostępnia metodę format, jest używany przy interpolacji.
    // Większość typów wbudowanych w Rust implementuje ten trait, ale np. bardziej złożone już nie np. array
}

fn debug_trait() {
    // działa jak display, ale ma za zadanie reprezentować typ w postaci stringa dla debugu
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", array); // ? formatuje za pomocą Debug trait
    println!("{array:?}");
    println!("{array:#?}") // #? formatuje za pomocą Debug, ale jest prettier
}

























