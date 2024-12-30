use clap::Parser;

use data::DataItem;

mod cli;
mod data;

fn main() -> Result<(), anyhow::Error> {
    let cli = cli::Cli::parse();

    let data = [
        DataItem {
            name: "Mike".to_string(),
            age: 40,
            weight: 86f32,
            has_dogs: false,
        },
        DataItem {
            name: "Jeff".to_string(),
            age: 19,
            weight: 48f32,
            has_dogs: true,
        },
        DataItem {
            name: "Caren".to_string(),
            age: 35,
            weight: 96f32,
            has_dogs: true,
        },
        DataItem {
            name: "Chelsea".to_string(),
            age: 20,
            weight: 55f32,
            has_dogs: false,
        },
        DataItem {
            name: "Moa".to_string(),
            age: 13,
            weight: 40f32,
            has_dogs: false,
        },
    ]
    .into_iter()
    .collect::<Vec<_>>();

    let filtered = data
        .into_iter()
        .filter(|item| {
            if let Some(f) = cli.filter.as_ref() {
                f.evaluate(item).ok().is_some_and(|b| b)
            } else {
                true
            }
        })
        .collect::<Vec<_>>();

    println!("{}", serde_json::to_string_pretty(&filtered).unwrap());
    Ok(())
}
