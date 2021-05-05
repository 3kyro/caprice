use caprice::{Caprice, CapriceCommand};
use std::thread;
use std::time::Duration;
fn main() {
    let caprice = Caprice::new()
        .set_prompt("!:") // set the prompt
        .disable_ctrl_c() // pressing control + c won't terminate the caprice console
        .set_keywords(&[
            // set some tokens
            "some_token".to_owned(),
            "some_other_token".to_owned(),
            "exit".to_owned(), // an exit keyword
        ])
        .init(); // initializes the caprice terminal

    // caprice.run() will run the caprice in a separate thread.
    // you can use the returned tx and rx channels for receiving and sending messages
    // to caprice
    let (tx, rx, caprice_handle) = caprice.run().unwrap();
    // our main application runs here
    // for this example we will simply print back
    // the tokens send by caprice
    loop {
        // if we received a token from caprice
        if let Ok(token) = rx.try_recv() {
            // token can contain arguments, split on a ' '
            let mut args = token.as_str().trim_end().split(' ');

            if let Some(token) = args.next() {
                match token {
                    // leave if the user types exit
                    "exit" => {
                        tx.send(CapriceCommand::Exit).unwrap();
                        caprice_handle
                            .join()
                            .expect("couldn't join thread")
                            .expect("Caprice run has encountered an error");
                        break; // at this point caprice has already exited, let the main process do as well
                    }
                    // else send back the token to be printed
                    _ => {
                        let print_token = format!("Got {}({}) from Caprice", token, args.collect::<Vec<&str>>().join(", "));
                        tx.send(CapriceCommand::Println(print_token)).unwrap();
                    }
                }
            }
        }
        // let the thread sleep for some time
        thread::sleep(Duration::from_millis(10));
    }
}
