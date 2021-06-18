use survey_rs::{
    question::Question,
    select::{Select, SelectOptions},
};

extern crate survey_rs;

fn main() {
    let options = vec![
        "Banana",
        "Apple",
        "Strawberry",
        "Grapes",
        "Lemon",
        "Tangerine",
        "Watermelon",
        "Orange",
        "Pear",
        "Avocado",
        "Pineapple",
    ];

    let ans = SelectOptions::new("What's your favorite fruit?", &options)
        .map(|so| so.with_page_size(10))
        .and_then(|so| so.with_starting_cursor(1))
        .map(Select::from)
        .and_then(Select::prompt)
        .expect("Failed when creating so");

    println!("Final answer was {}", ans);
}