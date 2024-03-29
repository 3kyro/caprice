use crate::error::Result;
use crossterm::cursor::{self, MoveLeft, MoveTo, RestorePosition, SavePosition};
use crossterm::event;
use crossterm::event::Event;
use crossterm::terminal::{
    self, disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
    LeaveAlternateScreen,
};
use crossterm::{execute, ExecutableCommand};

use std::io::{stdout, Stdout, Write};

#[derive(Debug)]
pub(super) struct Terminal {
    pub(crate) stdout: Stdout,
    alternate_screen: AlternateScreen,
}

impl Terminal {
    pub(super) fn new() -> Self {
        Terminal {
            stdout: stdout(),
            alternate_screen: AlternateScreen::Disabled,
        }
    }

    pub(super) fn next_key_event(&mut self) -> Result<Event> {
        Ok(event::read()?)
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

    pub(super) fn enable_alternate_screen(&mut self) -> Result<()> {
        execute!(stdout(), EnterAlternateScreen)?;
        execute!(stdout(), MoveTo(0, 0))?;
        self.alternate_screen = AlternateScreen::Enabled;
        Ok(())
    }

    pub(super) fn disable_alternate_screen(&mut self) -> Result<()> {
        execute!(stdout(), LeaveAlternateScreen)?;
        self.alternate_screen = AlternateScreen::Disabled;
        Ok(())
    }

    pub(crate) fn enable_raw_mode(&self) -> Result<()> {
        enable_raw_mode()?;
        Ok(())
    }

    pub(crate) fn disable_raw_mode(&self) -> Result<()> {
        disable_raw_mode()?;
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

    pub(crate) fn exit(&mut self) {
        match self.alternate_screen {
            AlternateScreen::Enabled => self.disable_alternate_screen().unwrap(),
            AlternateScreen::Disabled => (),
        }
        self.disable_raw_mode().unwrap();
        std::process::exit(0);
    }
}

#[derive(Debug)]
enum AlternateScreen {
    Disabled,
    Enabled,
}
