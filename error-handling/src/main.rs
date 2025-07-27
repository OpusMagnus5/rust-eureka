

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

use std::fs::{self, write, File};

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

// ============================================================================================== //

fn using_question_mark_with_option() {
    /*
     Jeśli metoda zwróci None to funkcja się zakończy zwracając None a jeśli Some to zwraca zawartość Some
    */

    let mut animals = vec!["Giraffe", "Monkey", "Zebra"];
    println!("{:?}", length_of_last_element(&mut animals));
    println!("{:?}", length_of_last_element(&mut animals));
    println!("{:?}", length_of_last_element(&mut animals));

    println!("{:?}", length_of_last_element(&mut animals)); // None
}

fn length_of_last_element(input: &mut Vec<&str>) -> Option<usize> {
    let last_element = input.pop()?;
    Some(last_element.len())
}

// ============================================================================================== //

/*
Define a `write_to_file` function. The function
should ask the user the following questions:

What file would you like to write to?
What would you like to write to the file?

Collect the user's two entries as Strings. If something
fails in either collection, propagate the error upwards;
the main function (the caller) will handle the error
later.

Then, use the file system module's `write` function
to write the user's specified contents to their
requested text file. The documentation for `write`
can be  found here:
https://doc.rust-lang.org/std/fs/fn.write.html

If the `write` function fails, propagate the error
upwards as well.

Your `write_to_file` function should return an
`io::Result`.

After you write to the file, return a `Ok` variant
storing the user's requested file name.

In the main function, use a match statement to react
to both variants of the returned Result enum. If
everything worked, output the text "Successfully
wrote to file { }" and interpolate the file name you
collected in the `write_to_file` function.

If there was any failure, output "There was an error:
{error}" to the standard error output and
interpolate the error. Then, exit the program with a
status code of 1.
 */

fn test() {
    match write_to_file() { 
        Ok(file_name) => println!("Successfully wrote to file {file_name}"),
        Err(error) => {
            eprintln!("There was an error: {error}");
            process::exit(1);
        }
    }
}

fn write_to_file() -> io::Result<String> {
    println!("What file would you like to write to?");
    let mut file_name = String::new();
    stdin().read_line(&mut file_name)?;

    println!("What would you like to write to the file?");
    let mut file_content = String::new();
    stdin().read_line(&mut file_content)?;
    write(file_name.trim(), file_content.trim())?;
    
    Ok(file_name.trim().to_string())
}

fn main() {
    test();
}



































