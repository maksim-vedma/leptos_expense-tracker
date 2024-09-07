use leptos::*;
use web_sys::MouseEvent;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[component]
pub fn SimpleButton(children: Children) -> impl IntoView {
    let (ripple_position, set_ripple_position) = create_signal::<Position>(Position { x: 0, y: 0 });
    let (show_ripple, set_show_ripple) = create_signal::<bool>(false);

    let on_click = move |ev: MouseEvent| {
        set_show_ripple.set(false);
        set_ripple_position.update(|position| {
            position.x = ev.offset_x();
            position.y = ev.offset_y();
        });
        set_show_ripple.set(true);
    };

    view! {
        <button
            class="relative inline-grid bg-neutral-700 text-neutral-100 rounded items-center px-4 py-2 text-balance
            transition disabled:opacity-50 isolate active:scale-[.99] focus-visible:ring-2 focus-visible:ring-teal-400 shadow overflow-hidden"
            on:click=on_click
        >
            <Show when=move || show_ripple.get()>
                <span
                    class="absolute top-[--ripple-y] left-[--ripple-x] h-12 aspect-square -translate-x-1/2 -translate-y-1/2
                    rounded-full bg-white pointer-events-none z-0 animate-ripple opacity-50"
                    style=(
                        "--ripple-x",
                        move || ripple_position.with(|position| format!("{}px", position.x)),
                    )

                    style=(
                        "--ripple-y",
                        move || ripple_position.with(|position| format!("{}px", position.y)),
                    )

                    on:animationend=move |_| set_show_ripple.set(false)
                ></span>
            </Show>
            {children()}
        </button>
    }
}
