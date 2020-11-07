use regex::Regex;

fn main() {
    // `r` enables us to write r"\d\d" instead of "\\d"
    // Using `unwrap()` because your RegExp may be invalid
    // but it orginates from your own code, so we don't use `expect()`
    // https://crates.io/crates/regex
    let re_date = Regex::new(r"\d{4}-\d{2}-\d{2}").unwrap();

    let valid_date_string = String::from("2020-11-11");
    let invalid_date_string = "11-11-99";

    println!("Is the date {} in valid format? {}", &valid_date_string, re_date.is_match(&valid_date_string));
    println!("Is the date {} in valid format? {}", invalid_date_string, re_date.is_match(invalid_date_string));

    // Captures
    let re_date = Regex::new(r"(\d{4})-\d{2}-\d{2}").unwrap();
    match re_date.captures(&valid_date_string) {
        // caps is something like a HashMap?
        // caps.get(m) returns a Match so we use unwrap().
        // We can also write `&caps[1]` instead of `caps.get(1).unwrap().as_str()`
        Some(caps) => println!("The year in the date is {}.", caps.get(1).unwrap().as_str()),
        None => println!("Invalid date string!")
    }

}
