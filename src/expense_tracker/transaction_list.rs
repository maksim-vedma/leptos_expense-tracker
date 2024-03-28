use crate::expense_tracker::expense_tracker::Transaction;
use crate::expense_tracker::transaction_item::*;
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

