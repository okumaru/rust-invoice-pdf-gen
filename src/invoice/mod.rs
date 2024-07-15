use std::fs;
use std::fs::File;
use serde::{Serialize, Deserialize};
use crate::transaction::Trx;

pub mod exporter;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct InvUser {
    name: String,
    address: Option<String>,
    phone: Option<String>,
    email: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct InvItem {
    description: String,
    quantity: u8,
    price: i64,
    amount: i64
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Invoice {
    number: String,
    status: String,
    issuedate: String,
    duedate: String,
    paiddate: Option<String>,
    subtotal: u64,
    tax: u64,
    total: u64,
    items: Vec<InvItem>,
    pub transactions: Trx,
    invto: InvUser,
    invfrom: InvUser,
    notes: Option<Vec<String>>
}

impl Invoice {
    pub fn new() -> Invoice {
        let dbfile = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("invoice.json")
            .expect("Failed to get invoice.json");

        let invoice = serde_json::from_reader::<File, Invoice>(dbfile)
            .expect("Failed to read file json");

        invoice
    }
}