use leptos::*;
use leptos_meta::{provide_meta_context, Meta, Stylesheet};
use leptos_router::*;

mod components;
use components::*;
mod pages;
use pages::*;

mod expense_tracker;
use expense_tracker::expense_tracker::ExpenseTracker;

fn main() {
    leptos::mount_to_body(|| {
        view! { <App/> }
    })
}

#[component]
fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Router>
            <div class="grid md:grid-cols-[auto_1fr] content-start md:content-stretch md:h-full">
                <nav::Nav></nav::Nav>
                <main class="overflow-auto">
                    <Routes>
                        <Route path="/" view=Playground/>
                        <Route path="/expense_tracker" view=ExpenseTracker/>
                        <Route path="/articles" view=|| view! { <Outlet/> }>
                            <Route path="" view=Articles/>
                            <Route path=":id" view=ArticleSingle/>
                        </Route>
                        <Route path="/*any" view=|| view! { <h1>"404 Not Found"</h1> }/>
                    </Routes>
                </main>
            </div>
        </Router>
    }
}
