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

fn turn_right(dir: i32) -> i32 {
    // always turn right
    match dir {
        1 => return 4,
        2 => return 3,
        3 => return 1,
        4 => return 2,
        _ => return dir
    }
}

fn turn_left(dir: i32) -> i32 {
    // always turn right
    match dir {
        1 => return 3,
        2 => return 4,
        3 => return 2,
        4 => return 1,
        _ => return dir
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut buf = String::new();
    let mut file = File::open(&args[1])?;
    file.read_to_string(&mut buf)?;
    let split = buf.trim().split(",");
    let mut memory : Vec<_> = split.filter_map(|x| x.parse::<i64>().ok()).collect();
    memory.resize(10000000, 0);
    let mut m = IntCode { prog: memory.to_vec(), input: vec![ 0 ], pos: 0, ipos: 0, ccnt: 0, relbase: 0 };
    let window = pancurses::initscr();
    pancurses::nl();
    pancurses::noecho();
    pancurses::curs_set(0);
    window.timeout(0);
    window.keypad(true);
    window.nodelay(true);
    let ix = window.get_max_x() / 2;
    let iy = window.get_max_y() / 2;
    let mw = window.get_max_x() as usize;
    let mut px = ix;
    let mut py = iy;
    let mut turns = -1;
    let mut press = 0;
    let mut dir : i32 = 1;
    let mut dx : i32;
    let mut dy : i32;
    let mut dist = vec![0; mw * mw];
    let mut ov = vec![ 0, 0 ];
    let dv = vec![ 0, 0, -1, 0, 1, 0, 0, -1, 0, 1];
    dist[py as usize * mw + px as usize] = 1;
    window.mvaddch(py, px, 'B');
    loop {
        window.refresh();
        match window.getch() {
            Some(Input::Character('q')) => {
                pancurses::curs_set(1);
                pancurses::endwin();
                return Ok(());
            }
            Some(Input::Character('w')) => dir = 1,
            Some(Input::Character('s')) => dir = 2, 
            Some(Input::Character('a')) => dir = 3,
            Some(Input::Character('d')) => dir = 4,
            _ => {}
        }
        dy = dv[(dir * 2) as usize];
        dx = dv[(dir * 2 + 1) as usize];
        pancurses::napms(10);
        if turns > 0 {
            turns -= 1;
            window.mvaddch(ov[1], ov[0], 'O');
            if turns % 100 == 0 {
                window.mvaddstr(2, 0, format!("Pressurization in {} sec", turns / 100));
            }
        }
        if turns == 0 {
            dir = 0;
            dist[ov[0] as usize * mw + ov[1] as usize] = 0;
            let mut nv = Vec::new();
            while ov.len() > 0 {
                let oy = ov.pop().unwrap();
                let ox = ov.pop().unwrap();
                for d in 1..=4 {
                    let py = oy + dv[2 * d];
                    let px = ox + dv[2 * d + 1];
                    if dist[py as usize * mw + px as usize] > 0 {
                        dist[py as usize * mw + px as usize] = 0;
                        nv.push(px);
                        nv.push(py);
                        window.mvaddch(py, px, 'o');
                    }
                }
            }
            ov = nv; press += 1;
            window.mvaddstr(3, 0, format!("Pressurizing: {}", press));
            pancurses::napms(40);
            if ov.len() == 0 {
                turns = -1;
            }
        }
        if dir == 0 { continue; }
        m.ipos = 0;
        m.input[0] = dir as i64;
        loop {
            let r = run_program(&mut m);
            if r.is_none() { break; }
            if r.unwrap() == 0 {
                window.mvaddch(py + dy, px + dx, '#');
                dir = turn_left(dir);
            } else {
                let pd = dist[py as usize * mw + px as usize];
                window.mvaddch(py, px, '.');
                px += dx; py += dy;
                if dist[py as usize * mw + px as usize] == 0 {
                    dist[py as usize * mw + px as usize] = pd + 1;
                }
                window.mvaddch(py, px, if r.unwrap() == 1 { 'B' } else { 'O' }); 
                dir = turn_right(dir);
            }
            if r.unwrap() == 2 {
                ov[0] = px;
                ov[1] = py;
                window.mvaddstr(0, 0, format!("Oxygen tank found at: {}x{}", px - ix, py - iy));
                window.mvaddstr(1, 0, format!("Distance: {}", dist[py as usize * mw + px as usize] - 1));
                turns = 1000;
            }
        }
    }
}
