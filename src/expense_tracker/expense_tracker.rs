use crate::expense_tracker::add_transaction::*;
use crate::expense_tracker::balance::*;
use crate::expense_tracker::income_expense::*;
use crate::expense_tracker::transaction_list::*;
use leptos::*;
use uuid::Uuid;

// TODO add impl and try it as a RwSignal
#[derive(Debug, Clone)]
pub struct Transactions(pub Vec<Transaction>);

impl Transactions {
    pub fn balance(&self) -> f32 {
        self.0.iter().fold(0.00, |mut sum, transaction| {
            sum += transaction.amount;
            sum
        })
    }
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub id: Uuid,
    pub text: String,
    pub amount: f32,
}

impl Transaction {
    pub fn new(text: String, amount: f32) -> Self {
        Self {
            id: Uuid::new_v4(),
            text,
            amount,
        }
    }

    pub fn is_income(&self) -> bool {
        self.amount > 0.0
    }

    pub fn is_expense(&self) -> bool {
        self.amount < 0.0
    }
}

#[derive(Copy, Clone)]
pub struct TransactionsContext(pub WriteSignal<Vec<Transaction>>);

#[component]
pub fn ExpenseTracker() -> impl IntoView {
    let (transactions, set_transactions) = create_signal(vec![
        Transaction::new("Test1".to_string(), 15.00),
        Transaction::new("Test2".to_string(), 15.00),
        Transaction::new("Test3".to_string(), 15.00),
    ]);

    let balance = move || {
        transactions
            .get()
            .iter()
            .fold(0.00, |mut sum, transaction| {
                sum += transaction.amount;
                sum
            })
    };

    let expense = move || {
        transactions
            .get()
            .iter()
            .filter(|t| t.amount < 0.0)
            .fold(0.00, |mut sum, transaction| {
                sum += transaction.amount;
                sum
            })
    };

    let income = move || {
        transactions
            .get()
            .iter()
            .filter(|t| t.amount >= 0.0)
            .fold(0.00, |mut sum, transaction| {
                sum += transaction.amount;
                sum
            })
    };

    // Probably less optimized because of the 2 calls
    // let balance = move || expense() + income();

    let add_transaction = move |text: String, amount: f32| {
        set_transactions.update(|t| {
            t.push(Transaction::new(text, amount));
        });
    };

    provide_context::<TransactionsContext>(TransactionsContext(set_transactions));

    view! {
        <div class="mx-auto px-4 max-w-sm">
            <Balance amount=balance/>
            <IncomeExpense income expense/>
            <AddTransaction add_transaction/>
            <TransactionList transactions/>
        </div>
    }
}

