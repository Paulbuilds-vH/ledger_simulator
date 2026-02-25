mod ledger;
use ledger::{LedgerEntry, EntryType};


fn main() {
    let entry = LedgerEntry {
        id: 1,
        amount: 100.0,
        description: "Deposit".to_string(),
        date: "2026-02-25".to_string(),
        entry_type: EntryType::Credit,
    };

    println!(
    "Ledger Entry: {} {} {} {} {:?}",
    entry.id,
    entry.amount,
    entry.description,
    entry.date,
    entry.entry_type
);
}