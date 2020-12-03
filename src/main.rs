//#[macro_use]
//extern crate lazy_static;

//pub mod day1;
//pub mod day2;

extern crate bump_alloc;

use bump_alloc::BumpAlloc;

#[global_allocator]
static A : BumpAlloc = BumpAlloc::with_size(500000000);

pub mod day3;

fn main() {
    day3::solve();
}
