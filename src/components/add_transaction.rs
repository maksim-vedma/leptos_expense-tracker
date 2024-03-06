use crate::components::button::{Button, ButtonColor, ButtonSize, ButtonVariant};
use leptos::{leptos_dom::logging::console_log, *};
use web_sys::SubmitEvent;

#[component]
pub fn AddTransaction<F>(add_transaction: F) -> impl IntoView
where
    F: Fn(String, f32) + 'static,
{
    let (text, set_text) = create_signal("".to_string());
    let (amount, set_amount) = create_signal(Ok(1.00));

    let input_element: NodeRef<html::Input> = create_node_ref();

    let reset_form = move || {
        set_text.set("".to_string());
        set_amount.set(Ok(1.0));
        let _ = input_element.get().unwrap().focus();
    };

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        add_transaction(text.get(), amount.get().unwrap());
        reset_form();
    };

    let cannot_submit =
        move || text.get().is_empty() || amount.get().is_err() || amount.get() == Ok(0.0);

    view! {
        <form action="" class="grid grid-cols-2 gap-2" on:submit=on_submit>
            <div class="group w-full">
                <input
                    type="text"
                    node_ref=input_element
                    id="transaction_label"
                    class="w-full px-4 py-2"
                    placeholder="Nom de la transaction"
                    prop:value=text
                    value=text
                    on:input=move |ev| { set_text.set(event_target_value(&ev).to_string()) }
                />
            </div>

            <div class="group w-full">
                <input
                    type="number"
                    id="transaction_amount"
                    class="w-full px-4 py-2"
                    placeholder="Montant"
                    prop:value=move|| amount.get().unwrap()
                    value=move|| amount.get().unwrap()
                    on:input=move |ev| { set_amount.set(event_target_value(&ev).parse::<f32>()) }
                />
            </div>
            <div class="col-span-2">
                <Button label="Ajouter" disabled=cannot_submit full=true color=ButtonColor::Accent/>
            </div>
        </form>
    }
}

