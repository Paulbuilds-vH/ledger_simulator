mod ledger;
use ledger::{Ledger, EntryType};

fn main(){
    let mut ledger=Ledger::new();

    ledger.add_entry(
    100.0,
    "Initial deposit".to_string(),
    "2026-02-26".to_string(),
    EntryType::Credit,
);

ledger.add_entry(
    40.0,
    "Groceries".to_string(),
    "2026-02-26".to_string(),
    EntryType::Debit,
);
    ledger.list_entries();
}