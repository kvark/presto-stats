#[derive(Debug, serde::Deserialize)]
struct Record {
    #[serde(rename = "Date")]
    date: String,
    #[serde(rename = "Transaction Number")]
    _transaction_id: String,
    #[serde(rename = "Transit Agency")]
    _agency: String,
    #[serde(rename = "Location")]
    _location: String,
    #[serde(rename = "Type ")]
    ty: String,
    #[serde(rename = "Service Class")]
    _class: String,
    #[serde(rename = "Discount")]
    discount: String,
    #[serde(rename = "Amount")]
    amount: String,
    #[serde(rename = "Balance")]
    balance: String,
}

fn money(string: &str) -> f32 {
    string.trim_start_matches('$').parse().unwrap()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_name = match std::env::args().nth(1) {
        Some(arg) => arg,
        None => {
            panic!("Please specify the path to CSV as an argument");
        }
    };
    let mut rdr = csv::Reader::from_path(file_name)?;
    let records: Vec<Record> = rdr
        .deserialize()
        .map(Result::unwrap)
        .collect();
    if records.is_empty() {
        panic!("No records found!");
    }

    let end = money(&records[0].balance);
    let last = records.last().unwrap();
    let start = money(&last.balance) + money(&last.amount);
    println!("Loads:");
    let mut loads = 0.0;
    for record in records.iter() {
        if record.ty != "Load Amount" {
            continue
        }
        loads += money(&record.amount);
        println!("\t{} on {}", record.amount, record.date);
    }
    println!("Total spent: ${:.2}", end + loads - start);
    Ok(())
}
