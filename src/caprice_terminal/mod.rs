use crossterm::{
    input, AlternateScreen, ClearType, InputEvent, RawScreen, Result, AsyncReader, Terminal,
    TerminalCursor,
};
use std::io::{stdout, Stdout, Write};

enum ScreenType {
    RawScreenType(RawScreen),
    AlternateScreenType(AlternateScreen),
    DefaultScreen,
}

pub(super) struct TerminalManipulator {
    terminal: crossterm::Terminal,
    cursor: TerminalCursor,
    stdin: AsyncReader,
    stdout: Stdout,
    screen: ScreenType,
}

impl TerminalManipulator {
    pub(super) fn new() -> Self {
        TerminalManipulator {
            terminal: Terminal::new(),
            cursor: TerminalCursor::new(),
            stdin: input().read_async(),
            stdout: stdout(),
            screen: ScreenType::DefaultScreen,
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
        self.cursor.restore_position()
    }

    pub(super) fn enable_raw_screen(&mut self) -> Result<()> {
        self.screen = ScreenType::RawScreenType(RawScreen::into_raw_mode().unwrap());
        Ok(())
    }

    pub(super) fn enable_alternate_screen(&mut self) -> Result<()> {
        self.screen = ScreenType::AlternateScreenType(AlternateScreen::to_alternate(true).unwrap());
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
        self.cursor.move_left(self.cursor.pos().unwrap().0).unwrap();
    }

    pub(crate) fn size(&self) -> (u16, u16) {
        if let Ok(size) = self.terminal.size() {
            size
        } else {
            (0, 0)
        }
    }

    pub(crate) fn get_cursor_pos(&self) -> (u16, u16) {
        if let Ok(pos) = self.cursor.pos() {
            pos
        } else {
            (0, 0)
        }
    }

    pub(crate) fn scroll_up(&mut self, step: u16) -> Result<()> {
        self.terminal.scroll_up(step)?;
        self.cursor.move_up(step)?;

        Ok(())
    }

    pub(crate) fn backspace(&mut self) -> Result<()> {
        self.cursor.move_left(1)?;
        self.clear_line()?;

        Ok(())
    }
}
