extern crate termion;
extern crate clipboard;
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;
use std::io::Read;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Wrap,Paragraph,Widget, Block, Borders,List,ListItem,ListState};
use tui::layout::{Layout, Alignment, Constraint, Direction};
use tui::style::{Style,Modifier,Color};
use tui::text::{Spans,Span};
use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;

fn render_gui_accgen(values:&[String;3],i:usize,options:[bool;5]) -> Result<(),std::io::Error>{
    let mut stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    stdout = io::stdout().into_raw_mode()?;
    write!(stdout,"{}",termion::clear::All);
    terminal.draw(|f| {
        let size = f.size();
        let text = vec![
            Spans::from(vec![
                Span::raw("First"),
            ]),
        ];
        let mut markers:[&str;10] = ["  ";10];
        markers[i] = ">>";
        let mut opstr:[&str;5] = [" ";5];
        for x in 0..5{
            if options[x]{opstr[x]="/";}
        }
        let mut indents:[String;3]=[EMP;3];
        for x in 0..3{
            let mut y;
            if (values[x].len()>25){y=0}else{y=25-values[x].len()};
            indents[x] = String::from_utf8(vec![32;y]).unwrap();
        }
        let inpt = "-".to_owned() + markers[0]+ "Service:  [" + &values[0]+ &indents[0]+"]\n\n-" + markers[1] + "Username: [" + &values[1] +&indents[1]+ "]\n\n\r-" + markers[2] +"Password: [" + &values[2] + &indents[2]+ "]"
            + "\n\n-" +markers[3] + "48-57 - numbers:        [" + opstr[0] + "]"
            + "\n\n-" +markers[4] + "58-64 - special 1:      [" + opstr[1] + "]"
            + "\n\n-" +markers[5] + "65-90 - upper case:     [" + opstr[2] + "]"
            + "\n\n-" +markers[6] + "91-96 - special 2:      [" + opstr[3] + "]"
            + "\n\n-" +markers[7] + "97-122 - lower case:    [" + opstr[4] + "]"
            + "\n\n-" +markers[8] + "       [generate]"
            + "\n\n-" +markers[9] + "       [  done  ]";
        let p = Paragraph::new(format!("{}",inpt))
            .block(Block::default().title("Paragraph").borders(Borders::ALL))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });
        f.render_widget(p, size);
    })
}

fn render_gui_main(num:usize,v:&Vec<[String;3]>,open:&Vec<bool>)-> Result<(),std::io::Error> {
    let mut stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    stdout = io::stdout().into_raw_mode()?;
    write!(stdout,"{}",termion::clear::All);
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
    let mut create = false;
    let stdin = io::stdin();
    let mut open = vec![false;v.len()];
    render_gui_main(i as usize, v,&open);
    let mut vals:[String;3] = [EMP;3];
    let mut options:[bool;5] = [true;5];
    for c in stdin.keys() {
        //write!(stdout,"{}",termion::clear::CurrentLine);
        if create{
            match c.unwrap(){
                Key::Esc => {create=false;i=0;open=vec![false;v.len()];},
                Key::Char(' ') => {
                    match i{
                        9 => {
                            //save data to file
                            let mut outf = fs::OpenOptions::new().append(true).open(".pass").unwrap();
                            for valindx in 0..3{
                                let bout = vernam(key,&vals[valindx]);
                                let bout = bout.as_bytes();
                                let mut byt:[u8;50]=[32;50];
                                for x in 0..bout.len(){
                                    byt[x]=bout[x];
                                }
                                outf.write(&byt);
                            }
                            create=false;
                            i=0;
                            open=vec![false;v.len()];
                        },
                        8 => {
                            //generate password
                            let mut tpass=Vec::new();
                            let mut r = rand::thread_rng();
                            while tpass.len() < 40{
                                let x = r.gen_range(48..122);
                                match x{
                                    48..=57 => if options[0]{tpass.push(x);},
                                    58..=64 => if options[1]{tpass.push(x);},
                                    65..=90 => if options[2]{tpass.push(x);},
                                    91..=96 => if options[3]{tpass.push(x);},
                                    97..=122=> if options[4]{tpass.push(x);}
                                    _ => panic!("{} is outside of 48-122",x)
                                }
                            }
                            vals[2] = String::from_utf8(tpass).unwrap();
                        },
                        3..=7 => options[i as usize -3] = !options[i as usize -3],
                        0..=2 => vals[i as usize].push(' '),
                        _ => panic!("wtf")}
                },
                Key::Ctrl('c')=>break,
                Key::Char('\t') => if i<9{i+=1;},
                Key::BackTab => if i>=1{i-=1;},
                Key::Char(c) => if i<3{ vals[i as usize].push(c)}, 
                Key::Backspace => if i<3{vals[i as usize].pop();}
                _=>continue};
        }else{
            match c.unwrap() {
                Key::Char('n') => {create=true;i=0;vals=[EMP;3];options=[true;5];},
                Key::Char(' ') => open[i as usize % v.len()]=!open[i as usize % v.len()],
                Key::Char('q') => break,
                Key::Up => {i-=1;
                    if i<0{i=0;}
                },
                Key::Down => i+=1,
                Key::Char('c') => {
                    let mut ctx:ClipboardContext = ClipboardProvider::new().unwrap();
                    ctx.set_contents(v[(v.len() - 1) - ((i as usize) % v.len())][2].to_owned());
                },
                _ =>continue};
        }
        if create{
            render_gui_accgen(&vals,i as usize,options);
        }else{
            render_gui_main(i as usize, v,&open);
        }
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
