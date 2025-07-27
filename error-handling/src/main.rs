

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

use std::fs::{ self, File };

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

use std::io::{ stdin, self, Read };

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

// ============================================================================================== //

fn propagating_error() {
    let file_results: Result<String, io::Error> = read_file();
    
    match file_results { 
        Ok(contents) => println!("{contents}"),
        Err(error) => {
            eprintln!("There was an error {error}");
            process::exit(1);
        }
    }
}

fn read_file() -> Result<String, io::Error> {
    println!("Please enter the name of the file you'd like to read:");
    let mut input = String::new();

    let user_requested_file = stdin().read_line(&mut input);

    if let Err(error) = user_requested_file {
        return Err(error); // przekazywanie błędu wyżej
    }

    let mut file = match File::open(input.trim()) {
        Ok(file) => file,
        Err(error) => return Err(error) // przekazywanie błędu wyżej
    };

    let mut file_contents = String::new();
    
    let read_operation = file.read_to_string(&mut file_contents);
    if let Err(error) = read_operation {
        return Err(error); // przekazywanie błędu wyżej
    }

    Ok(file_contents)
}

// ============================================================================================== //

fn questionmark_operator() {
    let file_results: Result<String, io::Error> = read_file_2();

    match file_results {
        Ok(contents) => println!("{contents}"),
        Err(error) => {
            eprintln!("There was an error {error}");
            process::exit(1);
        }
    }
}

fn read_file_2() -> Result<String, io::Error> {
    println!("Please enter the name of the file you'd like to read:");
    let mut input = String::new();

    /*
     ? - jeśli metoda zwraca Ok program zwraca to co w OK, jeśli Err przekazuje go wyżej
    */
    stdin().read_line(&mut input)?;

    let mut file_contents = String::new();
    /* Można łączyć wywołania po ? */
    File::open(input.trim())?.read_to_string(&mut file_contents)?;

    Ok(file_contents)
}

// ============================================================================================== //

fn read_to_string_function() {
    let file_results: Result<String, io::Error> = read_file_3();

    match file_results {
        Ok(contents) => println!("{contents}"),
        Err(error) => {
            eprintln!("There was an error {error}");
            process::exit(1);
        }
    }
}

fn read_file_3() -> Result<String, io::Error> {
    println!("Please enter the name of the file you'd like to read:");
    
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    
    fs::read_to_string(input.trim())
}

fn main() {
    questionmark_operator();
}



































