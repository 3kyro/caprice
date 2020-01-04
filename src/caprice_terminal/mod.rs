use anyhow::Result;
use crossterm::cursor::{self, MoveLeft, MoveTo, RestorePosition, SavePosition};
use crossterm::input::{input, AsyncReader, InputEvent};
use crossterm::screen::{AlternateScreen, RawScreen};
use crossterm::terminal::{self, Clear, ClearType};
use crossterm::{execute, ExecutableCommand};

use std::io::{stdout, Stdout, Write};

enum ScreenType {
    Raw(RawScreen),
    Alternate(AlternateScreen),
    Default,
}

pub(super) struct TerminalManipulator {
    stdin: AsyncReader,
    stdout: Stdout,
    screen: ScreenType,
}

impl TerminalManipulator {
    pub(super) fn new() -> Self {
        TerminalManipulator {
            stdin: input().read_async(),
            stdout: stdout(),
            screen: ScreenType::Default,
        }
    }

    pub(super) fn next_key_event(&mut self) -> Option<InputEvent> {
        self.stdin.next()
    }

    pub(super) fn clear_from_cursor(&self) -> Result<()> {
        stdout()
            .execute(Clear(ClearType::FromCursorDown))?
            .execute(Clear(ClearType::UntilNewLine))?;

        Ok(())
    }

    pub(super) fn goto_next_line(&self) -> Result<()> {
        self.clear_from_cursor()?;
        println!("\r");
        Ok(())
    }

    pub(super) fn clear_line(&self) -> Result<()> {
        execute!(stdout(), Clear(ClearType::UntilNewLine))?;
        Ok(())
    }

    pub(super) fn save_cursor(&self) -> Result<()> {
        execute!(stdout(), SavePosition)?;
        Ok(())
    }

    pub(super) fn restore_cursor(&self) -> Result<()> {
        execute!(stdout(), RestorePosition)?;
        Ok(())
    }

    pub(super) fn enable_raw_screen(&mut self) -> Result<()> {
        self.screen = ScreenType::Raw(RawScreen::into_raw_mode()?);
        Ok(())
    }

    pub(super) fn enable_alternate_screen(&mut self) -> Result<()> {
        self.screen = ScreenType::Alternate(AlternateScreen::to_alternate(true)?);
        execute!(stdout(), MoveTo(0, 0))?;
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

    pub(crate) fn goto_begining_of_line(&mut self) -> Result<()> {
        execute!(stdout(), MoveLeft(cursor::position()?.0))?;
        Ok(())
    }

    pub(crate) fn size(&self) -> (u16, u16) {
        if let Ok(size) = terminal::size() {
            size
        } else {
            (0, 0)
        }
    }

    pub(crate) fn get_cursor_pos(&self) -> (u16, u16) {
        if let Ok(pos) = cursor::position() {
            pos
        } else {
            (0, 0)
        }
    }

    pub(crate) fn scroll_up(&mut self, step: u16) -> Result<()> {
        stdout()
            .execute(terminal::ScrollUp(step))?
            .execute(cursor::MoveUp(step))?;
        Ok(())
    }

    pub(crate) fn backspace(&mut self) -> Result<()> {
        execute!(stdout(), cursor::MoveLeft(1))?;
        self.clear_line()?;
        Ok(())
    }

    pub(crate) fn exit(&self) {
        terminal::exit();
    }
}
