#![allow(dead_code)]

fn scope(f: impl FnOnce()) {
    f();
}

fn foo(iter: impl Iterator<Item = u32>) {
    // A and B.
    scope(move || for _i in iter {});
}

fn main() {
    println!("hello world!");
}
