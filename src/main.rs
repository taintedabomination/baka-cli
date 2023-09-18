use reqwest;
use std::collections::HashMap;
use std::env;
use tokio;

async fn test_fetch() -> Result<(), reqwest::Error> {
    let url = "https://jsonplaceholder.typicode.com/posts/1";
    let response = reqwest::get(url).await?;
    let body = response.text().await?;

    println!("Odpověď: {}", body);

    Ok(())
}

fn print_timetable() -> () {
    println!("Rozvrh");
}

fn print_marks() -> () {
    println!("Známky");
}

fn print_changes() -> () {
    println!("Změny v rozvrhu");
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let total_args = args.len();

    let _ = test_fetch().await;

    let mut command_map: HashMap<&str, Box<dyn Fn()>> = HashMap::new();
    command_map.insert("rozvrh", Box::new(print_timetable));
    command_map.insert("znamky", Box::new(print_marks));
    command_map.insert("zmeny", Box::new(print_changes));

    if total_args <= 1 {
        eprintln!("Použití: baka-cli <prikaz>");
        std::process::exit(1);
    }

    let command = &args[1];
    match command_map.get(command.as_str()) {
        Some(func) => func(),
        None => {
            eprintln!("Neznámý příkaz: {}!", command);
            std::process::exit(1);
        }
    }
}
