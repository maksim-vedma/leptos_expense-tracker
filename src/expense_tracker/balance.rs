use leptos::*;

#[component]
pub fn Balance(#[prop(into)] amount: Signal<f32>) -> impl IntoView {
    view! {
        <p>Balance</p>
        <strong class="block font-bold text-2xl">"$" {amount}</strong>
    }
}

