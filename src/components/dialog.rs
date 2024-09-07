use leptos::{html::Dialog, *};
use web_sys::MouseEvent;

use crate::button::Button;

#[component]
pub fn Dialog(
    show: ReadSignal<bool>,
    set_show: WriteSignal<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let diag_ref = create_node_ref::<Dialog>();

    let close = move |_: MouseEvent| {
        set_show.set(false);
    };

    create_effect(move |_| {
        if let Some(diag_ref) = diag_ref.get() {
            if show.get() {
                let _ = diag_ref.show_modal();
            } else {
                diag_ref.close();
            }
        }
    });

    view! {
        <Portal>
            <dialog
                _ref=diag_ref
                class="absolute inset-0 bg-neutral-600 text-white backdrop:backdrop-blur-sm transition duration-1000 backdrop:transition"
            >
                <header>
                    <p class="text-2xl">"Header de la modal"</p>
                    <Button label="Fermer" on:click=close/>
                </header>
                <div class="bg-slate-400 p-4">
                    {children()}
                </div>
                <footer class="flex justify-end items-center bg-slate-800 text-white p-4">
                 <p>"Footer"</p>
                </footer>
            </dialog>
        </Portal>
    }
}
