//! Caprice is a [REPL](https://en.wikipedia.org/wiki/Read%E2%80%93eval%E2%80%93print_loop) for Rust projects featuring an easy to use, zsh like
//! autocomplete feature.
//!
//! # Example:
//! ```
//! use caprice::{Caprice, CapriceCommand};
//!
//! let mut caprice = Caprice::new()
//!     .set_prompt("!:") // set the prompt
//!     .enable_alternate_screen(false) // do not use alternate screen
//!     .disable_ctrl_c() // pressing control + c won't terminate the caprice console
//!     .init(); // initialises the caprice terminal
//!
//! // set some keywords
//! caprice.set_keywords(&vec![
//!    "some_token".to_owned(),
//!    "some_other_token".to_owned(),
//!    "exit".to_owned(), // an exit keyword
//! ]);
//!
//! // caprice.run() will execute the repl in a separate thread.
//! // You can use the returned tx and rx channels for receiving and sending messages
//! // to caprice and the handle to join Caprice's thread with the main thread.
//! let (tx,rx, handle) = caprice.run();
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
//!                 tx.send(CapriceCommand::Println("bye".to_owned())).unwrap();  
//!                 tx.send(CapriceCommand::Exit).unwrap();
//!                 caprice_handle.join().expect("couldn't join thread").expect("Caprice run has encountered an error");
//!                 break; // at this point caprice has already exited, let the main process do as well
//!             },
//!             // else send back the token to be printed
//!             _ => {
//!                 let print_token = format!("Got {} from Caprice.", token);
//!                 tx.send(CapriceCommand::Println(print_token)).unwrap();
//!             }
//!         }   
//!     }
//!     // let the thread sleep for some time
//!     thread::sleep(Duration::from_millis(10));
//! }
//! ```

pub mod caprice;
mod caprice_autocomplete;
mod caprice_engine;
mod caprice_scanner;
mod caprice_terminal;
pub use caprice::Caprice;
pub use caprice::CapriceCommand;
