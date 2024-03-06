use crate::components::transaction_item::*;
use crate::Transaction;
use leptos::*;

#[component]
pub fn TransactionList(#[prop(into)] transactions: Signal<Vec<Transaction>>) -> impl IntoView {
    view! {
        <p class="text-xl">History</p>
        // <Divider/>
        <ul class="grid gap-1 max-h-60 overflow-auto">
            <For each=move || transactions.get() key=|state| state.id let:transaction>
                <TransactionItem transaction/>
            </For>
        </ul>
    }
}

