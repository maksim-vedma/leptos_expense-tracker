use leptos::{leptos_dom::logging::*, *};
use leptos_router::*;

#[derive(Debug, Clone)]
struct NavItemData {
    route: String,
    label: String,
}

impl NavItemData {
    fn new(route: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            route: route.into(),
            label: label.into(),
        }
    }
}

#[component]
pub fn Nav() -> impl IntoView {
    let links: Vec<NavItemData> = vec![
        NavItemData::new("", "Home"),
        NavItemData::new("expense_tracker", "Expense Tracker"),
        NavItemData::new("articles", "Articles"),
        NavItemData::new("dialog", "Dialog"),
    ];

    view! {
        <nav class="relative px-2 shadow bg-neutral-800 isolate
        text-neutral-200 whitespace-nowrap
        transform md:-translate-x-[90%] transition hover:md:translate-x-0
        ">
            <ul class="grid">
                {links.into_iter().map(move |link| view! { <NavItem link/> }).collect_view()}

            </ul>
        </nav>
    }
}

const NAV_ITEM_STYLE: &str =
    "block p-2 rounded-sm hover:text-teal-700 hover:bg-neutral-700 transition active-page:bg-neutral-900 active-page:text-teal-700";

#[component]
fn NavItem(#[prop()] link: NavItemData) -> impl IntoView {
    view! {
        <li>
            <A href=link.route class=NAV_ITEM_STYLE>
                {link.label}
            </A>
        </li>
    }
}
