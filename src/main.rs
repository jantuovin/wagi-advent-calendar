use chrono::{Datelike, Utc};
use handlebars::Handlebars;
use std::collections::BTreeMap;

fn main() {
    println!("Content-Type: text/html\n");

    // create the handlebars registry
    let mut handlebars = Handlebars::new();

    // register the template
    handlebars
        .register_template_file("template", "/templates/template.hbs")
        .unwrap();

    // Prepare data
    let current = Utc::now();
    let days_until_christmas =
        count_days_to_christmas(current.year(), current.month(), current.date_naive().day());

    // The data type should implements `serde::Serialize`
    let mut data = BTreeMap::new();
    data.insert(
        "days_until_christmas".to_string(),
        format!("{}", days_until_christmas),
    );

    println!("{}", handlebars.render("template", &data).unwrap());
}

fn count_days_to_christmas(year: i32, month: u32, day: u32) -> u32 {
    if 2022 < year {
        return 0;
    }
    if month == 12 && 24 < day {
        return 0;
    }
    if month == 11 {
        return 30 - day + 24;
    }
    return 24 - day;
}
