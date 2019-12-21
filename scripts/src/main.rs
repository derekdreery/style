use std::collections::BTreeSet;

fn main() {
    let mut rdr = csv::Reader::from_path("../all-properties.en.csv").unwrap();
    let mut all_properties = BTreeSet::new();
    for row in rdr.records() {
        let row = row.unwrap();
        match row.get(2).unwrap() {
            "CR" | "REC" => {
                all_properties.insert(row.get(0).unwrap().to_owned());
            }
            _ => (),
        }
    }
    eprintln!("Count: {}", all_properties.len());
    println!("# CSS properties");
    println!();
    for prop in all_properties.iter() {
        if prop.contains("*") {
            continue;
        }
        println!(" - [ ] {}", prop);
    }
}
