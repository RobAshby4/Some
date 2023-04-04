use std::{env::args, fs, io};
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

fn draw_to_term(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, input: &Vec<String>, start: i32) {
    terminal.draw(|x| {
        let size = x.size();
        let block = Block::default()
            .title("output")
            .borders(Borders::ALL);
        let tempbuf = get_text(input.clone(), start, size.height);
        let para = Paragraph::new(tempbuf).block(block);
        x.render_widget(para, size);
    }).expect("image drawn to term");
}

fn parse_file(name: String, input: &mut Vec<String>) {
    match fs::read_to_string(name) {
        Ok(file) => {
            file.split('\n').for_each( |s| {
                let mut next_line = String::from(s.clone());
                next_line.push('\n');
                input.push(next_line);
            });
        },
        Err(_) => {
            panic!("file not found!");
        },
    };
}

fn main() -> Result<(), io::Error> {

    let mut input: Vec<String>= Vec::new();
    let mut start = 0;
    if atty::is(atty::Stream::Stdin) {      // if there is no stdin input then try to 
        if args().len() < 2 {               // read file from args
            println!("No file given or input on stdin, closing");
            return Ok(());
        } else if args().len() >= 2 {
            match args().nth(1) {
                Some(name) => parse_file(name, &mut input),
                None => {
                    println!("Failed to parse arg");
                    return Ok(());
                },
            };
        }
    } else {        // read stdin until empty 
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
    }

    // start tui
    enable_raw_mode()?;
    let mut stdout = io::stdout();

    execute!(
        stdout,
        EnterAlternateScreen,
        EnableMouseCapture
    )?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    // main loop of program, take input and adjust buffer shown in terminal by that ammount
    loop { 
        let dim = terminal.size().expect("Get terminal dimensions on redraw");
        draw_to_term(&mut terminal, &input, start);
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Char('j') => if start <= (input.len() as i32 - dim.height as i32) as i32 {start = start + 1},
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn get_text_test() {
        let mut bufvec: Vec<String> = Vec::new();
        
        // check for empty string 
        let new_str = get_text(bufvec.clone(), 0, 10);
        assert_eq!("", new_str);

        let mut vecstring = String::from("vec three\n");
        bufvec.insert(0, vecstring);
        vecstring = String::from("vec two\n");
        bufvec.insert(0, vecstring);
        vecstring = String::from("vec one\n");
        bufvec.insert(0, vecstring);
        
        // check for a few strings
        let new_str = get_text(bufvec.clone(), 0, 10);
        assert_eq!("vec one\nvec two\nvec three\n", new_str);
        
        // check for invisible strings
        let new_str = get_text(bufvec.clone(), 4, 10);
        assert_eq!("", new_str);
        
        let new_str = get_text(bufvec.clone(), 2, 10);
        assert_eq!("vec three\n", new_str);
    }
}
