mod vectors;
mod utf8;
mod hashmaps;

use crate::hashmaps::part3;
use crate::vectors::part1;
use crate::utf8::part2;

fn main() {
    println!("#### PART 1 : VECTORS ####");
    part1();

    println!("#### PART 2 : UTF8 String ####");
    part2();

    println!("#### PART 3 : HASHMAPS ####");
    part3();
}