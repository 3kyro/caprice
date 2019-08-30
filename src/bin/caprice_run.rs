use caprice::Flags;

use std::collections::BTreeMap;

fn main() {
    let mut map: BTreeMap<String, bool> = BTreeMap::new();

    map.insert("simulate_bms".to_owned(), false);
    map.insert("simulate_battery".to_owned(), false);
    map.insert("three".to_owned(), false);

    let mut flags = Flags::from_map(&map);

    println!("Welcome to caprice runtime.");
    println!("Autocomplete by pressing tab,");
    println!("type #list to get a list of availiable options,");
    println!("ctrl+c to exit.");

    flags.init();
    loop {
        flags.run();
    }
}
