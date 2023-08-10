use sycamore::prelude::*;

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn do_sum() {
    let a: i32 = 0;
    let b: i32 = 0;
    sycamore::render(|cx| view! { cx,
        input(placeholder="Value of a") {
            "a = " (a)
        }
        input(placeholder="Value of b") {
            "b = " (b)
        }
        p { "The sum of " (a) " and " (b) " is " (sum(a, b)) "." }
    });
}

fn say_hi() {
    sycamore::render(|cx| view! { cx,
        p { "hello, world!" }
    });
}

fn main() {
    do_sum();
}
