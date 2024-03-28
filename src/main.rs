use leptos::*;
use leptos_meta::Stylesheet;
use leptos_router::*;

pub mod components;
use components::header::Header;
mod expense_tracker;
use expense_tracker::expense_tracker::ExpenseTracker;

fn main() {
    leptos::mount_to_body(|| {
        view! { <App/> }
    })
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <Router>
            <div class="grid grid-rows-[auto_1fr] min-h-full">
                <Header />
                <main class="place-self-center">
                    <Routes>
                        <Route path="" view=move || view!{<div>Je suis la HOME</div>}></Route>
                        <Route path="expense_tracker" view=ExpenseTracker></Route>
                        <Route path="/*any" view=|| view! { <h1>"404 Not Found"</h1> }/>
                    </Routes>
                </main>
            </div>
        </Router>
    }
}
