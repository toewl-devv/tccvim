use crossterm::{cursor, execute, terminal, event::{read, Event, KeyCode}};
use std::{io::{self, Write}, vec};

fn main() -> io::Result<()> {

    terminal::enable_raw_mode()?;

    let mut lines: Vec<String> = vec![String::new()];

    // clear and move cursor to 0,0
    execute!(
        io::stdout(),
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0),
    )?;

    // create left column of ~
    let (_, rows) = terminal::size()?;
    for _ in 1..=rows {
        write!(io::stdout(), "~\r\n")?;
    }
    execute!(io::stdout(), cursor::MoveTo(0, 0))?;

    let mut cursor_row = 0;
    let mut cursor_col = 0;
    let mut mode = 0;

    loop {
        if let Event::Key(key) = read()? {
            match key.code {
                KeyCode::Char(c) => {
                    if mode == 0 {
                        match c {
                            'q' => {
                                execute!(
                                    io::stdout(),
                                    terminal::Clear(terminal::ClearType::All),
                                    cursor::MoveTo(0, 0)
                                )?;
                                break;
                            }
                            'j' => {
                                if cursor_row < lines.len()-1 {
                                    cursor_row += 1;
                                    cursor_col = cursor_col.min(lines[cursor_row].len());
                                }
                            }
                            'k' => {
                                if cursor_row > 0 {
                                    cursor_row -= 1;
                                    cursor_col = cursor_col.min(lines[cursor_row].len());
                                }
                            }
                            'h' => {
                                if cursor_col > 0 {
                                    cursor_col -= 1;
                                }
                            }
                            'l' => {
                                if cursor_col < lines[cursor_row].len() {
                                    cursor_col += 1;
                                }
                            }
                            _ => {}
                        }
                    } else if mode == 1 {
                        lines[cursor_row].insert(cursor_col, c);
                        // redraw whole line
                        execute!(io::stdout(), cursor::MoveTo(0, cursor_row as u16))?;
                        write!(io::stdout(), "{}", lines[cursor_row])?;
                        io::stdout().flush()?;
                        cursor_col += 1;
                    }
                }
                KeyCode::Esc => {
                    mode = 1 - mode
                }
                KeyCode::Enter => {
                    // basically need to do the opposite of backspace :/
                    write!(io::stdout(), "\r\n")?;
                    cursor_row += 1;
                    cursor_col = 0;
                    lines.insert(cursor_row, String::new());
                }
                KeyCode::Backspace => {
                    //handle on line or back a line
                    if mode == 1 {
                        if cursor_col > 0 {
                            // cursor should delete character behind it
                            lines[cursor_row].remove(cursor_col-1);
                            cursor_col -= 1;
                            execute!(io::stdout(),
                                    cursor::MoveTo(0, cursor_row as u16),
                                    terminal::Clear(terminal::ClearType::CurrentLine)
                                    )?;
                            write!(io::stdout(), "{}", lines[cursor_row])?;
                            io::stdout().flush()?;
                        } else {
                            if cursor_row > 0 {
                                let current_line = lines.remove(cursor_row);
                                cursor_col = lines[cursor_row - 1].len();
                                lines[cursor_row - 1].push_str(&current_line);
                                cursor_row -= 1;
                                // print entire text again ig
                                for i in 0..(rows as usize) {
                                    execute!(io::stdout(),
                                            cursor::MoveTo(0, i as u16),
                                            terminal::Clear(terminal::ClearType::CurrentLine))?;
                                    if i < lines.len() {
                                        write!(io::stdout(), "{}", lines[i])?;
                                    } else {
                                        write!(io::stdout(), "~")?;
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
            execute!(io::stdout(), cursor::MoveTo(cursor_col as u16, cursor_row as u16))?;
        }
    }
    
    terminal::disable_raw_mode()?;

    Ok(())
}
