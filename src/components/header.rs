use leptos::*;

const COUCOU_STYLE: &str = "bg-teal-900 md:grid-cols-2";

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="teal-900 text-white mb-2">
            <h1 class="text-2xl">"Expanse Tracker"</h1>
        </header>
    }
}

