use leptos::{
    leptos_dom::logging::{self, console_log},
    *,
};
use leptos_router::*;

// struct RouteData<'a> {
//     route: &'a str,
//     view: Box<dyn IntoView>,
// }
//
// impl<'a> RouteData<'a> {
//     fn new<T>(route: &'a str, view: T) -> Self
//     where
//         T: IntoView + 'static,
//     {
//         RouteData {
//             route,
//             view: Box::new(view),
//         }
//     }
// }

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
pub fn Header() -> impl IntoView {
    let links: Vec<NavItemData> = vec![
        NavItemData::new("", "Home"),
        NavItemData::new("expense_tracker", "Expense Tracker"),
    ];

    view! {
        <header class="shadow bg-neutral-800 text-neutral-200">
            <div class="container">
                <nav>
                    <ul class="flex justify-center items-center">
                        {links.into_iter().map(|l| view! { <NavItem link=l />}).collect_view()}
                    </ul>
                </nav>
            </div>
        </header>
    }
}

const NAV_ITEM_STYLE: &'static str =
    "block py-8 px-4 hover:text-teal-700 hover:bg-neutral-900 transition duration-500";
const NAV_ITEM_ACTIVE_STYLE: &'static str = "text-teal-700 bg-neutral-900";

#[component]
fn NavItem(#[prop()] link: NavItemData) -> impl IntoView {
    let route = use_route();
    let check_route = move |_| console_log(&route.path());

    view! {
        <li on:click=check_route>
            <A href=link.route class=NAV_ITEM_STYLE>{link.label}</A>
        </li>
    }
}

