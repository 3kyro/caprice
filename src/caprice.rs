use crate::engine::Executor;
use crate::error::Result;
use crossterm::style::Attribute;
use std::sync::mpsc;
use std::thread::{self, JoinHandle};

/// Return type of Caprice::run
/// Sender can be used to send commands to the caprice REPL
/// Receiver can be used to receive the keywords typed by the user
/// Handle can be used to join caprices' thread
pub type CapriceMessage = (
    mpsc::Sender<CapriceCommand>,
    mpsc::Receiver<String>,
    JoinHandle<Result<()>>,
);

/// Commands that can be sent to the Caprice REPL by the invoking application
pub enum CapriceCommand {
    /// Print the provided string
    Println(String),
    /// Exit the caprice terminal
    Exit,
    /// Continue with the next keyword. This command is necessary to be given
    /// if we received a keyword from `Caprice` but do not wont to send some other
    /// coomand. See also the `spinning_square` example.
    Continue,
}

/// Builds and initializes the caprice terminal
pub struct CapriceBuilder {
    caprice: Caprice,
}

impl CapriceBuilder {
    /// Initializes the caprice REPL.
    /// This function should be the last one called in the
    /// caprice object's construction chain
    ///
    /// # Example
    /// ```rust, no_run
    ///  use caprice::Caprice;
    ///
    /// let mut caprice = Caprice::new()
    ///     .set_prompt("!:") // set the prompt
    ///     .disable_ctrl_c() // pressing control + c won't terminate the caprice console
    ///     .init(); // initializes the caprice terminal
    /// ```
    pub fn init(mut self) -> Caprice {
        if self.caprice.executor.reset_prompt().is_ok() {
            self.caprice
        } else {
            panic!("Caprice: Error initializing prompt");
        }
    }

    /// Sets the current active keywords for the parser
    ///
    /// ## Note
    /// This method __will not__ check for the length of the provided keywords,
    /// nor if these keywords can be correctly displayed in all supported
    /// terminals.
    /// This method will only include keywords that start with an alphabetic character
    pub fn set_keywords(mut self, keywords: Vec<String>) -> Self {
        self.caprice.executor.set_keywords(keywords);
        self
    }

    /// Enables Alternate Screen rendering
    pub fn enable_alternate_screen(mut self) -> Self {
        self.caprice
            .executor
            .terminal
            .enable_alternate_screen()
            .expect("Caprice: Error enabling alternate screen");
        self
    }

    /// Disables exiting the REPL when pressing ctrl+c
    pub fn disable_ctrl_c(mut self) -> Self {
        self.caprice.executor.scanner.enable_ctrl_c = false;
        self
    }

    /// Sets the prompt displayed while the caprice parser is running
    ///
    /// ## Note
    /// This method __will not__ check for the length of the provided prompt,
    /// nor if this prompt can be correctly displayed in all supported
    /// terminals.
    ///
    pub fn set_prompt(mut self, prompt: &'static str) -> Self {
        self.caprice.executor.prompt = prompt;
        self
    }
}

/// The main object of the Caprice REPL
#[derive(Debug)]
pub struct Caprice {
    executor: Executor,
}

impl Caprice {
    #![allow(clippy::new_ret_no_self)]
    /// Creates a new Caprice object.
    pub fn new() -> CapriceBuilder {
        CapriceBuilder {
            caprice: Caprice {
                executor: Executor::new(),
            },
        }
    }

    /// Runs the REPL in a separate thread returning the transmit and receive channels for message
    /// passing as well as the thread handle for its manipulation by the parent application
    pub fn run(mut self) -> Result<CapriceMessage> {
        let (tx_keyword, rx_keyword) = mpsc::channel();
        let (tx_command, rx_command) = mpsc::channel();

        let handle = thread::spawn(move || -> Result<()> {
            loop {
                if let Some(keyword) = self.eval()? {
                    tx_keyword.send(keyword)?;
                }

                if let Ok(command) = rx_command.try_recv() {
                    match command {
                        CapriceCommand::Println(msg) => {
                            self.executor.print_msg(msg)?;
                        }
                        CapriceCommand::Exit => {
                            self.executor.exec_exit()?;
                            return Ok(());
                        }
                        CapriceCommand::Continue => continue,
                    }
                }
            }
        });

        Ok((tx_command, rx_keyword, handle))
    }

    fn eval(&mut self) -> Result<Option<String>> {
        self.executor.poll()
    }
}

/// Ensures the process exits gracefully, returning the terminal to its
/// original state
impl Drop for Caprice {
    fn drop(&mut self) {
        // reset terminal attributes
        println!("{}", Attribute::Reset);
        self.executor.terminal.clear_from_cursor().unwrap();
        self.executor.terminal.flush().unwrap();
        self.executor.terminal.disable_raw_mode().unwrap();
    }
}
