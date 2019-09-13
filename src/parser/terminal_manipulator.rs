use crossterm::{
    input, ClearType, InputEvent, RawScreen, Result, SyncReader, Terminal,
    TerminalCursor, AlternateScreen,
};
use std::io::{stdout, Stdout, Write};

pub(super) struct TerminalManipulator {
    terminal: crossterm::Terminal,
    cursor: TerminalCursor,
    stdin: SyncReader,
    stdout: Stdout,
    screen: Option<AlternateScreen>,
}

impl TerminalManipulator {
    pub(super) fn new() -> Self {
        TerminalManipulator {
            terminal: Terminal::new(),
            cursor: TerminalCursor::new(),
            stdin: input().read_sync(),
            stdout: stdout(),
            screen: None,
        }
    }

    pub(super) fn next_key_event(&mut self) -> Option<InputEvent> {
        self.stdin.next()
    }

    pub(super) fn clear_from_cursor(&self) -> Result<()> {
        self.terminal.clear(ClearType::FromCursorDown)?;
        self.terminal.clear(ClearType::UntilNewLine)?;
        Ok(())
    }

    pub(super) fn goto_next_line(&self) -> Result<()> {
        self.clear_from_cursor()?;
        println!("\r");
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

    pub(super) fn enable_alternate_screen(&mut self) -> Result<()> {
        self.screen = Some(AlternateScreen::to_alternate(true)?);
        self.cursor.goto(0, 0)?;
        Ok(())
    }

    pub(crate) fn disable_raw_screen(&self) -> Result<()> {
        RawScreen::disable_raw_mode()?;

        Ok(())
    }

    pub(crate) fn flush(&mut self) -> Result<()> {
        self.stdout.flush()?;

        Ok(())
    }

    pub(crate) fn goto_begining_of_line(&mut self) {
        self.cursor.move_left(self.cursor.pos().0);
    }

    pub(crate) fn size(&self) -> (u16, u16) {
        self.terminal.terminal_size()
    }

    pub(crate) fn get_cursor_pos(&self) -> (u16, u16) {
        self.cursor.pos()
    }

    pub(crate) fn scroll_up(&mut self, step: i16) -> Result<()> {
        self.terminal.scroll_up(step)?;
        self.cursor.move_up(step as u16);

        Ok(())
    }

    pub(crate) fn backspace(&mut self) -> Result<()> {
        self.cursor.move_left(1);
        self.clear_line()?;

        Ok(())
    }
}
