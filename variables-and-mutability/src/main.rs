#![allow(unused_variables)] // dyrektywa dla całego pliku

#[allow(unused_variables)] // dyrektywa dla kompilatora, dla całej funkcji, jeśli damy nad zmienną to tylko dla niej
fn main() {
    // definicja zmiennej, używamy snake case
    let apples_in_garden: i32 = 50;
    let oranges: i32 = 14 + 6;
    let fruits: i32 = apples_in_garden + oranges;
    
    // string interpolation
    println!("This year, my garden has {} apples.", apples_in_garden);
    println!("This year, my garden has {apples_in_garden} apples and {oranges} oranges.");
    println!("This year, my garden has {1} apples and {0} oranges.", oranges, apples_in_garden);

    let gym_reps: i32 = 10;
    // gym_reps = 15 // error - zmienne domyślnie są immutable

    let mut gym_reps_mut: i32 = 10;
    gym_reps_mut = 15; // przy użyciu mut deklarujemy zmienną mutable, ale nie można zmienić typu

    // variable shadowing, pozwala również na re-deklarowanie typu zmiennej w tym samym scope,
    // jeśli zadeklarujemy o takiej samej nazwie w nested scope to tworzymy tak naprawdę nową zmienną dostępną tylko w tym scope
    let grams_of_protein: &str = "100.345";
    let grams_of_protein: f64 = 100.345;

    // stałe mogą być deklarowane w dowolnym miejscu w kodzie, nie tak jak w przypadku zmiennych tylko w bloku funkcji
    // stałe nie mogą się zmieniać i nie można użyć słowa mut, musimy explicit zadeklarować typ stałej
    const TAX_RATE: f64 = 3.14;

    // type alias
    let mile_race_length: Meters = 1600;
}

// type alias, alias dla istniejącego typu, aby nadać dodatkowo kontekst
type Meters = i32;