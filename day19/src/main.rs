extern crate pancurses;

use std::env;
use std::cmp;
use std::fs::File;
use std::io::prelude::*;

use pancurses::Input;

#[derive(Clone)]
struct IntCode {
    prog : Vec<i64>,
    input : Vec<i64>,
    pos : usize,
    ipos : usize,
    ccnt : usize,
    relbase: i64,
    finished: bool
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
            99 => {
                    ic.finished = true;
                    break;
                  }
            _ => panic!("Failed"),
        }
    }
    return None;
}

fn find_left_binary(py: usize, mut xmin: usize, mut xmax: usize, prog : &IntCode, cnt: &mut i64) -> usize {
    while xmax != xmin {
        let mut m = prog.clone();
        let px = (xmax + xmin) / 2;
        m.input.push(px as i64);
        m.input.push(py as i64);
        println!("Test: {:?}", m.input);
        let r = run_program(&mut m);
        if r.unwrap() == 1 {
            xmax = px;
        } else {
            xmin = px + 1;
        }
        *cnt += 1;
    }
    return xmax;
}

fn find_left_slow(py: usize, mut xmin: usize, mut xmax: usize, prog : &IntCode, cnt: &mut i64) -> usize {
    let mut px = xmin;
    loop {
        let mut m = prog.clone();
        m.input.push(px as i64);
        m.input.push(py as i64);
        println!("Test: {:?}", m.input);
        let r = run_program(&mut m);
        *cnt += 1;
        if r.unwrap() == 1 {
            return px;
        }
        px += 1;
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
    let mut m = IntCode { prog: memory.to_vec(), input: vec![ ], pos: 0, ipos: 0, ccnt: 0, relbase: 0, finished: false };
    let backup = m.clone();
    let mut px = 0;
    let mut py = 0;
    let mut ymin : usize = 100;
    let mut ymax : usize = 1000;
    let mut cnt = 0;

    // determine the lower bound angle
    let p10 = find_left_slow(10, 0, 0, &backup, &mut cnt);
    let p100 = find_left_binary(100, p10 * 10 - 10, p10 * 10 + 10, &backup, &mut cnt);
    let p1000 = find_left_slow(1000, p100 * 10 - 10, p100 * 10 + 10, &backup, &mut cnt);
    println!("@1000: {} cnt: {}", p1000, cnt);

    while ymax != ymin {
        py = (ymax + ymin) / 2;
        px = (p1000 * py) / 1000;
        find_left_binary(py, px - 10, px + 10, &backup, &mut cnt);        
        m.input.push(px as i64 + 99);
        m.input.push(py as i64 - 99);
        println!("Test: {:?}", m.input);
        let q = run_program(&mut m);
        m = backup.clone();
        cnt += 1;
        println!("y: {} x: {} cnt: {} min: {} max: {} status: {}", py, px, cnt, ymin, ymax, q.unwrap());
        if q.unwrap() == 1 {
            ymax = py;
        } else {
            ymin = py + 1;
        }
    }
    println!("Min: {}x{}", px, py - 100); 
    Ok(())
}

fn main1() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut buf = String::new();
    let mut file = File::open(&args[1])?;
    file.read_to_string(&mut buf)?;
    let split = buf.trim().split(",");
    let mut memory : Vec<_> = split.filter_map(|x| x.parse::<i64>().ok()).collect();
    memory.resize(10000000, 0);
    let mut m = IntCode { prog: memory.to_vec(), input: vec![ ], pos: 0, ipos: 0, ccnt: 0, relbase: 0, finished: false };
    let backup = m.clone();
    let window = pancurses::initscr();
    pancurses::nl();
    pancurses::noecho();
    pancurses::curs_set(0);
    window.timeout(0);
    window.keypad(true);
    window.nodelay(true);
    let mw = window.get_max_x() as usize;
    let mut px : usize = 0;
    let mut py : usize = 100;
    let mut cnt = 0;
    let pc = vec!['-', '\\', '|', '/' ];
    let mc = vec!['.', '#'];
    let mut pp = 0;
    let mut left = 0;
    let mut lh = vec![0; 100];
    let mut right = 0;
    let mut rh = vec![0; 100];
    let mut hp = 0;
    let mut state = 0;
    // m.prog[0] = 2;
    let mut refresh = false;
    let mut sqm = 0;
    let mut sqx = 0;
    let mut sqy = 0;
    let mut exc = 0;

    loop {
        if refresh {
            window.refresh();
            pancurses::napms(10);
            refresh = false;
        }
        match window.getch() {
            Some(Input::Character('q')) => {
                pancurses::curs_set(1);
                pancurses::endwin();
                return Ok(());
            }
            _ => {}
        }
        // top 1 + 2-2-2-3 / 2-2-2-2-3 : 9/4 .. 11/5
        // bottom 1 + 2-2-1 / 5/3
        if sqm < 100 {
            window.mvaddch(0, 0, pc[pp]); pp = (pp + 1) % pc.len();
            if m.finished { m = backup.clone(); }
            m.input.push(px as i64);
            m.input.push(py as i64);
            //window.mvaddstr(exc % 50, 50, format!("{}: {}x{}", exc, px, py));
        }
        loop {
            let r = run_program(&mut m);
            if r.is_none() { break; }
            exc += 1;
            if py % 20 == 0 && r.unwrap() != 0 {
                let zx = px as i32 / 20;
                let zy = py as i32 / 20;
                window.mvaddch(zy + 1, zx + 1, mc[r.unwrap() as usize]);
            }
            cnt += r.unwrap();
            if state == 0 && r.unwrap() == 1 {
                // found left edge, skip to right side
                if left < px { left = px; }
                if right > px { let d = right - px; cnt += d as i64; px = right; }
                state = 1;
                while hp < rh.len() && rh[hp] < left { hp += 1; }
                for hi in hp..rh.len() {
                    let sh = rh.len() - hi + 1;
                    let sw = rh[hi] - left + 1;
                    let sq = if sh < sw { sh } else { sw };
                    if sq > sqm && sqm < 100 { 
                        sqy = hi;
                        sqx = left;
                        sqm = sq; 
                    }
                }
            } else if state == 1 && r.unwrap() == 0 {
                // found right edge, skip to next line
                right = px - 1;
                lh.push(left);
                rh.push(right);
                px = left;
                state = 0;
                //window.mvaddstr(0, 2, format!("{}", cnt));
                window.mvaddstr(0, 2, format!("@{} left={} right={} hp={} max={}@{}x{} exc={}",py,left,right,hp,sqm,sqx,sqy,exc));
                refresh = true;
                py += 1;
            } else {
                // just advance...
                px += 1;
            }
        }
    }
}
