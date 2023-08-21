use sycamore::prelude::*;

#[component]
fn App<G: Html>(range: &str) -> View<G> {}
    let range = create_signal(range.to_string());
    let range = range.get();
    let range = range.parse::<i32>().unwrap();
    let mut sum = 0;
    for i in 0..range {
        sum += i;
    }
    view! { cx,
        p { "The sum to " (range) " is " (sum) "." }
    </> }
}
