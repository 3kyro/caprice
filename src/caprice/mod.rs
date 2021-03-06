use crate::caprice_engine::Executor;
use crate::caprice_error::Result;
use crossterm::style::Attribute;
use std::sync::mpsc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

/// Return type of Caprice::run
/// Sender can be used to send commands to the caprice repl
/// Receiver can be used to receive the keywords typed by the user
/// Handle can be used to join caprices' thread
pub type CapriceMessage = (
    mpsc::Sender<CapriceCommand>,
    mpsc::Receiver<String>,
    JoinHandle<Result<()>>,
);

/// Commands that can be sent to the Caprice repl by the invoking application
pub enum CapriceCommand {
    Println(String),
    Exit,
}

/// The main object of the Caprice repl
#[derive(Debug)]
pub struct Caprice {
    executor: Executor,
    tx_out: Option<mpsc::Sender<String>>,
    rx_in: Option<mpsc::Receiver<CapriceCommand>>,
}

impl Caprice {
    /// Creates a new Caprice object.
    pub fn new() -> Self {
        Caprice {
            executor: Executor::new(),
            tx_out: None,
            rx_in: None,
        }
        .enable_raw_mode()
    }

    /// Sets the current active keywords for the parser
    ///
    /// ## Note
    /// This method __will not__ check for the length of the provided keywords,
    /// nor if these keywords can be correctly displayed in all supported
    /// terminals.
    /// This method will only include keywords that start with an alphabetic character
    pub fn set_keywords(mut self, keywords: &[String]) -> Self {
        self.executor.set_keywords(keywords);
        self
    }

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
    pub fn init(mut self) -> Self {
        if self.executor.reset_prompt().is_ok() {
            self
        } else {
            panic!("Caprice: Error initializing prompt");
        }
    }

    /// Enables Alternate Screen rendering
    pub fn enable_alternate_screen(mut self) -> Self {
        self.executor
            .terminal
            .enable_alternate_screen()
            .expect("Caprice: Error enabling alternate screen");
        self
    }

    /// Disables exiting the repl when pressing ctrl+c
    pub fn disable_ctrl_c(mut self) -> Self {
        self.executor.scanner.enable_ctrl_c = false;
        self
    }

    /// Sets the prompt displayed while the caprice parser is running
    ///
    /// ## Note
    /// This method __will not__ check for the length of the provided prompt,
    /// nor if this prompt can be correctly displayed in all supported
    /// terminals.
    ///
    pub fn set_prompt(mut self, prompt: &str) -> Self {
        self.executor.set_prompt(prompt);
        self
    }

    /// Runs the repl in a separate thread returning the transmit and receive channels for message
    /// passing as well as the thread handle for its manipulation by the parent application
    pub fn run(mut self) -> Result<CapriceMessage> {
        let (tx_stop, rx_token) = self.channels();

        let tx = self
            .tx_out
            .clone()
            .expect("Caprice: Uninitialized channels");

        let handle = thread::spawn(move || -> Result<()> {
            loop {
                // give the cpu some time
                thread::sleep(Duration::from_millis(10));

                if let Some(keyword) = self.eval()? {
                    tx.send(keyword)?;
                }

                if let Some(rx) = &self.rx_in {
                    if let Ok(command) = rx.try_recv() {
                        match command {
                            CapriceCommand::Println(msg) => {
                                self.executor.print_msg(msg)?;
                            }
                            CapriceCommand::Exit => {
                                self.executor.exec_exit()?;
                                return Ok(());
                            }
                        }
                    }
                }
            }
        });

        Ok((tx_stop, rx_token, handle))
    }

    fn eval(&mut self) -> Result<Option<String>> {
        self.executor.poll()
    }

    // Creates and binds the channels used for communication between caprice and the parent application
    fn channels(&mut self) -> (mpsc::Sender<CapriceCommand>, mpsc::Receiver<String>) {
        let (tx_token, rx_token) = mpsc::channel();
        let (tx_stop, rx_stop) = mpsc::channel();

        self.tx_out = Some(tx_token);
        self.rx_in = Some(rx_stop);

        (tx_stop, rx_token)
    }

    // Enables raw mode. Raw mode is enabled by default during caprice's creation
    fn enable_raw_mode(mut self) -> Self {
        self.executor
            .terminal
            .enable_raw_mode()
            .expect("Caprice: Error enabling raw terminal mode");
        self
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

impl Default for Caprice {
    fn default() -> Self {
        Self::new()
    }
}
