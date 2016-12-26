// field initialization shorthand formatting
// RFC: https://github.com/rust-lang/rfcs/pull/1682

#![feature(field_init_shorthand)]

struct AA{}
struct BB{}

struct CC {
    a: AA,
    b: BB
}

impl CC {
    fn new(a: AA, b: BB) -> Self {
        Self { a, b }
    }
}
