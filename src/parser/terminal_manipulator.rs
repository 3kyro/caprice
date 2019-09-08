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

    pub(crate) fn size(&self) -> (u16, u16) {
        self.terminal.terminal_size()
    }

    pub(crate) fn move_cursor_up(&mut self, step: i16) {
        self.cursor.move_up(step as u16);
    }

    pub(crate) fn move_cursor_down(&mut self, step: i16) {
        self.cursor.move_down(step as u16);
    }

    pub(crate) fn get_cursor_pos(&self) -> (u16, u16) {
        self.cursor.pos()
    }

    pub(crate) fn scroll_down(&self, step: i16) -> Result<()> {
        self.terminal.scroll_down(step)?;
        Ok(())
    }

    pub(crate) fn scroll_up(&self, step: i16) -> Result<()> {
        self.terminal.scroll_up(step)?;
        Ok(())
    }

    pub(crate) fn cursor_to_last_line(&mut self) -> Result<()> {
        let ydiff = (self.terminal.terminal_size().1 - self.cursor.pos().1 - 1) as i16;
        if ydiff != 0 {
            self.terminal.scroll_down(
                (ydiff) as i16
            )?;
            self.cursor.move_down(ydiff as u16);
        } 
        Ok(())
    }

    
}
