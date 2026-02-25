#[allow(dead_code)]
#[derive(Debug)]
pub enum EntryType {
    Credit,
    Debit,
}

pub struct LedgerEntry{
    pub id:u32,
    pub amount: f64,
    pub description: String,
    pub date: String,
    pub entry_type: EntryType,
}
