
// use std::io;
use std::fs;
use std::fs::File;
use serde::{Serialize, Deserialize};
use genpdf::{fonts, elements, style};
use genpdf::Alignment;
use genpdf::Element as _;
use genpdf::style::Style;
use genpdf::elements::FrameCellDecorator;
use genpdf::elements::TableLayout;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct InvUser {
    name: String,
    address: Option<String>,
    phone: Option<String>,
    email: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct TrxItem {
    id: String,
    date: String,
    amount: u64,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct Trx {
    balance: u64,
    items: Vec<TrxItem>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct InvItem {
    description: String,
    quantity: u8,
    price: i64,
    amount: i64
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct Invoice {
    number: String,
    status: String,
    issuedate: String,
    duedate: String,
    paiddate: Option<String>,
    subtotal: u64,
    tax: u64,
    total: u64,
    items: Vec<InvItem>,
    transactions: Trx,
    invto: InvUser,
    invfrom: InvUser,
    notes: Option<Vec<String>>
}

impl Invoice {
    fn new() -> Invoice {
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

struct InvoiceExporter {
    invoice: Invoice,
    style_inv_header_title: Style,
    style_inv_number: Style,
    style_status_inv_unpaid: Style,
    style_status_inv_paid: Style,
    style_tbl_header: Style,
    style_text_normal: Style,
    style_text_big: Style,
    style_text_big_total: Style,
    style_section_title: Style,
    style_tbl_frame_decorator: FrameCellDecorator
}

impl InvoiceExporter {
    fn new(invoice: Invoice) -> InvoiceExporter {
        InvoiceExporter {
            invoice: invoice,
            style_inv_header_title: style::Style::new().bold().with_font_size(40),
            style_inv_number: style::Style::new().with_font_size(11),
            style_status_inv_unpaid: style::Style::new().with_color(style::Color::Rgb(190, 48, 48)).bold().with_font_size(20),
            style_status_inv_paid: style::Style::new().with_color(style::Color::Rgb(62, 142, 126)).bold().with_font_size(20),
            style_tbl_header: style::Style::new().with_color(style::Color::Rgb(146, 154, 171)).bold().with_font_size(11),
            style_text_normal: style::Style::new().with_font_size(9),
            style_text_big: style::Style::new().bold().with_font_size(11),
            style_text_big_total: style::Style::new().bold().with_font_size(11).with_color(style::Color::Rgb(82, 97, 107)),
            style_section_title: style::Style::new().bold().with_font_size(20),
            style_tbl_frame_decorator: elements::FrameCellDecorator::new(false, false, false)
        }
    }

    fn export(&self) {

        // Load a font from the file system
        let font_family = fonts::from_files("./fonts", "LiberationSans", None)
        .expect("Failed to load font family");

        // Create a document and set the default font family
        let mut doc = genpdf::Document::new(font_family.clone());

        // Change the default settings
        doc.set_title(
            format!(
                "Invoice - {}", 
                self.invoice.number
            )
        );

        // Customize the pages
        let mut decorator = genpdf::SimplePageDecorator::new();
        decorator.set_margins(10);
        doc.set_page_decorator(decorator);

        // Invoice Header
        let mut inv_number = elements::LinearLayout::vertical();
        inv_number.push(
            elements::Paragraph::new("INVOICE")
                .styled(self.style_inv_header_title)
        );
        inv_number.push(
            elements::Paragraph::new(
                format!(
                    "No. {}", 
                    self.invoice.number
                )
            ).styled(self.style_inv_number)
        );

        let mut inv_header = elements::TableLayout::new(vec![1, 1]);
        inv_header.set_cell_decorator(self.style_tbl_frame_decorator.clone());

        let mut style_inv_status: Style = self.style_status_inv_unpaid;
        if self.invoice.status.clone() == "PAID" {
            style_inv_status = self.style_status_inv_paid;
        }

        let mut row_bill = inv_header.row();
        row_bill.push_element(inv_number);
        row_bill.push_element(
            elements::Paragraph::new(self.invoice.status.clone())
                .aligned(Alignment::Right)
                .styled(style_inv_status)
        );
        row_bill.push().expect("Invalid table row");
        doc.push(inv_header);
        // Finish Invoice Header

        doc.push(elements::Break::new(1.5));

        // Invoice Date
        let mut date_layout = elements::LinearLayout::vertical();

        // Issue date
        date_layout.push(
            elements::Paragraph::new(
                format!(
                    "Issue Date: {}", 
                    self.invoice.issuedate
                )
            ).styled(self.style_text_normal)
        );

        // Due date
        date_layout.push(
            elements::Paragraph::new(
                format!(
                    "Due Date: {}", 
                    self.invoice.duedate
                )
            ).styled(self.style_text_normal)
        );

        // Paid date
        date_layout.push(
            elements::Paragraph::new(
                format!(
                    "Paid Date: {}", 
                    self.invoice.paiddate
                        .clone()
                        .unwrap_or("-".to_string())
                )
            ).styled(self.style_text_normal)
        );
        doc.push(date_layout);
        // Finish Invoice Date

        // Line break
        doc.push(elements::Break::new(1.5));

        // Add table invoice from & invoice to
        let to_title = elements::Paragraph::new("Bill To:")
            .padded(genpdf::Margins::trbl(1, 0, 1, 0))
            .styled(self.style_text_big);

        let to_name = elements::Paragraph::new(self.invoice.invto.name.clone())
            .styled(self.style_text_normal);

        let to_addrs = elements::Paragraph::new(
                self.invoice.invto.address
                    .clone()
                    .unwrap_or("-".to_string())
            ).padded(genpdf::Margins::trbl(1, 5, 2, 0))
            .styled(self.style_text_normal);

        let to_phone = elements::Paragraph::new(
                self.invoice.invto.phone
                    .clone()
                    .unwrap_or("-".to_string())
            ).padded(genpdf::Margins::trbl(1, 0, 1, 0))
            .styled(self.style_text_normal);

        let to_email = elements::Paragraph::new(
                self.invoice.invto.email
                    .clone()
                    .unwrap_or("-".to_string())
            ).styled(self.style_text_normal);

        let mut cell_bill_to = elements::LinearLayout::vertical();
        cell_bill_to.push(to_title);
        cell_bill_to.push(to_name);
        cell_bill_to.push(to_addrs);
        cell_bill_to.push(to_phone);
        cell_bill_to.push(to_email);

        let from_title = elements::Paragraph::new("Bill From:")
            .padded(genpdf::Margins::trbl(1, 0, 1, 0))
            .styled(self.style_text_big);

        let from_name = elements::Paragraph::new(self.invoice.invfrom.name.clone())
            .styled(self.style_text_normal);

        let from_addrs = elements::Paragraph::new(
                self.invoice.invfrom.address
                    .clone()
                    .unwrap_or("-".to_string())
            ).padded(genpdf::Margins::trbl(1, 5, 2, 0))
            .styled(self.style_text_normal);

        let from_phone = elements::Paragraph::new(
                self.invoice.invfrom.phone
                    .clone()
                    .unwrap_or("-".to_string())
            ).padded(genpdf::Margins::trbl(1, 0, 1, 0))
            .styled(self.style_text_normal);

        let from_email = elements::Paragraph::new(
                self.invoice.invfrom.email
                    .clone()
                    .unwrap_or("-".to_string())
            ).styled(self.style_text_normal);

        let mut cell_bill_from = elements::LinearLayout::vertical();
        cell_bill_from.push(from_title);
        cell_bill_from.push(from_name);
        cell_bill_from.push(from_addrs);
        cell_bill_from.push(from_phone);
        cell_bill_from.push(from_email);

        let mut table_bill = elements::TableLayout::new(vec![1, 1]);
        table_bill.set_cell_decorator(
            elements::FrameCellDecorator::new(false, false, false)
        );
        let mut row_bill = table_bill.row();
        row_bill.push_element(cell_bill_to);
        row_bill.push_element(cell_bill_from);
        row_bill.push().expect("Invalid table row");

        doc.push(table_bill);
        // Finish add table invoice from & invoice to

        doc.push(elements::Break::new(3.0));

        // Add table invoice item
        doc.push(
            elements::Paragraph::new("Items")
                .padded(genpdf::Margins::trbl(2, 0, 2, 0))
                .styled(self.style_section_title)
        );

        let mut table_inv = elements::TableLayout::new(vec![3, 1, 1, 1]);
        table_inv.set_cell_decorator(
            elements::FrameCellDecorator::new(false, false, false)
        );
        let mut inv_item_header = table_inv.row();
        inv_item_header.push_element(
            elements::Paragraph::new("Description")
                .padded(genpdf::Margins::trbl(2, 0, 2, 0))
                .styled(self.style_tbl_header)
        );
        inv_item_header.push_element(
            elements::Paragraph::new("Quantity")
                .aligned(Alignment::Center)
                .padded(genpdf::Margins::trbl(2, 0, 2, 0))
                .styled(self.style_tbl_header)
        );
        inv_item_header.push_element(
            elements::Paragraph::new("Price")
                .aligned(Alignment::Right)
                .padded(genpdf::Margins::trbl(2, 0, 2, 0))
                .styled(self.style_tbl_header)
        );
        inv_item_header.push_element(
            elements::Paragraph::new("Amount")
                .aligned(Alignment::Right)
                .padded(genpdf::Margins::trbl(2, 0, 2, 0))
                .styled(self.style_tbl_header)
        );
        inv_item_header.push().expect("Invalid table row");

        for item in self.invoice.items.iter() {

            let mut inv_item = table_inv.row();
            inv_item.push_element(
                elements::Paragraph::new(format!("- {}", item.description))
                    .padded(genpdf::Margins::trbl(2, 5, 2, 0))
                    .styled(self.style_text_normal)
            );
            inv_item.push_element(
                elements::Paragraph::new(item.quantity.to_string())
                    .aligned(Alignment::Center)
                    .padded(genpdf::Margins::trbl(2, 0, 2, 0))
                    .styled(self.style_text_normal)
            );
            inv_item.push_element(
                elements::Paragraph::new(format!("Rp. {}", item.price))
                    .aligned(Alignment::Right)
                    .padded(genpdf::Margins::trbl(2, 0, 2, 0))
                    .styled(self.style_text_normal)
            );
            inv_item.push_element(
                elements::Paragraph::new(format!("Rp. {}", item.amount))
                    .aligned(Alignment::Right)
                    .padded(genpdf::Margins::trbl(2, 0, 2, 0))
                    .styled(self.style_text_normal)
            );
            inv_item.push().expect("Invalid table row");

        }

        let mut inv_subtotal = table_inv.row();
        inv_subtotal.push_element(
            elements::Paragraph::new("")
                .padded(genpdf::Margins::trbl(2, 5, 2, 0))
                .styled(self.style_text_normal)
        );
        inv_subtotal.push_element(
            elements::Paragraph::new("")
                .aligned(Alignment::Center)
                .padded(genpdf::Margins::trbl(2, 0, 2, 0))
                .styled(self.style_text_normal)
        );
        inv_subtotal.push_element(
            elements::Paragraph::new("Sub-total")
                .aligned(Alignment::Right)
                .padded(genpdf::Margins::trbl(2, 0, 2, 0))
                .styled(self.style_text_normal.bold())
        );
        inv_subtotal.push_element(
            elements::Paragraph::new(format!("Rp. {}", self.invoice.subtotal))
                .aligned(Alignment::Right)
                .padded(genpdf::Margins::trbl(2, 0, 2, 0))
                .styled(self.style_text_normal)
        );
        inv_subtotal.push().expect("Invalid table row");

        let mut inv_tax = table_inv.row();
        inv_tax.push_element(
            elements::Paragraph::new("")
                .padded(genpdf::Margins::trbl(2, 5, 2, 0))
                .styled(self.style_text_normal)
        );
        inv_tax.push_element(
            elements::Paragraph::new("")
                .aligned(Alignment::Center)
                .padded(genpdf::Margins::trbl(2, 0, 2, 0))
                .styled(self.style_text_normal)
        );
        inv_tax.push_element(
            elements::Paragraph::new("Tax")
                .aligned(Alignment::Right)
                .padded(genpdf::Margins::trbl(2, 0, 2, 0))
                .styled(self.style_text_normal.bold())
        );
        inv_tax.push_element(
            elements::Paragraph::new(format!("Rp. {}", self.invoice.tax))
                .aligned(Alignment::Right)
                .padded(genpdf::Margins::trbl(2, 0, 2, 0))
                .styled(self.style_text_normal)
        );
        inv_tax.push().expect("Invalid table row");

        let mut inv_total = table_inv.row();
        inv_total.push_element(
            elements::Paragraph::new("")
                .padded(genpdf::Margins::trbl(2, 5, 2, 0))
        );
        inv_total.push_element(
            elements::Paragraph::new("")
                .padded(genpdf::Margins::trbl(2, 0, 2, 0))
        );
        inv_total.push_element(
            elements::Paragraph::new("Total")
                .aligned(Alignment::Right)
                .padded(genpdf::Margins::trbl(2, 0, 2, 0))
                .styled(self.style_text_big_total)
        );
        inv_total.push_element(
            elements::Paragraph::new(format!("Rp. {}", self.invoice.total))
                .aligned(Alignment::Right)
                .padded(genpdf::Margins::trbl(2, 0, 2, 0))
                .styled(self.style_text_big_total)
        );
        inv_total.push().expect("Invalid table row");

        doc.push(table_inv);
        // Finish add table invoice item

        doc.push(elements::Break::new(3.0));

        // Add table trx item
        doc.push(
            elements::Paragraph::new("Transactions")
                .padded(genpdf::Margins::trbl(2, 0, 2, 0))
                .styled(self.style_section_title)
        );
        let mut table_trx = elements::TableLayout::new(vec![2, 2, 1]);
        table_trx.set_cell_decorator(
            elements::FrameCellDecorator::new(false, false, false)
        );
        let mut trx_item_header = table_trx.row();
        trx_item_header.push_element(
            elements::Paragraph::new("Date Transaction")
                .padded(genpdf::Margins::trbl(2, 0, 2, 0))
                .styled(self.style_tbl_header)
        );
        trx_item_header.push_element(
            elements::Paragraph::new("ID Transaction")
                .padded(genpdf::Margins::trbl(2, 0, 2, 0))
                .styled(self.style_tbl_header)
        );
        trx_item_header.push_element(
            elements::Paragraph::new("Amount")
                .aligned(Alignment::Right)
                .padded(genpdf::Margins::trbl(2, 0, 2, 0))
                .styled(self.style_tbl_header)
        );
        trx_item_header.push().expect("Invalid table row");

        for trx in self.invoice.transactions.items.iter() {

            let mut trx_item = table_trx.row();
            trx_item.push_element(
                elements::Paragraph::new(trx.date.clone())
                .padded(genpdf::Margins::trbl(2, 5, 2, 0))
                .styled(self.style_text_normal)
            );
            trx_item.push_element(
                elements::Paragraph::new(trx.id.to_string())
                    .padded(genpdf::Margins::trbl(2, 0, 2, 0))
                    .styled(self.style_text_normal)
            );
            trx_item.push_element(
                elements::Paragraph::new(format!("Rp. {}", trx.amount))
                    .aligned(Alignment::Right)
                    .padded(genpdf::Margins::trbl(2, 0, 2, 0))
                    .styled(self.style_text_normal)
            );
            trx_item.push().expect("Invalid table row");

        }

        let mut trx_total = table_trx.row();
        trx_total.push_element(
            elements::Paragraph::new("")
                .padded(genpdf::Margins::trbl(2, 5, 2, 0))
        );
        trx_total.push_element(
            elements::Paragraph::new("Balance")
                .aligned(Alignment::Right)
                .padded(genpdf::Margins::trbl(2, 0, 2, 0))
                .styled(self.style_text_big_total)
        );
        trx_total.push_element(
            elements::Paragraph::new("Rp. 0")
                .aligned(Alignment::Right)
                .padded(genpdf::Margins::trbl(2, 0, 2, 0))
                .styled(self.style_text_big_total)
        );
        trx_total.push().expect("Invalid table row");

        doc.push(table_trx);
        // Finish add trx item

        doc.push(elements::Break::new(3.0));

        // Add Note
        let mut note_layout = elements::LinearLayout::vertical();
        note_layout.push(
            elements::Paragraph::new("Notes")
                .padded(genpdf::Margins::trbl(2, 0, 2, 0))
                .styled(self.style_section_title)
        );

        for note in self.invoice.notes.clone().unwrap_or(Vec::new()).iter() {

            note_layout.push(
                elements::BulletPoint::new(
                    elements::Paragraph::new(note)
                        .padded(genpdf::Margins::trbl(1, 0, 1, 0))
                        .styled(self.style_text_normal),
                ).with_bullet("•")
            );

        }
        doc.push(note_layout);
        // Finish add note

        // Render the document and write it to a file
        doc.render_to_file(format!("invoice_{}.pdf", self.invoice.number))
            .expect("Failed to write PDF file");
    }
}

fn main() {

    let invoice = Invoice::new();
    let invoice_exporter = InvoiceExporter::new(invoice);
    invoice_exporter.export();

    println!("Hello, invoice finish created!");
}
