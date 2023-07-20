use sycamore::prelude::*;
use sycamore_futures::spawn_local_scoped;

#[component]
pub fn greet<G: Html>(cx: Scope<'_>) -> View<G> {
    let name = create_signal(cx, String::new());
    let greet_msg = create_signal(cx, String::new());

    let change_msg = |_| {
        spawn_local_scoped(cx, async move {
            let new_msg = format!("Hello {}", name.get());
            greet_msg.set(new_msg)
        });
    };

    view! { cx,
        main(class="foo") {
            form(class="bar",on:submit=change_msg) {
                input(id="greet-input",bind:value=name,placeholder="Username...")
                button(
                    type="submit",
                    style="
                        background-color: #64CCC5;
                        color: #222831;
                    ",
                ) { "Click me" }
            }
            p {
                b {
                    (greet_msg.get())
                }
            }
        }
    }

}
