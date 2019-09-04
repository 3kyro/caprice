use crossterm::{
    input, Attribute, ClearType, InputEvent, RawScreen, Result, SyncReader, Terminal,
    TerminalCursor,
};
use std::io::{stdout, Stdout, Write};

pub(super) struct TerminalManipulator {
    pub(crate) terminal: crossterm::Terminal,
    pub(crate) cursor: TerminalCursor,
    stdin: SyncReader,
    stdout: Stdout,
}

impl TerminalManipulator {
    pub(super) fn new() -> Self {
        TerminalManipulator {
            terminal: Terminal::new(),
            cursor: TerminalCursor::new(),
            stdin: input().read_sync(),
            stdout: stdout(),
        }
    }

    pub(super) fn next_key_event(&mut self) -> Option<InputEvent> {
        self.stdin.next()
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

    pub(super) fn enable_raw_screen(&self) -> Result<()> {
        let mut screen = RawScreen::into_raw_mode()?;
        screen.disable_drop();

        Ok(())
    }

    pub(crate) fn flush(&mut self) -> Result<()> {
        self.stdout.flush()?;

        Ok(())
    }

    pub(crate) fn exit(&self) -> Result<()> {
        RawScreen::disable_raw_mode()?;
        println!("{}", Attribute::Reset);
        self.terminal.exit();

        Ok(())
    }

    pub(crate) fn goto_begining_of_line(&mut self) {
        self.cursor.move_left(self.cursor.pos().0);
    }
}
