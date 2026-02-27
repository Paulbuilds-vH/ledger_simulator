use chrono::NaiveDate;

mod ledger;
use ledger::{EntryType, Ledger};

fn main() {
    let mut ledger = Ledger::new();

    let date = NaiveDate::from_ymd_opt(2026, 2, 26).unwrap();

    match ledger.add_entry(10000, "Initial deposit".to_string(), date, EntryType::Credit) {
        Ok(()) => println!("Entry added successfully"),
        Err(e) => println!("Failed: {:?}", e),
    }

    match ledger.add_entry(4000, "Groceries".to_string(), date, EntryType::Debit) {
        Ok(()) => println!("Entry added successfully"),
        Err(e) => println!("Failed: {:?}", e),
    }

    ledger.list_entries();
}