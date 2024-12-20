


use crossterm::{
    execute,
    style::{Color, SetBackgroundColor, Print, ResetColor},
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, size},
    cursor::MoveTo,
};

use std::io::{stdout, Write};

fn main() -> std::io::Result<()> {
    let mut stdout = stdout();

    execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;

    loop {
        execute!(stdout, Clear(ClearType::All))?;

        let terminal_size = size()?;

        let width = terminal_size.0;
        let height = terminal_size.1;

        execute!(
            stdout,
            MoveTo(0, height-1),
            SetBackgroundColor(Color::White),
            Print(" ".repeat(width as usize)),
            ResetColor
        )?;

        stdout.flush()?;
        break;
    }



    Ok(())
        
}

