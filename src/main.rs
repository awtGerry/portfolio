use sycamore::prelude::*;

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn say_hi() {
    let a = 1;
    let b = 2;
    sycamore::render(|cx| view! { cx,
        p { "The sum of " (a) " and " (b) " is " (sum(a, b)) "." }
    });
}

fn main() {
    say_hi();
}
