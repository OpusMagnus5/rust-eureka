

fn panic_macro() {
    /*
     Błędy w Rust dzielą się na błędy: Recoverable i Unrecoverable
     Recoverable to błędy które możemy obsłużyć w kodzie
     Unrecoverable to błędy które powoduję wywalenie się programu
    */
    
    panic!("Something went wrong"); // rzuca błąd
}

fn main() {
    panic_macro();
}
