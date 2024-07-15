use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TrxItem {
    pub id: String,
    pub date: String,
    pub amount: u64,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Trx {
    balance: u64,
    pub items: Vec<TrxItem>,
}