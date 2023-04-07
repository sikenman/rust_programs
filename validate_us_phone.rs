use regex::Regex;

fn is_valid_us_phone_no(phone_number: &str) -> bool {
    let regex = Regex::new(r#"^\(?[2-9][0-8][0-9]\)?[-.\s]?[2-9][0-9]{2}-[0-9]{4}$"#).unwrap();

    regex.is_match(phone_number)
}

fn main() {
    let phone_numbers = ["(555) 555-1234", "555-555-1234", "1-555-555-1234", "5555551234"];


    for ph in phone_numbers {
        println!("{} is valid: {}", ph, is_valid_us_phone_no(ph));
    }
}
