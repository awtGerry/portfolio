mod greet;

fn main() {
    sycamore::render(view! cx {
        greet::greet(cx);
    })
}
