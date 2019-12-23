use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone)]
struct IntCode {
    prog : Vec<i64>,
    input : Vec<i64>,
    pos : usize,
    ipos : usize,
    ccnt : usize,
    relbase: i64,
    waiting : bool,
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
                        ic.waiting = true;
                        return None;
                    }
                    ic.waiting = false;
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
    memory.resize(1000000, 0);
    let m = IntCode { prog: memory.to_vec(), input: vec![ ], pos: 0, ipos: 0, ccnt: 0, relbase: 0, waiting: false };
    let mut net = vec![];
    let mut msgs = vec![];
    let mut natp = (0, 0);
    let mut natm = (0, 0);
    for i in 0..50 {
        let mut mi = m.clone();
        mi.input.push(i);
        net.push(mi);
        msgs.push(vec![]);
    }
    loop {
        let mut idle = 0;
        for i in 0..50 {
            let mut mi = &mut net[i];
            // push all pending inputs...
            for j in 0..msgs[i].len() {
                let (ix, iy) = msgs[i][j];
                mi.input.push(ix);
                mi.input.push(iy);
            }
            msgs[i].clear();
            // empty input queue -- notify
            if mi.waiting && mi.ipos == mi.input.len() {
                mi.input.push(-1);
                idle += 1;
            }
            loop {
                let r = run_program(&mut mi);
                let x = run_program(&mut mi);
                let y = run_program(&mut mi);
                if r.is_none() {
                    // waiting or terminated
                    break;
                }
                if r.unwrap() == 255 {
                    println!("NAT message from {}: {} {}", i, x.unwrap(), y.unwrap());
                    natm = (x.unwrap(), y.unwrap());
                } else if r.unwrap() < 0 {
                    println!("invalid message from {} : {}", i, r.unwrap());
                } else {
                    println!("{}=>{} {}x{}",i,r.unwrap(),x.unwrap(),y.unwrap());
                    msgs[r.unwrap() as usize].push((x.unwrap(), y.unwrap()));
                }
            }
            if !mi.waiting {
                println!("instance {} is terminated", i);
            }
        }
        if idle == 50 {
            println!("Network is idle. transmit {}x{} to 0", natm.0, natm.1);
            if natp.1 == natm.1 {
                println!("Duplicate message detected");
                break;
            }
            natp = natm.clone();
            msgs[0].push(natm);
        }
    }
    Ok(())
}
