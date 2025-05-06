use chrono::{Local, NaiveDate}; // Import the necessary chrono items
use error_types::*; // Make sure to import the `error_types` module

fn main() {
    let mut form_output = Form {
        first_name: "Lee".to_owned(),
        last_name: "Smith".to_owned(),
        birth: NaiveDate::from_ymd(1990, 5, 1),
        birth_location: "USA".to_owned(),
        password: "qwqwsa1dty_".to_owned(),
    };

    println!("{:?}", form_output);  // Output form details

    // Test case with empty first_name
    form_output.first_name = "".to_owned();
    println!("{:?}", form_output.validate());

    // Test case with short password
    form_output.first_name = "as".to_owned();
    form_output.password = "dty_1".to_owned();
    println!("{:?}", form_output.validate());

    // Test case with password missing symbols
    form_output.password = "asdasASd(_".to_owned();
    println!("{:?}", form_output.validate());

    // Test case with password missing numbers
    form_output.password = "asdasASd123SA".to_owned();
    println!("{:?}", form_output.validate());

    // Valid case
    form_output.password = "qwqwsa1dty_".to_owned();
    println!("{:?}", form_output.validate());
}
