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

    // Since transit-pass-related transactions record a balance of 0, we need
    // to look for the non-zero balances to get our true start and end balances.
    let mut non_zero_balance_iter = records.iter().filter(|rec| money(&rec.balance) != 0.0);
    let first = non_zero_balance_iter.next();
    let last = non_zero_balance_iter.last();

    let mut end = 0.0;
    let mut start = 0.0;
    if let Some(rec) = first { end = money(&rec.balance) }
    if let Some(rec) = last { start = money(&rec.balance) + money(&rec.amount) }

    let mut transit_uses = 0;
    println!("Loads:");
    let mut loads = 0.0;
    for record in records.iter() {
        if record.ty == "Transit Pass Payment" {
            transit_uses += 1;
            continue
        } else if record.ty != "Load Amount" && record.ty != "Load Transit Pass" {
            continue
        }
        loads += money(&record.amount);
        println!("\t{} on {}", record.amount, record.date);
    }
    if transit_uses > 0 {
        println!("Transit pass(es) used {} times.", transit_uses);
    }
    println!("Total spent: ${:.2}", end + loads - start);
    Ok(())
}
