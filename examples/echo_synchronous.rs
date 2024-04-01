use caprice::{theme::DARK_BLUE, Caprice, CapriceCommand};
fn main() {
    let mut caprice = Caprice::new()
        .set_prompt("!:") // set the prompt
        .disable_ctrl_c() // pressing control + c won't terminate the caprice console
        .theme(DARK_BLUE)
        .set_keywords(vec![
            // set some tokens
            "some_token",
            "some_other_token",
            "exit", // an exit keyword
        ])
        .init()
        .unwrap(); // initializes the caprice terminal

    loop {
        // Block until we get the next keyword from `Caprice`.
        let response = caprice.get().unwrap();
        // keyword can contain arguments, split on spaces
        let mut args = response.as_str().trim_end().split(' ');
        // First arg is the keyword
        if let Some(keyword) = args.next() {
            match keyword {
                "exit" => {
                    // Clean up the terminal
                    caprice.send(Some(CapriceCommand::Exit)).unwrap();
                    break;
                }
                _ => {
                    // Format what we got, separating keyword
                    // arguments with commas
                    let print_token = format!(
                        "Got {}({}) from Caprice",
                        keyword,
                        args.collect::<Vec<&str>>().join(", ")
                    );
                    caprice
                        .send(Some(CapriceCommand::Println(print_token)))
                        .unwrap();
                }
            }
        }
    }
}
