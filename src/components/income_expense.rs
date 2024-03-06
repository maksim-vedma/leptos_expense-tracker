use leptos::*;

#[component]
pub fn IncomeExpense(
    #[prop(into)] income: Signal<f32>,
    #[prop(into)] expense: Signal<f32>,
) -> impl IntoView {
    view! {
        <section class="grid grid-cols-2 gap-2">
            <div class="shadow p-4">
                <p>Income</p>
                <em class="text-teal-600">"+$" {income}</em>
            </div>

            <div class="shadow p-4">
                <p>Expense</p>
                <em class="text-red-400">"-$" {expense}</em>
            </div>
        </section>
    }
}

