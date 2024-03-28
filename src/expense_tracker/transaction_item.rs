use crate::expense_tracker::expense_tracker::{Transaction, TransactionsContext};
use leptos::*;
use uuid::Uuid;

#[component]
pub fn TransactionItem(#[prop()] transaction: Transaction) -> impl IntoView {
    let transactions_ctx = use_context::<TransactionsContext>()
        .expect("Should have been able to find TransactionContext");

    let remove_transaction = move |transaction_id: Uuid| {
        transactions_ctx
            .0
            .update(move |transactions| transactions.retain(|t| t.id != transaction_id));
    };

    view! {
        <li
        class="group flex gap-2 p-4 border-r-2 shadow"
        class=("border-green-600", transaction.is_income())
        class=("border-red-600", transaction.is_expense())
        >
            <button class="h-8 w-8 text-red-500 bg-neutral-800 opacity-0 -translate-x-1 transition-all group-hover:opacity-100 group-hover:translate-x-0"
                on:click=move|_|remove_transaction(transaction.id)
        >
                X
            </button>
            <p class="flex-1">{transaction.text}</p>
            <em class="font-bold text-lg">{transaction.amount}</em>
        </li>
    }
}

