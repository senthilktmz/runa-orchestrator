use crossterm::{
    execute,
    style::{Color, SetBackgroundColor, Print, ResetColor},
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, size},
    cursor::MoveTo,
    event::{read, Event, KeyCode},
};
use std::io::{stdout, Write};

fn main() -> std::io::Result<()> {
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let mut text_buffer: Vec<String> = vec![String::new()];
    let mut cursor_position = (0, 0); // Cursor starts at the first line and first column

    loop {
        // Clear the screen
        execute!(stdout, Clear(ClearType::All))?;

        // Get terminal size
        let terminal_size = size()?;
        let width = terminal_size.0;
        let height = terminal_size.1;

        // Render the text buffer from top to bottom
        for (i, line) in text_buffer.iter().enumerate() {
            execute!(
                stdout,
                MoveTo(0, i as u16),
                Print(line)
            )?;
        }

        // Draw the white bar
        execute!(
            stdout,
            MoveTo(0, height - 1),
            SetBackgroundColor(Color::White),
            Print(" ".repeat(width as usize)),
            ResetColor
        )?;

        // Draw the cursor
        execute!(
            stdout,
            MoveTo(cursor_position.1 as u16, cursor_position.0 as u16)
        )?;

        stdout.flush()?;

        // Read user input
        if let Event::Key(key_event) = read()? {
            match key_event.code {
                KeyCode::Char(c) => {
                    // Add character at the cursor position
                    if let Some(line) = text_buffer.get_mut(cursor_position.0) {
                        line.insert(cursor_position.1, c);
                        cursor_position.1 += 1; // Move cursor to the right
                    }
                }
                KeyCode::Enter => {
                    // Split the line at the cursor position and move to a new line
                    if let Some(line) = text_buffer.get_mut(cursor_position.0) {
                        let new_line = line.split_off(cursor_position.1);
                        text_buffer.insert(cursor_position.0 + 1, new_line);
                    }
                    cursor_position.0 += 1; // Move to the next line
                    cursor_position.1 = 0; // Reset to the start of the line
                }
                _ => {}
            }
        }
    }

    // Cleanup
    execute!(stdout, LeaveAlternateScreen)?;
    Ok(())
}

