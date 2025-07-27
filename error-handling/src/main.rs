

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
    let file = match File::open("story.txt") { // otwiera plik i zwraca enum
        Ok(file) => file,
        Err(error) => {
            eprintln!("Something went wrong reading the file. The error was {error:?}");
            process::exit(1);
        }
    };
    println!("{file:?}");
}

// ============================================================================================== //

use std::io::{ stdin, Read };

fn user_input() {
    println!("Please enter the name of the file you'd like to read:");
    let mut input = String::new();

    let user_requested_file = stdin().read_line(&mut input); // odczytanie linii

    if let Err(error) = user_requested_file {
        eprintln!("Something went wrong collecting user input. The error was {error}");
        process::exit(1);
    }

    let file = match File::open(input.trim()) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Something went wrong reading the file. The error was {error:?}");
            process::exit(1);
        }
    };
    println!("{file:?}");
}

// ============================================================================================== //

fn reading_file_content() {
    println!("Please enter the name of the file you'd like to read:");
    let mut input = String::new();

    let user_requested_file = stdin().read_line(&mut input); // odczytanie linii

    if let Err(error) = user_requested_file {
        eprintln!("Something went wrong collecting user input. The error was {error}");
        process::exit(1);
    }

    let mut file = match File::open(input.trim()) { // musi być mut aby odczytać, pewnie zmienia swoje wewnętrzne wskaźniki
        Ok(file) => file,
        Err(error) => {
            eprintln!("Something went wrong opening the file. The error was {error:?}");
            process::exit(1);
        }
    };
    
    let mut file_contents = String::new();
    /*
     aby móc wykorzystać metodę read_to_string musimy zadeklarować Read trait
    */
    let read_operation = file.read_to_string(&mut file_contents);
    if let Err(error) = read_operation {
        eprintln!("Something went wrong reading file as a string. The error was {error}");
        process::exit(1);
    }
    
    println!("{file_contents}")
}

fn main() {
    reading_file_content();
}



































