use caprice::parser::*;

fn main() {
    let mut parser = Parser::new(functor);

    parser.init().unwrap();
    loop {
        parser.parse().unwrap();
    }
}

fn functor(s: String) -> Result<(), std::io::Error> {
    println!("{} activated", s);

    Ok(())
}

// use caprice::Flags;

// use std::collections::BTreeMap;

// fn main() {
//     let mut map: BTreeMap<String, bool> = BTreeMap::new();

//     map.insert("simulate_bms".to_owned(), false);
//     map.insert("simulate_battery".to_owned(), false);
//     map.insert("three_1".to_owned(), false);
//     map.insert("three_2".to_owned(), false);
//     map.insert("three_3".to_owned(), false);
//     map.insert("three_4".to_owned(), false);
//     map.insert("three_5".to_owned(), false);
//     map.insert("three_6".to_owned(), false);
//     map.insert("three_7".to_owned(), false);
//     map.insert("three_8".to_owned(), false);
//     map.insert("three_9".to_owned(), false);
//     map.insert("three_10".to_owned(), false);
//     map.insert("three_11".to_owned(), false);
//     map.insert("three_12".to_owned(), false);

//     let mut flags = Flags::from_map(&map);

//     println!("Welcome to caprice runtime.");
//     println!("Autocomplete by pressing tab,");
//     println!("type #list to get a list of availiable options,");
//     println!("ctrl+c to exit.");

//     flags.init();
//     loop {
//         if let Ok(()) = flags.run() {
//             continue;
//         } else {
//             break;
//         }
//     }
// }
