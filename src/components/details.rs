use leptos::{
    html::{Details, Div},
    *,
};
use web_sys::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DetailsState {
    Open,
    IsOpening,
    Closed,
    IsClosing,
}

impl DetailsState {
    fn toggle(self) -> Self {
        match self {
            Self::Open | Self::IsOpening => Self::IsClosing,
            Self::Closed | Self::IsClosing => Self::IsOpening,
        }
    }
}

#[component]
pub fn Details(summary: String, children: Children) -> impl IntoView {
    let (state, set_state) = create_signal::<DetailsState>(DetailsState::Closed);
    let details_ref = create_node_ref::<Details>();
    let content_ref = create_node_ref::<Div>();

    let on_click = move |ev: MouseEvent| {
        ev.prevent_default();
        let details = details_ref.get().expect("details ref not found");

        details.scroll_height();

        // set overflow hidden
        // if (state.get() == DetailsState::IsClosing || !details.open() {
        //     set_state.update(|state| *state = state.toggle())

        set_state.update(|state| *state = state.toggle());

        details.set_open(!details.open());
    };

    create_effect(move |_| logging::log!("{:?}", state.get()));

    view! {
        <details _ref=details_ref
            class="text-white group transition overflow-hidden"
            open=move || state.with(|state| *state == DetailsState::Open)
            // style=("overflow", move || state.with(|state| state.overflow_value()))
            on:click=on_click>
            <summary class="bg-neutral-700 p-4 rounded group-open:rounded-b-none transition">{summary}</summary>
            <div class="bg-neutral-800 p-4 rounded-b transition duration-700 overflow-hidden"
                _ref=content_ref
                style=("max-height", move || state.with(|state| {
                    if *state == DetailsState::Open || *state == DetailsState::IsOpening {
                        format!("{}px", content_ref.get().unwrap().scroll_height())
                    } else {
                        "0".to_string()
                    }
                }))
        >{children()}</div>
        </details>
    }
}
