use crate::caprice_engine::Executor;
use crate::caprice_terminal::TerminalManipulator;
use crossterm::style::Attribute;
use std::sync::mpsc;
use std::thread::{self, JoinHandle};
use std::time::Duration;
use anyhow::Result;

pub type CapriceMessage = (
    mpsc::Sender<CapriceCommand>,
    mpsc::Receiver<String>,
    JoinHandle<()>,
);

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
    /// Creates a new Caprice object
    pub fn new() -> Self {
        Caprice {
            executor: Executor::new(),
            terminal: TerminalManipulator::new(),
            tx_out: None,
            rx_in: None,
        }
        .enable_raw_screen()
    }

    /// Sets the current active keywords for the parser
    ///
    pub fn set_keywords(&mut self, keywords: &Vec<String>) {
        self.executor.set_keywords(keywords);
    }

    pub fn init(mut self) -> Self {
        self.executor.reset_prompt();
        self
    }

    pub fn enable_alternate_screen(mut self, flag: bool) -> Self {
        if flag {
            self.terminal.enable_alternate_screen();
        } else {
            self.terminal.enable_raw_screen();
        }
        self
    }

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

    pub fn eval(&mut self) -> Option<String> {
        self.executor.poll()
    }


    
    pub fn run(
        mut self,
    ) -> Result<CapriceMessage> {
        let (tx_stop, rx_token) = self.channels();

        let tx = self.tx_out.clone().unwrap();

        let handle = thread::spawn(move || loop {
            thread::sleep(Duration::from_millis(10));

            if let Some(keyword) = self.eval() {
                tx.send(keyword).unwrap();
            }

            if let Some(rx) = &self.rx_in {
                if let Ok(command) = rx.try_recv() {
                    match command {
                        CapriceCommand::Println(msg) => {
                            self.executor.print_msg(msg);
                        }
                        CapriceCommand::Exit => {
                            self.executor.exec_exit();
                            break;
                        }
                    }
                }
            }
        });

        Ok((tx_stop, rx_token, handle))
    }

    fn channels(&mut self) -> (mpsc::Sender<CapriceCommand>, mpsc::Receiver<String>) {
        let (tx_token, rx_token) = mpsc::channel();
        let (tx_stop, rx_stop) = mpsc::channel();

        self.tx_out = Some(tx_token);
        self.rx_in = Some(rx_stop);

        (tx_stop, rx_token)
    }

    pub fn enable_raw_screen(mut self) -> Self {
        self.terminal.enable_raw_screen();
        self
    }
}
/// Ensures the process exits gracefully, returning the terminal to its
/// original state
impl Drop for Caprice {
    fn drop(&mut self) {
        self.terminal.clear_from_cursor();
        self.terminal.flush();
        self.terminal.disable_raw_screen();
        // reset terminal attributes
        println!("{}", Attribute::Reset);
    }
}
