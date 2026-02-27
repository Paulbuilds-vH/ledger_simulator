use chrono::NaiveDate;

#[derive(Debug,Clone,PartialEq)]
pub enum EntryType {
    Credit, //money coming in
    Debit,  //money coming out
}
#[derive(Debug, Clone)]
pub struct LedgerEntry{
    pub id:u32,
    pub amount_cents: u64,
    pub description: String,
    pub date: NaiveDate,
    pub entry_type: EntryType,
}

#[derive(Debug)]
pub enum LedgerError{
    InvalidAmount,
    //InvalidDate,
}


pub struct Ledger{
    entries: Vec<LedgerEntry>,
    //starts at 1, auto increments with each new entry
    next_id:u32,
}

impl Ledger {
    pub fn new() -> Self {
        Ledger {
            entries: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add_entry(
        &mut self,
        amount_cents: u64,
        description: String,
        date: NaiveDate,
        entry_type: EntryType,
    ) -> Result<(), LedgerError> {
        // reject zero amount entries, they have no meaning in a ledger
        if amount_cents == 0 {
            return Err(LedgerError::InvalidAmount);
        }

        self.entries.push(LedgerEntry {
            id: self.next_id,
            amount_cents,
            description,
            date,
            entry_type,
        });

        self.next_id += 1;
        Ok(())
    }

    pub fn list_entries(&self) {
        for entry in &self.entries {
            println!(
                "ID: {} | Amount: ${:.2} | Desc: {} | Date: {} | Type: {:?}",
                entry.id,
                entry.amount_cents as f64 / 100.0,  // convert for display
                entry.description,
                entry.date,
                entry.entry_type
            );
        }
    }
}
