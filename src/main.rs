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

fn main() -> Result<(), io::Error> {
    let mut buf = String::new();
    loop {
        match io::stdin().read_line(&mut buf) {
            Ok(siz) => if siz == 0 {break},
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
        let tempbuf = buf.clone();
        terminal.draw(|x| {
            let size = x.size();
            let block = Block::default()
                .title("output")
                .borders(Borders::ALL);
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
