use invoice::Invoice;
use invoice::exporter::InvoiceExporter;

pub mod invoice;
pub mod transaction;

fn main() {

    let invoice = Invoice::new();
    let invoice_exporter = InvoiceExporter::new(invoice);
    invoice_exporter.export();

    println!("Hello, invoice finish created!");
}
