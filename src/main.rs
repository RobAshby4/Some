use std::*;
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

fn get_text(buf: Vec<String>, start: i32, height: u16) -> Result<String, _> {
    let mut retbuf = String::new();
    for x in start..height as i32 {
        retbuf.push_str( &buf.get(x as usize).clone());
    }
    return Ok(retbuf);
}

fn main() -> Result<(), io::Error> {
    if atty::is(atty::Stream::Stdin) {
        println!("No input on stdin, closing");
        return Ok(());
    }

    let mut buf: Vec<String>= Vec::new();
    loop {
        let mut linebuf = String::new();
        match io::stdin().read_line(&mut linebuf) {
            Ok(siz) => {
                if siz == 0 {break};
                buf.push(linebuf);
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
        terminal.draw(|x| {
            let size = x.size();
            let block = Block::default()
                .title("output")
                .borders(Borders::ALL);
            let tempbuf = get_text(buf.clone(), 0, size.height);
            let para = Paragraph::new(tempbuf).block(block);
            x.render_widget(para, size);
        })?;
       
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
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
