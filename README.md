
![Build Status: Ubuntu](https://github.com/okumaru/rust-invoice-pdf-gen/actions/workflows/rust.yml/badge.svg)

# Rust Invoice PDF Generator

Generate invoice PDF from data JSON with Rust Programming Language.

## Develop on

**Windows:** 10 (64-bit)  
**Rustc:** 1.73.0 (cc66ad468 2023-10-03)  
**Cargo:** 1.73.0 (9c4383fb5 2023-08-26)

## Dependency

- `./fonts/`
- `./invoice.json`
## Run Locally

Clone the project

```bash
  git clone git@github.com:okumaru/rust-invoice-pdf-gen.git
```

Go to the project directory

```bash
  cd rust-invoice-pdf-gen
```

Read/Change file `invoice.json`

```bash
  cat invoice.json
```

Execute run locally

```bash
  cargo run
```

Output will be on same directory of project with prefix `invoice_` and file extention `.pdf`.

## Example Data Structure

Please check on [invoice.json](./invoice.json)

```json
{
  "number": "00000002",
  "status": "PAID",
  "issuedate": "2023/10/01",
  "duedate": "2024/04/01",
  "paiddate": "2023/12/01",
  "subtotal": 4500000,
  "tax": 0,
  "total": 4500000,
  "items": [
    {
      "description": "Percobaan",
      "quantity": 1,
      "price": 18000000,
      "amount": 18000000
    }
  ],
  "transactions": {
    "balance": 0,
    "items": [
      {
        "id": "asd_123_zxc_456_qwe",
        "date": "2023/11/30",
        "amount": 3000000
      }
    ]
  },
  "invto": {
    "name": "CV. XYZ",
    "address": "Jl. Google, Malang, Jawa Timur, Indonesia. 60000",
    "phone": null,
    "email": "xyz@gmail.com"
  },
  "invfrom": {
    "name": "Andhika M. Wijaya",
    "address": "Dusun Glanggang, Slamet, Kec. Tumpang, Malang, Jawa Timur, Indonesia. 65156",
    "phone": "(+62) 851 5695 0905",
    "email": "Andhikamarthawijaya@gmail.com"
  },
  "notes": ["testing 1", "testing 2", "testing 3"]
}

```

## License

See the [LICENSE](LICENSE.md) file for license rights and limitations (MIT).