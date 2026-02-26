
#[derive(Debug)]
pub enum EntryType {
    Credit,
    Debit,
}
//struct
pub struct LedgerEntry{
    pub id:u32,
    pub amount: f64,
    pub description: String,
    pub date: String,
    pub entry_type: EntryType,
}

//ledger struct
pub struct Ledger{
    entries: Vec<LedgerEntry>,
    next_id:u32,
}

impl Ledger{
    pub fn new()-> Self{
        Ledger{
            entries: Vec::new(),
            next_id:1,
        }
    }
    pub fn add_entry(
        &mut self,
        amount: f64,
        description: String,
        date: String,
        entry_type: EntryType,
    ) {
        let entry = LedgerEntry {
            id: self.next_id,
            amount,
            description,
            date,
            entry_type,
        };
        self.entries.push(entry);
        self.next_id += 1; // ✅ No bold formatting around +=
    }
    pub fn list_entries(&self) {
        for entry in &self.entries {
            println!(
            "ID: {} | Amount: {} | Desc: {} | Date: {} | Type: {:?}",
            entry.id,
            entry.amount,
            entry.description,
            entry.date,
            entry.entry_type
            );
        }
    }
}
