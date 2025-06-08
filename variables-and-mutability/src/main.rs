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
    
    // variable shadowing, pozwala również na re-deklarowanie typu zmiennej
    let grams_of_protein: &str = "100.345";
    let grams_of_protein: f64 = 100.345;
}