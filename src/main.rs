use sycamore::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[component]
fn RegistrationForm<G: Html>(context: Scope<'_>) -> View<G> {
    let email_address: &Signal<String> = create_signal(context, String::new());
    let password: &Signal<String> = create_signal(context, String::new());

    let submit_registration = |_| spawn_local(context, async move {
        if email_address.get().is_empty() || password.get().is_empty() {
            return;
        }
        let response = register_user(
            email_address.get().as_ref().clone(),
            password.get().as_ref().clone(),
        )
        .await
        .unwrap_or_default();
    });

    view! { context,
            form {
                div {
                    label {"Email Address"}
                    input(placeholder="example@email.com", type="text", bind=email_address.clone())
                }
                div {
                    label {"Password"}
                    input(placeholder="", type="password", bind=password.clone())
                }
                div {
                    button(type="submit", on:submit=submit_registration)
                }
        }
    }
}

fn main() {
    sycamore::render(|cx| view! { cx,
        div() {
            RegistrationForm(cx)
            // greet(cx)
        }
    });
}

