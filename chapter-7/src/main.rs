use chapter_7::eat_at_restaurant;

use crate::garden::vegetables::Asparagus; // cannot find it unless we add the module and it is visible

// inline module
// pub mod garden {
//     pub mod vegetables {
//         #[derive(Debug)]
//         pub struct Asparagus;
//     }
// }

mod garden; // includes all the public modules inside garden.
// this will include: [ garden( vegetables(Asparagus) ) ]
fn main() {
    let plant = Asparagus;
    println!("I'm growing {plant:?}!");

    eat_at_restaurant();
}