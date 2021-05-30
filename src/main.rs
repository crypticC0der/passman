extern crate termion;
use std::io;
use std::io::Write;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Block, Borders,List,ListItem,ListState};
use tui::layout::{Layout, Constraint, Direction};
use tui::style::{Style,Modifier,Color};
use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;

fn draw(num:usize)-> Result<(),std::io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default()
            .title("Block")
            .borders(Borders::ALL);
        let items = [ListItem::new("Item 1"), ListItem::new("Item 2"), ListItem::new("Item 3")];
        let num = num % items.len();
        let l = List::new(items)
            .block(Block::default().title("List").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>");
        let mut state:ListState =  ListState::default();
        state.select(Some(num));
        f.render_stateful_widget(l,size,&mut state);
    })
}

fn comp(){
    
}

fn vernam(key:&str,val:String) -> String{
    let mut ret = String::new();
    let lenk:usize = key.len();
    let lenv:usize= val.len();
    let mut i:usize = 0;
    let key=key.as_bytes();
    let val=val.as_bytes();
    while i<lenv{
        print!("one");
        ret.push((key[i%lenk] ^ val[i]) as char);
        i+=1;
    }
    return ret;
}


fn main(){ 
    let mut key=String::new(); 
    println!("what is the key");
    let stdin = io::stdin();
    stdin.read_line(&mut key).unwrap();
    let key = key.as_str().trim_end();
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    write!(stdout,"{}",termion::clear::All);
    draw(0);
    let mut i:isize= 0;
    for c in stdin.keys() {
        //write!(stdout,"{}",termion::clear::CurrentLine);
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Up => {i-=1;
                if i<0{i=0;}
            },
            Key::Down => i+=1,
            _ =>continue};
        draw(i as usize);
    }
}
