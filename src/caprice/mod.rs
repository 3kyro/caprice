use crate::caprice_engine::Executor;
use crate::caprice_terminal::TerminalManipulator;
use crate::Result;
use crossterm::Attribute;
use std::thread;
use std::sync::mpsc;
use std::mem::drop;

pub enum CapriceCommand {
    Println(String),
    Exit,
}

pub struct Caprice {
    executor: Executor,
    terminal: TerminalManipulator,
    tx_out: Option<mpsc::Sender<String>>,
    rx_in: Option<mpsc::Receiver<CapriceCommand>>,
}

impl Caprice {
    // pub fn set_callback(&mut self, functor: Arc<Mutex<dyn FnOnce(String)>>) {
    //     self.callback = Some(functor);
    // }
    /// Creates a new Caprice object
    pub fn new() -> Self {
        Caprice {
            executor: Executor::new(),
            terminal: TerminalManipulator::new(),
            tx_out: None,
            rx_in: None, 
        }
    }

    /// Sets the current active keywords for the parser
    ///
    pub fn set_keywords(&mut self, keywords: &Vec<String>) {
        self.executor.set_keywords(keywords);
    }

    pub fn init(mut self) -> Self {
        self.executor.reset_prompt().unwrap();
        self
    }

    pub fn enable_alternate_screen(mut self) -> Self {
        self.terminal.enable_alternate_screen().unwrap();
        self
    }

    pub fn enable_raw_screen(mut self) -> Self {
        self.terminal.enable_raw_screen().unwrap();
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

    /// Caprice internally is using Crossterms Rawmode for terminal manipulation.
    /// In order for the process to exit correcktly, cleaning up all changes made
    /// to the current terminal, a standard process::exit() procedure cannot be used.
    /// Instead eval will return a Error::new(ErrorKind::Interrupted, "Program Exit"),
    /// which the calling funxtion should interpret as a stop command
    ///
    /// # Example
    /// ```
    /// loop {
    ///     // ignoring possible token return
    ///     if let Ok(_) = caprice_instance.eval() {}
    ///     else {
    ///         break
    ///     }
    /// }
    pub fn eval(&mut self) -> Result<Option<String>> {
        self.executor.poll()
    }

    pub fn run(mut self) -> (mpsc::Sender<CapriceCommand>, mpsc::Receiver<String>) {

        let (tx_stop, rx_token) = self.channels();

        let tx = self.tx_out.clone().unwrap();

        thread::spawn(move || {
            loop {
                if let Ok(option) = self.eval() {
                    if let Some(keyword) = option {
                        tx.send(keyword).unwrap();
                    }
                } else {
                    
                    break;
                }

                if let Some(rx) = &self.rx_in {
                    if let Ok(command) = rx.try_recv() {
                        match command {
                            CapriceCommand::Println(msg) => {
                                dbg!("gere");
                                self.executor.print_msg(msg);
                            }
                            CapriceCommand::Exit => {
                                drop(self);
                                break; }
                        }
                    }
                }
            }

        });

        (tx_stop, rx_token)
        
    }

    fn channels(&mut self) -> (mpsc::Sender<CapriceCommand>, mpsc::Receiver<String>) {
        let (tx_token, rx_token) = mpsc::channel();
        let (tx_stop, rx_stop) = mpsc::channel();

        self.tx_out = Some(tx_token);
        self.rx_in = Some(rx_stop);

        (tx_stop, rx_token)
    }

}
/// Ensures the process exits gracefully, returning the terminal to its
/// original state
impl Drop for Caprice {
    fn drop(&mut self) {
        dbg!("dropping");
        self.terminal.clear_from_cursor().unwrap();
        self.terminal.flush().unwrap();
        self.terminal.disable_raw_screen().unwrap();
        // reset terminal attributes
        println!("{}", Attribute::Reset);
    }
}
