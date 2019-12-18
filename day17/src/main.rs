extern crate pancurses;

use std::env;
use std::fs::File;
use std::io::prelude::*;

use pancurses::Input;

struct IntCode {
    prog : Vec<i64>,
    input : Vec<i64>,
    pos : usize,
    ipos : usize,
    ccnt : usize,
    relbase: i64,
}

struct Op3 {
    v1: i64,
    v2: i64,
    v3: i64
}

fn read3(v : &Vec<i64>, mode: i64, ic: &IntCode) -> Op3 {
    let mut op = Op3 { v1: 0, v2: 0, v3: 0 };
    let mode1 = mode % 10;
    let mode2 = (mode / 10) % 10;
    let mode3 = (mode / 100) % 10;
    op.v1 = v[ic.pos+1]; 
    op.v2 = v[ic.pos+2];
    op.v3 = v[ic.pos+3];
    //println!("@{} opcode {} mode {} : {}, {}, {}", pos, v[pos], mode, op.v1, op.v2, op.v3);
    if mode1 == 0 { op.v1 = v[op.v1 as usize]; }
    if mode1 == 2 { op.v1 = v[(op.v1 + ic.relbase) as usize]; }
    if mode2 == 0 { op.v2 = v[op.v2 as usize]; }
    if mode2 == 2 { op.v2 = v[(op.v2 + ic.relbase) as usize]; }
    if mode3 == 2 { op.v3 += ic.relbase; }
    return op;
} 

struct Op2 {
    v1: i64,
    v2: i64
}

fn read2(v : &Vec<i64>, mode: i64, ic: &IntCode) -> Op2 {
    let mut op = Op2 { v1: 0, v2: 0 };
    let mode1 = mode % 10;
    let mode2 = (mode / 10) % 10;
    op.v1 = v[ic.pos+1]; 
    op.v2 = v[ic.pos+2];
    //println!("@{} opcode {} mode {} : {}, {}", pos, v[pos], mode, op.v1, op.v2);
    if mode1 == 0 { op.v1 = v[op.v1 as usize]; }
    if mode1 == 2 { op.v1 = v[(op.v1 + ic.relbase) as usize]; }
    if mode2 == 0 { op.v2 = v[op.v2 as usize]; }
    if mode2 == 2 { op.v2 = v[(op.v2 + ic.relbase) as usize]; }
    return op;
} 

fn read1(v : &Vec<i64>, mode: i64, ic: &IntCode) -> i64 {
    let mut v1 = v[ic.pos+1]; 
    //println!("@{} opcode {} mode {} : {}", pos, v[pos], mode, v1);
    if mode == 0 { v1 = v[v1 as usize]; }
    if mode == 2 { v1 = v[(v1 + ic.relbase) as usize]; }
    return v1;
} 

