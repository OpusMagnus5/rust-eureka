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

fn main() {
    
}
