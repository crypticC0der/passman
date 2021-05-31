extern crate termion;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;
use std::io::Read;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Block, Borders,List,ListItem,ListState};
use tui::layout::{Layout, Constraint, Direction};
use tui::style::{Style,Modifier,Color};
use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;

fn draw(num:usize,v:&Vec<[String;3]>,open:&Vec<bool>)-> Result<(),std::io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.draw(|f| {
        let size = f.size();
        let mut items=Vec::new();
        let mut t = v.clone();
        for x in 0..v.len(){
            let item = t.pop().unwrap();
            let mut out = item[0].clone();
            out = out + " | " + &item[1] + " | ";
            if open[x]{
                out = out + &item[2];
            }else{
                out = out + &String::from_utf8(vec![42;item[2].len()]).unwrap();
            }
            items.push(ListItem::new(out));
            
        }
        let block = Block::default()
            .title("Block")
            .borders(Borders::ALL);
        let num = num % items.len();
        let l = List::new(items)
            .block(Block::default().title("passman").borders(Borders::ALL))
            .style(Style::default().fg(Color::Green))
            .highlight_style(Style::default().fg(Color::Rgb(0,0,0)).bg(Color::Green).add_modifier(Modifier::BOLD))
            .highlight_symbol(">>");
        let mut state:ListState =  ListState::default();
        state.select(Some(num));
        f.render_stateful_widget(l,size,&mut state);
    })
}

fn temp(){
    let mut outf = fs::File::create(".pass").expect("fucky wucky with file");
    let mut bout:[u8;10] = [32;10];
    let mut rng = rand::thread_rng();
    for j in 0..18{
        for i in 0..10{
            bout[i] = rng.gen_range(48..57);
        }
        let bout=vernam("loriSmells", &String::from_utf8(bout.to_vec()).unwrap());
        let bout = bout.as_bytes();
        let mut byt:[u8;50]=[32;50];
        for x in 0..10{
            byt[x] = bout[x];
        }
        outf.write(&byt);
    }    
}

fn comp(key:&str, v:&Vec<[String;3]>){
    let mut i:isize= 0;
    let stdin = io::stdin();
    let mut open = vec![false;v.len()];
    draw(i as usize, v,&open);
    for c in stdin.keys() {
        //write!(stdout,"{}",termion::clear::CurrentLine);
        match c.unwrap() {
            Key::Char(' ') => open[i as usize]=!open[i as usize],
            Key::Char('q') => break,
            Key::Up => {i-=1;
                if i<0{i=0;}
            },
            Key::Down => i+=1,
            _ =>continue};
        draw(i as usize, v,&open);
    }
}

fn vernam(key:&str,val:&str) -> String{
    let mut ret = String::new();
    let lenk:usize = key.len();
    let lenv:usize= val.len();
    let mut i:usize = 0;
    let key=key.as_bytes();
    let val=val.as_bytes();
    while i<lenv{
        ret.push((key[i%lenk] ^ val[i]) as char);
        i+=1;
    }
    return ret;
}

const EMP:String = String::new();

fn main(){ 
    //temp();
    let mut v = Vec::new();
    let mut key=String::new(); 
    println!("what is the key");
    let stdin = io::stdin();
    stdin.read_line(&mut key).unwrap();
    let key = key.as_str().trim_end();
    let mut f = fs::File::open(".pass").expect("cant open");
    let mut byt:[u8;50]=[0;50];
    
    while f.read(&mut byt).unwrap()>0{
        let mut t:[String;3]= [EMP;3];
        t[0] = vernam(key,(&String::from_utf8(byt.to_vec()).unwrap()).trim_end());
        f.read(&mut byt);
        t[1] = vernam(key,(&String::from_utf8(byt.to_vec()).unwrap()).trim_end());
        f.read(&mut byt);
        t[2] = vernam(key,(&String::from_utf8(byt.to_vec()).unwrap()).trim_end());
        v.push(t);
    }

    let mut stdout = io::stdout().into_raw_mode().unwrap();
    write!(stdout,"{}",termion::clear::All);
    comp(key,&mut v);
}
