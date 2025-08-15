use crate::lodging::{ Hotel, Accommodation, Description };

pub fn book_for_one_night(entity: &mut impl Accommodation, guest: &str) {
    entity.book(guest, 1);
}

/* ============================================================================================== */

fn book_for_one_night_2<T: Accommodation>(entity: &mut T, guest: &str) {
    entity.book(guest, 1);
}

// tu możemy przekazać dwie różne implementacje Accommodation
pub fn mix_and_match(first: &mut impl Accommodation, second: &mut impl Accommodation, guest: &str) {
    first.book(guest, 1);
    second.book(guest, 2);
}

// tutaj muszą być to dwie takie same implementacje Accommodation
fn mix_and_match_2<T: Accommodation>(first: &mut T, second: &mut T, guest: &str) {
    first.book(guest, 1);
    second.book(guest, 2);
}

// a tu wspieramy dwa rożne typy Accommodation
fn mix_and_match_3<T: Accommodation, U: Accommodation>(first: &mut T, second: &mut U, guest: &str) {
    first.book(guest, 1);
    second.book(guest, 2);
}

/* ============================================================================================== */

fn mix_and_match_4(
    first: &mut (impl Accommodation + Description), // musi również wspierać Description
    second: &mut impl Accommodation,
    guest: &str
) {
    first.book(guest, 1);
    second.book(guest, 2);
}

// przykład z generics
fn book_for_one_night_3<T: Accommodation + Description>(entity: &mut T, guest: &str) {
    entity.book(guest, 1);
}

/* ============================================================================================== */

/*
 za where możemy zdefiniować generyki zamiast w < >
*/
fn mix_and_match_5<T, U>(
    first: &mut T, // musi również wspierać Description
    second: &mut U,
    guest: &str
) where
    T: Accommodation + Description,
    U: Accommodation
{
    first.book(guest, 1);
    second.book(guest, 2);
}

/* ============================================================================================== */

pub fn choose_best_place_to_stay() -> impl Accommodation + Description {
    Hotel::new("The Lux")

    /* nie możemy zwracać różnych typów implementacji na podstawie logiki
    if true {
        Hotel::new("The Lux")
    } else {
        AirBnB::new("AirBnB")
    }*/
}