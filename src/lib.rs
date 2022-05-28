//! Caprice is a simple [REPL](https://en.wikipedia.org/wiki/Read%E2%80%93eval%E2%80%93print_loop) for Rust projects featuring an easy to use, zsh like
//! autocomplete feature.
//!
//! For a synchronous execution example, see `echo_synchronous` in the examples folder
//!
//! # Example running `Caprice` on a separate thread:
//! ```rust, no_run
//! use caprice::{Caprice, CapriceCommand};
//! use std::thread;
//! use std::time::Duration;
//!
//! let mut caprice = Caprice::new()
//!     .set_prompt("!:") // set the prompt
//!     .enable_alternate_screen() // use alternate screen
//!     .disable_ctrl_c() // pressing control+c won't terminate the caprice console
//!     .set_keywords(vec![
//!         "some_token".to_owned(),
//!         "some_other_token".to_owned(),
//!         "exit".to_owned(), // an exit keyword
//!     ])
//!     .init()
//!     .unwrap(); // initializes the caprice terminal
//!
//! // caprice.run() will execute the REPL in a separate thread.
//! // You can use the returned tx and rx channels for receiving and sending messages
//! // to caprice and the handle to join Caprice's thread with the main thread.
//! let (tx,rx, handle) = caprice.run().unwrap();
//!
//! // Our main application runs here.
//! // For this example we will simply print back
//! // the tokens sent by caprice
//!
//! loop {
//!     // check if we received a token from caprice
//!     if let Ok(token) = rx.try_recv() {
//!         match token.as_str() {
//!             // leave if the user types exit
//!             "exit" => {
//!                 // print a message to the screen before exiting
//!                 tx.send(CapriceCommand::Println("bye".to_owned())).unwrap();
//!                 // send an exit command
//!                 tx.send(CapriceCommand::Exit).unwrap();
//!                 handle.join().expect("couldn't join thread").expect("Caprice run has encountered an error");
//!                 break; // at this point caprice has already exited, let the main process do as well
//!             },
//!             // else send back the token to be printed
//!             _ => {
//!                 let print_token = format!("Got {} from Caprice.", token);
//!                 tx.send(CapriceCommand::Println(print_token)).unwrap();
//!             }
//!         }
//!     }
//!     // Caprice thread polling period.
//!     thread::sleep(Duration::from_millis(100));
//! }
//! ```

mod autocomplete;
pub mod caprice;
mod engine;
pub mod error;
mod scanner;
mod terminal;
pub mod theme;
pub use self::caprice::Caprice;
pub use self::caprice::CapriceCommand;
