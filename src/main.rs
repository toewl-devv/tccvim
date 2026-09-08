use crossterm::{cursor, execute, terminal, event::{read, Event, KeyCode}};
use std::{io::{self, Write}};

fn main() -> io::Result<()> {

    terminal::enable_raw_mode()?;

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
    
    let mut text =  "".to_string();
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
                            'j' => execute!(io::stdout(), cursor::MoveDown(1))?,
                            'k' => execute!(io::stdout(), cursor::MoveUp(1))?,
                            'h' => execute!(io::stdout(), cursor::MoveLeft(1))?,
                            'l' => execute!(io::stdout(), cursor::MoveRight(1))?,
                            _ => {}
                        }
                    } else if mode == 1 {
                        text.push(c);
                        write!(io::stdout(), "{c}")?;
                        execute!(io::stdout())?;
                    }
                }
                KeyCode::Esc => {
                    mode = 1 - mode
                }
                KeyCode::Enter => write!(io::stdout(), "\r\n")?,
                KeyCode::Backspace => {
                    if mode == 1 {
                        execute!(io::stdout(), cursor::MoveLeft(1))?;
                        write!(io::stdout(), " ")?;
                        execute!(io::stdout(), cursor::MoveLeft(1))?;
                    }
                }
                _ => {}
            }
        }
    }
    
    terminal::disable_raw_mode()?;

    Ok(())
}
