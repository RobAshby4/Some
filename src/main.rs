use std::{env, fs, io};
use tui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph},
    //layout::{Layout, Constraint, Direction},
    Terminal,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

fn get_text(bufvec: Vec<String>, start: i32, height: u16) -> String {
    if start > bufvec.len() as i32 {
        return String::from("");
    }
    let mut retbuf = String::new();
    for x in start..(start + height as i32) {
        retbuf.push_str( 
            match bufvec.get(x as usize).clone() {
                Some(buf) => buf,
                None => "",
            });
    }
    return retbuf;
}

fn main() -> Result<(), io::Error> {
    if atty::is(atty::Stream::Stdin) {
        if std::env::args().len() < 2 {
            println!("No file given or input on stdin, closing");
            return Ok(());
        } else if std::env::args().len() >= 2 {
            
        }
    }

    let mut input: Vec<String>= Vec::new();
    let mut start = 0;
    loop {
        let mut linebuf = String::new();
        match io::stdin().read_line(&mut linebuf) {
            Ok(siz) => {
                if siz == 0 {break};
                input.push(linebuf);
            },
            Err(_) => break,
        };
    }
    
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    
    execute!(
        stdout,
        EnterAlternateScreen,
        EnableMouseCapture
    )?;
     
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        let dim = terminal.size().unwrap();
        terminal.draw(|x| {
            let size = x.size();
            let block = Block::default()
                .title("output")
                .borders(Borders::ALL);
            let tempbuf = get_text(input.clone(), start, size.height);
            let para = Paragraph::new(tempbuf).block(block);
            x.render_widget(para, size);
        })?;
       
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Char('j') => if start <= 2 + (input.len() as u16 - dim.height) as i32 {start = start + 1},
                KeyCode::Char('k') => if start >= 1 {start = start - 1},
                _   => {},
                
            }
        }
    }
    
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    terminal.show_cursor()?;

    return Ok(());
}
