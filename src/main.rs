mod grammar;
mod production;
mod first;

use production::Production;
use grammar::Grammar;
// use std::io;
// use std::str::FromStr;

// const MENU_TEXT: &str = "[1] Add Production\n[2] Compute firsts\n[3] Exit";

// #[derive(Debug)]
// enum MenuChoices {
//     Add = 1,
//     Compute = 2,
//     Exit = 3,
// }

// impl FromStr for MenuChoices {
//     type Err = String;

//     fn from_str(s: &str) -> Result<Self, String> {
//         match s.replace('\n', "").parse::<u8>() {
//             Ok(n) => match n {
//                 1 => Ok(MenuChoices::Add),
//                 2 => Ok(MenuChoices::Compute),
//                 3 => Ok(MenuChoices::Exit),
//                 _ => Err(format!("{} is an invalid option.", n))
//             },
//             Err(value) => Err(format!("{}.", value)),
//         }
//     }
// }

// fn menu_add_action(productions_collection: &mut ProductionCollection) {
//     let mut buffer = String::new();
//     let mut variable: char;
//     let mut derivation: String;

//     loop {
//         println!("Insert the variable (must be an uppercase letter):");
//         buffer.clear();

//         io::stdin()
//             .read_line(&mut buffer)
//             .expect("Error reading input");

//         if let Ok(ch) = buffer.replace('\n', "").parse::<char>() {
//             variable = ch;
//         } else {
//             println!("Invalid variable, type again please.");
//             continue;
//         }

//         buffer.clear();
//         println!("Now type the derivation, use £ (Altgr + 4) for epsilon derivation:");
//         io::stdin()
//             .read_line(&mut buffer)
//             .expect("Error reading input");

//         if let Ok(d) = buffer.parse::<String>() {
//             derivation = d.replace('\n', "");

//             match Production::new(variable, derivation) {
//                 Ok(p) => productions_collection.add(p),
//                 Err(msg) => {
//                     println!("[ERROR]: {}", msg);
//                     println!("Please insert production again:");
//                     continue;
//                 }
//             };
//             break;
//         } else {
//             println!("Invalid derivation, type again please.");
//             continue;
//         }
//     }
// }

// fn menu_compute_action(productions_collection: &mut ProductionCollection) {
//     productions_collection.compute_all_firsts();
//     println!("{}", productions_collection);

//     std::process::exit(0)
// }

// fn main() {
//     let mut productions_collection = ProductionCollection::new();

//     let mut buffer = String::new();

//     println!("FIRST AND FOLLOW IMPLEMENTATION!");

//     loop {
//         println!("{}", MENU_TEXT);
//         buffer.clear();
//         io::stdin()
//             .read_line(&mut buffer)
//             .expect("Error reading input");

//         match buffer.parse::<MenuChoices>() {
//             Ok(MenuChoices::Add) => menu_add_action(&mut productions_collection),
//             Ok(MenuChoices::Compute) => menu_compute_action(&mut productions_collection),
//             Ok(MenuChoices::Exit) => std::process::exit(0),
//             Err(e) => println!("Invalid Option error: {}", e),
//         }
//     }
// }

fn main() {

    let mut grammar = Grammar {
        variables: vec!['E', 'Z', 'T', 'Y', 'X'],
        terminals: vec!["+".to_string(), "*".to_string(), "(".to_string(), "id".to_string(), ")".to_string()],
        productions: vec![],
    };

    grammar.add_production(Production::new('E', "TZ".to_string()).unwrap());
    grammar.add_production(Production::new('Z', "+TZ | £".to_string()).unwrap());
    grammar.add_production(Production::new('T', "XY".to_string()).unwrap());
    grammar.add_production(Production::new('Y', "*XY | £".to_string()).unwrap());
    grammar.add_production(Production::new('X', "(E) | id".to_string()).unwrap());

    grammar.compute_firsts();

    println!("{}", grammar);
}