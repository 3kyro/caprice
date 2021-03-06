use crate::caprice_error::Result;
use crossterm::cursor::{self, MoveLeft, MoveTo, RestorePosition, SavePosition};
use crossterm::event;
use crossterm::event::Event;
use crossterm::terminal::{
    self, disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
    LeaveAlternateScreen,
};
use crossterm::{execute, ExecutableCommand};
use std::time::Duration;

use std::io::{stdout, Stdout, Write};

pub(super) struct TerminalManipulator {
    stdout: Stdout,
}

impl TerminalManipulator {
    pub(super) fn new() -> Self {
        TerminalManipulator { stdout: stdout() }
    }

    pub(super) fn next_key_event(&mut self) -> Result<Option<Event>> {
        if event::poll(Duration::from_millis(0))? {
            Ok(Some(event::read()?))
        } else {
            Ok(None)
        }
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
        enable_raw_mode()?;
        Ok(())
    }

    pub(super) fn enable_alternate_screen(&mut self) -> Result<()> {
        execute!(stdout(), EnterAlternateScreen)?;
        execute!(stdout(), MoveTo(0, 0))?;
        Ok(())
    }

    pub(super) fn disable_alternate_screen(&mut self) -> Result<()> {
        execute!(stdout(), LeaveAlternateScreen)?;
        execute!(stdout(), MoveTo(0, 0))?;
        Ok(())
    }

    pub(crate) fn disable_raw_screen(&self) -> Result<()> {
        disable_raw_mode()?;
        execute!(stdout(), LeaveAlternateScreen)?;
        Ok(())
    }

    pub(crate) fn flush(&mut self) -> Result<()> {
        self.stdout.flush()?;
        Ok(())
    }

    pub(crate) fn goto_beginning_of_line(&mut self) -> Result<()> {
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
        std::process::exit(0);
    }
}