fn run_program(ic: & mut IntCode) -> Option<i64> {
    loop {
        ic.ccnt += 1;
        let opcode = ic.prog[ic.pos] % 100;
        let mode = ic.prog[ic.pos] / 100;
        match opcode {
            1 => { 
                    let op = read3(&ic.prog, mode, ic);
                    ic.prog[op.v3 as usize]= op.v1 + op.v2;
                    //println!("m[{}] = {} + {} => {}",op.v3,op.v1,op.v2,prog[op.v3 as usize]);
                    ic.pos += 4;
                }, 
            2 => { 
                    let op = read3(&ic.prog, mode, ic);
                    ic.prog[op.v3 as usize] = op.v1 * op.v2;
                    //println!("m[{}] = {} * {} => {}",op.v3,op.v1,op.v2,prog[op.v3 as usize]);
                    ic.pos += 4;
                }, 
            3 => { 
                    if ic.ipos >= ic.input.len() {
                        return None;
                    }
                    let mut v = ic.prog[ic.pos + 1]; 
                    if mode == 2 { v += ic.relbase; }
                    ic.prog[v as usize] = ic.input[ic.ipos];
                    //println!("Input: {} => {} (mode {})", v, ic.prog[v as usize], mode);
                    ic.ipos += 1;
                    ic.pos += 2;
                 }, 
            4 => { 
                    let v = read1(&ic.prog, mode, ic); 
                    //println!("Output: {}", v);
                    ic.pos += 2;
                    return Some(v);
                 }, 
            5 => {  
                    let op = read2(&ic.prog, mode, ic); 
                    ic.pos += 3;
                    if op.v1 != 0 { ic.pos = op.v2 as usize; }
                    //println!("@ = {}? => {}",op.v1,op.v2);
                 }, 
            6 => {  
                    let op = read2(&ic.prog, mode, ic); 
                    ic.pos += 3;
                    if op.v1 == 0 { ic.pos = op.v2 as usize; }
                    //println!("@ = !{}? => {}",op.v1,op.v2);
                  }, 
            7 => {  
                    let op = read3(&ic.prog, mode, ic); 
                    ic.prog[op.v3 as usize] = if op.v1 < op.v2 { 1 } else { 0 };
                    //println!("m[{}] = {} < {} => {}",op.v3,op.v1,op.v2,prog[op.v3 as usize]);
                    ic.pos += 4;
                 }, 
            8 => {  
                    let op = read3(&ic.prog, mode, ic); 
                    ic.prog[op.v3 as usize] = if op.v1 == op.v2 { 1 } else { 0 };
                    //println!("m[{}] = {} == {} => {}",op.v3,op.v1,op.v2,prog[op.v3 as usize]);
                    ic.pos += 4;
                  }, 
            9 => { 
                    let v = read1(&ic.prog, mode, ic); 
                    ic.relbase += v;
                    ic.pos += 2;
                 }, 
            99 => break,
            _ => panic!("Failed"),
        }
    }
    return None;
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut buf = String::new();
    let mut file = File::open(&args[1])?;
    file.read_to_string(&mut buf)?;
    let split = buf.trim().split(",");
    let mut memory : Vec<_> = split.filter_map(|x| x.parse::<i64>().ok()).collect();
    memory.resize(10000000, 0);
    let mut m = IntCode { prog: memory.to_vec(), input: vec![ ], pos: 0, ipos: 0, ccnt: 0, relbase: 0 };
    let window = pancurses::initscr();
    pancurses::nl();
    pancurses::noecho();
    pancurses::curs_set(0);
    window.timeout(0);
    window.keypad(true);
    window.nodelay(true);
    let mw = window.get_max_x() as usize;
    let mut px : i32 = 0;
    let mut py : i32 = 0;
    let mut pb = 0;
    let mut scr = vec![0; mw * mw];
    let mut cnt = 0;
    let mut xdb = 0;
    let mut tot = 0;
    let pc = vec!['-', '\\', '|', '/' ];
    let mut pp = 0;
    m.prog[0] = 2;
    loop {
        window.refresh();
        let mut dir = 0;
        match window.getch() {
            Some(Input::Character('q')) => {
                pancurses::curs_set(1);
                pancurses::endwin();
                return Ok(());
            }
            Some(Input::Character(',')) => dir = b',',
            Some(Input::Character('r')) => dir = b'R',
            Some(Input::Character('l')) => dir = b'L', 
            Some(Input::Character('y')) => dir = b'Y',
            Some(Input::Character('n')) => dir = b'N', 
            Some(Input::Character(q)) if q >= '0' && q <= '9'  => dir = q as u8,
            Some(Input::Character(p)) if p >= 'a' && p <= 'c' => dir = p as u8 + b'A' - b'a',
            Some(Input::Character('$')) => dir = 10,
            Some(Input::KeyEnter) => dir = 10,
            _ => {}
        }
        pancurses::napms(10);
        if dir != 0 {
            if dir == 10 {
                window.mvaddch(py - 1, pb + xdb, '$'); 
                xdb = 0;
            } else {
                window.mvaddch(py - 1, pb + xdb, dir as char);
                xdb += 1;
            }
            //if dir == b'Y' || dir == b'N'
            m.input.push(dir as i64);
        }
        loop {
            let r = run_program(&mut m);
            if r.is_none() { break; }
            window.mvaddch(0, 0, pc[pp]); pp = (pp + 1) % pc.len();
            if r.unwrap() == 10 {
                pb = px + 1; px = 0; py += 1;
            } else if r.unwrap() > 128 {
                window.mvaddstr(0, 56, format!("Non-ascii output: {}", r.unwrap()));
            } else {
                let ch = r.unwrap() as u8;
                // wraparound screen.
                if px == 0 && py > 53 && (ch == b'#' || ch == b'.') {
                    window.refresh();
                    pancurses::napms(20);
                    py = 0;
                    cnt = 0; 
                    tot = 0;
                }
                scr[py as usize * mw + px as usize] = ch;
                window.mvaddch(py, px, ch as char);
                if ch == b'#' && px > 0 && py > 1 {
                    let y = py - 1;
                    let x = px;
                    if scr[y as usize * mw + x as usize] == b'#' &&
                    scr[(y+1) as usize * mw + x as usize] == b'#' &&
                    scr[(y-1) as usize * mw + x as usize] == b'#' &&
                    scr[y as usize * mw + (x+1) as usize] == b'#' &&
                    scr[y as usize * mw + (x-1) as usize] == b'#' {
                     window.mvaddch(y, x, 'O');
                     tot += x * y; cnt += 1;
                     window.mvaddstr(cnt, pb, format!("{}: {}x{} align {} tot {}", cnt, x, y, x*y, tot));
                    }
                }
                px += 1;
            }
        }
    }
}
