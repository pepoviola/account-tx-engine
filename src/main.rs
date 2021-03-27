use std::env;
use std::error::Error;
use std::fs::File;

mod account;
mod amount;
mod engine;
mod transaction;
mod transaction_row;

/// Helper function to parse and validate the required args
/// We expect one argument and this should be an .csv file.
fn get_file_name_to_process() -> Result<String, Box<dyn Error>> {
    if let Some(file_name) = env::args().nth(1) {
        match file_name.ends_with(".csv") {
            true => Ok(file_name),
            false => Err(From::from(format!(
                "Invalid filename {}, expected an .csv file",
                file_name
            ))),
        }
    } else {
        Err(From::from("Expected 1 argument, but got none"))
    }
}

fn get_reader_from_file(file_name: &str) -> Result<csv::Reader<File>, Box<dyn Error>> {
    let reader = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_path(file_name)?;

    Ok(reader)
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_name = get_file_name_to_process()?;
    let reader = get_reader_from_file(&file_name)?;
    let mut engine = engine::PaymentEngine::new();
    engine.process_from_reader(reader)?;
    engine.output_report();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_1() -> Result<(), Box<dyn Error>> {
        let reader = get_reader_from_file("./tests/input_1.csv")?;
        let mut engine = engine::PaymentEngine::new();
        engine.process_from_reader(reader)?;
        let client_id: u16 = 1;
        if let Some(client) = engine.accounts_by_client.get(&client_id) {
            assert_eq!(client.locked, false);
            assert_eq!(client.available, amount::Amount::from_input(1.5).value());
        }

        Ok(())
    }

    #[test]
    fn input_disput() -> Result<(), Box<dyn Error>> {
        let reader = get_reader_from_file("./tests/input_d.csv")?;
        let mut engine = engine::PaymentEngine::new();
        engine.process_from_reader(reader)?;
        let client_id: u16 = 1;
        if let Some(client) = engine.accounts_by_client.get(&client_id) {
            assert_eq!(client.locked, false);
            assert_eq!(client.available, amount::Amount::from_input(0.0).value());
            assert_eq!(client.held, amount::Amount::from_input(1.5).value());
        }

        Ok(())
    }

    #[test]
    fn input_resolve() -> Result<(), Box<dyn Error>> {
        let reader = get_reader_from_file("./tests/input_r.csv")?;
        let mut engine = engine::PaymentEngine::new();
        engine.process_from_reader(reader)?;
        let client_id: u16 = 1;
        if let Some(client) = engine.accounts_by_client.get(&client_id) {
            assert_eq!(client.locked, false);
            assert_eq!(client.available, amount::Amount::from_input(1.5).value());
            assert_eq!(client.held, amount::Amount::from_input(0.0).value());
        }

        Ok(())
    }
    #[test]
    fn input_widrawal() -> Result<(), Box<dyn Error>> {
        let reader = get_reader_from_file("./tests/input_w.csv")?;
        let mut engine = engine::PaymentEngine::new();
        engine.process_from_reader(reader)?;
        let client_id: u16 = 2;
        if let Some(client) = engine.accounts_by_client.get(&client_id) {
            assert_eq!(client.locked, false);
            assert_eq!(client.available, amount::Amount::from_input(0.9988).value());
            assert_eq!(client.held, amount::Amount::from_input(0.0).value());
        }

        Ok(())
    }

    #[test]
    fn input_lock() -> Result<(), Box<dyn Error>> {
        let reader = get_reader_from_file("./tests/input_lock.csv")?;
        let mut engine = engine::PaymentEngine::new();
        engine.process_from_reader(reader)?;
        let client_id: u16 = 2;
        if let Some(client) = engine.accounts_by_client.get(&client_id) {
            assert_eq!(client.locked, true);
            assert_eq!(client.available, amount::Amount::from_input(0.0).value());
            assert_eq!(client.held, amount::Amount::from_input(0.0).value());
        }

        Ok(())
    }
}
