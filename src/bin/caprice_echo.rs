use caprice::{Caprice, CapriceCommand};
use std::thread;
use std::time::Duration;
fn main() {
    let mut caprice = Caprice::new()
        .set_prompt("!:") // set the prompt
        .disable_ctrl_c() // pressing control + c won't terminate the caprice console
        .init(); // initialises the caprice terminal
                 // set some tokens
    caprice.set_keywords(&vec![
        "some_token".to_owned(),
        "some_other_token".to_owned(),
        "exit".to_owned(), // an exit keyword
    ]);
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
            match token.as_str() {
                // leave if the user types exit
                "exit" => {
                    tx.send(CapriceCommand::Println("bye".to_owned())).unwrap();
                    tx.send(CapriceCommand::Exit).unwrap();
                    // wait for caprice to exit
                    caprice_handle.join().unwrap().unwrap();
                    break; // caprice has already exited, let the main process do as well
                }
                // else send back the token to be printed
                _ => {
                    let print_token = format!("Hello {}", token);
                    tx.send(CapriceCommand::Println(print_token)).unwrap();
                }
            }
        }
        // let the thread sleep for some time
        thread::sleep(Duration::from_millis(10));
    }
}
