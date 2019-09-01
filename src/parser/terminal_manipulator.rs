use crossterm::{Terminal, KeyEvent, ClearType, TerminalCursor, Result};




pub(super) struct TerminalManipulator {
    pub(super) terminal: crossterm::Terminal,
    pub(super) cursor: TerminalCursor
} 

impl TerminalManipulator {
    pub(super) fn new() -> Self {
        TerminalManipulator {
            terminal: Terminal::new(),
            cursor: TerminalCursor::new(),
        }

    }

    pub(super) fn next_key_event(&self) -> KeyEvent {
        unimplemented!();
    }

    pub(super) fn clear_from_cursor(&self) -> Result<()> {
        self.terminal.clear(ClearType::UntilNewLine)?;
        self.terminal.clear(ClearType::FromCursorDown)?;
        Ok(())
    }

    pub(super) fn goto_next_line(&self) -> Result<()> {
        self.clear_from_cursor()?;
        print!("\r\n");
        Ok(())
    }

    pub(super) fn clear_line(&self) -> Result<()> {
        self.terminal.clear(ClearType::UntilNewLine)
    }

    pub(super) fn save_cursor(&self) -> Result<()> {
        self.cursor.save_position()
    }

    pub(super) fn restore_cursor(&self) -> Result<()> {
        self.cursor.reset_position()
    }
}