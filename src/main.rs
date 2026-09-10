use crossterm::{cursor, execute, terminal, event::{read, Event, KeyCode}};
use std::{io::{self, Write}, fs, env};

fn write_full(lines: Vec<String>, rows: u16) -> io::Result<()> {
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

    Ok(())
}

fn main() -> io::Result<()> {
    
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut lines = if fs::exists(file_path).unwrap() {
        let contents = fs::read_to_string(file_path)
            .expect("Should have been able to read the file");
        contents
            .split('\n')
            .map(String::from)
            .collect()
    } else {
        vec![String::new()]
    };

    terminal::enable_raw_mode()?;

    //let mut lines: Vec<String> = vec![String::new()];

    // clear and move cursor to 0,0
    execute!(
        io::stdout(),
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0),
    )?;
    
    /*
    // create left column of ~
    for _ in 1..=rows {
        write!(io::stdout(), "~\r\n")?;
    }
    */

    let (_, rows) = terminal::size()?;
    write_full(lines.clone(), rows)?;

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
                            'w' => {
                                fs::write(file_path, lines.join("\n"))?;
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
                    let (before, after) = lines[cursor_row].split_at(cursor_col);
                    let after = after.to_string();
                    let before = before.to_string();
                    lines.insert(cursor_row+1, after.to_string());
                    lines[cursor_row] = before;
                    cursor_row += 1;
                    cursor_col = 0;
                    // print entire text
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
