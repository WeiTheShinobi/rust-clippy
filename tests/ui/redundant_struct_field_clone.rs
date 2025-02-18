#![warn(clippy::redundant_struct_field_clone)]
#![allow(clippy::useless_format)]

struct S {
    a: String,
    b: String,
}

pub fn main() {
    let x = String::from("hello");

    let s = S {
        a: x.clone(), // :(
        b: x,
    };
}
