extern crate termion;
use std::io;
use std::io::Write;
use tui::Terminal;
use tui::backend::TermionBackend;
use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;
use tui::widgets::{Widget, Block, Borders,Wrap,Paragraph};
use tui::layout::{Alignment,Layout, Constraint, Direction};
use tui::style::{Style,Modifier};
use tui::text::{Spans,Span};

fn draw(inpt:&mut String)-> Result<(),io::Error>{
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    stdout = io::stdout().into_raw_mode().unwrap();
    write!(stdout,"{}",termion::clear::All);
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(80),
                    Constraint::Percentage(10)
                ].as_ref()
            )
            .split(f.size());
        let block = Block::default()
             .title("Block")
             .borders(Borders::ALL);
        f.render_widget(block, chunks[0]);
        let block = Block::default()
             .title("Block 2")
             .borders(Borders::ALL);
        f.render_widget(block, chunks[1]);
        let text = vec![
            Spans::from(vec![
                Span::raw("First"),
            ]),
        ];
        
        let p = Paragraph::new(format!("{}",inpt))
            .block(Block::default().title("Paragraph").borders(Borders::ALL))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });
        f.render_widget(p, chunks[2]);
    })
}

fn notmain(){
    let stdin = io::stdin();
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let mut inpt = String::new();
    write!(stdout,"{}",termion::clear::All);
    draw(&mut inpt);
    for c in stdin.keys() {
        //write!(stdout,"{}",termion::clear::CurrentLine);
        match c.unwrap() {
            Key::Ctrl('c') => break,
            Key::Char(c) => inpt.push(c),
            Key::Backspace => {inpt.pop();},
            _ =>continue,}
        draw(&mut inpt);
        };
}

