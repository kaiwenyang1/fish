#![allow(dead_code)]

mod aliases;
mod magic;
mod masks;
mod positions;
mod tables;
mod utils;

fn main() {
    let ms = masks::make_mask_lookup();
    tables::make_table_set(&ms);

    let p = positions::init_chess();
    p.print();
}
