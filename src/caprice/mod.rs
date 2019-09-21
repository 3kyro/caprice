use crate::caprice_engine::Executor;
use crate::caprice_terminal::TerminalManipulator;
use crate::Result;
use crossterm::Attribute;

pub struct Caprice {
    executor: Executor,
    terminal: TerminalManipulator,
    // callback: Option<lalala>,
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
}
/// Ensures the process exits gracefully, returning the terminal to its
/// original state
impl Drop for Caprice {
    fn drop(&mut self) {
        self.terminal.clear_from_cursor().unwrap();
        self.terminal.flush().unwrap();
        self.terminal.disable_raw_screen().unwrap();
        // reset terminal attributes
        println!("{}", Attribute::Reset);
    }
}
