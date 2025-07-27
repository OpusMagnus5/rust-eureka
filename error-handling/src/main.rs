

fn panic_macro() {
    /*
     Błędy w Rust dzielą się na błędy: Recoverable i Unrecoverable
     Recoverable to błędy które możemy obsłużyć w kodzie
     Unrecoverable to błędy które powoduję wywalenie się programu
    */
    
    panic!("Something went wrong"); // rzuca błąd
}

// ============================================================================================== //

use std::process;

fn process_module_and_exit_function() {
    /*
     Aby zamknąć program bez błędów używamy funkcji exit z modułu process
    */
    
    process::exit(0); // zamyka program z kodem, 0 - brak błędu
}

// ============================================================================================== //

fn standard_error() {
    /*
     Drukuje wiadomość do wyjścia err
    */
    println!("Some status update");
    eprintln!("Some error update");
}

// ============================================================================================== //

use std::fs::File;

fn opening_file() {
    let file = match File::open("story1.txt") { // otwiera plik i zwraca enum
        Ok(file) => file,
        Err(error) => {
            eprintln!("Something went wrong reading the file. The error was {error:?}");
            process::exit(1);
        }
    }; 
    println!("{file:?}");
}

fn main() {
    opening_file();
}
