use sycamore::prelude::*;

fn greet(name: String) {
    sycamore::render(|cx| view! { cx,
        div(class="bar") {
            p { "Hello " (name) }
        }
    });
}

fn main() {
    sycamore::render(|cx| view! { cx,
        div(class="foo") {
            form() {
                input(placeholder="username") {}
                input(type="password") {}
                button(
                    type="button",
                    style="
                        background-color: #64CCC5;
                        color: #222831;
                    ",
                    on:click=|_| { greet("admin".to_string()); }
                ) { "Click me" }
            }
        }
    });
}
