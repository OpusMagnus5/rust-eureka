fn main() {
    // definicja zmiennej, używamy snake case
    let apples_in_garden: i32 = 50;
    let oranges: i32 = 14 + 6;
    let fruits: i32 = apples_in_garden + oranges;
    
    // string interpolation
    println!("This year, my garden has {} apples.", apples_in_garden);
    println!("This year, my garden has {apples_in_garden} apples and {oranges} oranges.");
}