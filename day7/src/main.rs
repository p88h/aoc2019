extern crate itertools; // 0.7.8
extern crate permutohedron;

use permutohedron::LexicalPermutation;
use std::convert::TryInto;
use std::io;

struct Op3 {
    v1: i32,
    v2: i32,
    v3: i32
}

fn read3(v : &Vec<i32>, mode: i32, pos: usize) -> Op3 {
    let mut op = Op3 { v1: 0, v2: 0, v3: 0 };
    let mode1 = mode % 10;
    let mode2 = (mode / 10) % 10;
    let _mode3 = (mode / 100) % 10;
    op.v1 = v[pos+1]; 
    op.v2 = v[pos+2];
    op.v3 = v[pos+3];
    //println!("@{} opcode {} mode {} : {}, {}, {}", pos, v[pos], mode, op.v1, op.v2, op.v3);
    if mode1 == 0 { op.v1 = v[op.v1 as usize]; }
    if mode2 == 0 { op.v2 = v[op.v2 as usize]; }
    //if mode3 == 0 { op.v3 = v[op.v3 as usize]; }
    return op;
} 

struct Op2 {
    v1: i32,
    v2: i32
}

fn read2(v : &Vec<i32>, mode: i32, pos: usize) -> Op2 {
    let mut op = Op2 { v1: 0, v2: 0 };
    let mode1 = mode % 10;
    let mode2 = (mode / 10) % 10;
    op.v1 = v[pos+1]; 
    op.v2 = v[pos+2];
    //println!("@{} opcode {} mode {} : {}, {}", pos, v[pos], mode, op.v1, op.v2);
    if mode1 == 0 { op.v1 = v[op.v1 as usize]; }
    if mode2 == 0 { op.v2 = v[op.v2 as usize]; }
    return op;
} 

fn read1(v : &Vec<i32>, mode: i32, pos: usize) -> i32 {
    let mut v1 = v[pos+1].try_into().unwrap(); 
    //println!("@{} opcode {} mode {} : {}", pos, v[pos], mode, v1);
    if mode == 0 { v1 = v[v1 as usize]; }
    return v1;
} 

struct IntCode {
    prog : Vec<i32>,
    input : Vec<i32>,
    pos : usize,
    ipos : usize,
    ccnt : usize,
}

fn run_program(ic: & mut IntCode) -> Option<i32> {
    loop {
        ic.ccnt += 1;
        let opcode = ic.prog[ic.pos] % 100;
        let mode = ic.prog[ic.pos] / 100;
        match opcode {
            1 => { 
                    let op = read3(&ic.prog, mode, ic.pos);
                    ic.prog[op.v3 as usize]= op.v1 + op.v2;
                    //println!("m[{}] = {} + {} => {}",op.v3,op.v1,op.v2,prog[op.v3 as usize]);
                    ic.pos += 4;
                }, 
            2 => { 
                    let op = read3(&ic.prog, mode, ic.pos);
                    ic.prog[op.v3 as usize] = op.v1 * op.v2;
                    //println!("m[{}] = {} * {} => {}",op.v3,op.v1,op.v2,prog[op.v3 as usize]);
                    ic.pos += 4;
                }, 
            3 => { 
                    let v = ic.prog[ic.pos + 1]; 
                    ic.prog[v as usize] = ic.input[ic.ipos];
                    //println!("Input: {} => {} (mode {})", v, prog[v as usize], mode);
                    ic.ipos += 1;
                    ic.pos += 2;
                 }, 
            4 => { 
                    let v = read1(&ic.prog, mode, ic.pos); 
                    //println!("Output: {}", v);
                    ic.pos += 2;
                    return Some(v);
                 }, 
            5 => {  
                    let op = read2(&ic.prog, mode, ic.pos); 
                    ic.pos += 3;
                    if op.v1 != 0 { ic.pos = op.v2 as usize; }
                    //println!("@ = {}? => {}",op.v1,op.v2);
                 }, 
            6 => {  
                    let op = read2(&ic.prog, mode, ic.pos); 
                    ic.pos += 3;
                    if op.v1 == 0 { ic.pos = op.v2 as usize; }
                    //println!("@ = !{}? => {}",op.v1,op.v2);
                  }, 
            7 => {  
                    let op = read3(&ic.prog, mode, ic.pos); 
                    ic.prog[op.v3 as usize] = if op.v1 < op.v2 { 1 } else { 0 };
                    //println!("m[{}] = {} < {} => {}",op.v3,op.v1,op.v2,prog[op.v3 as usize]);
                    ic.pos += 4;
                 }, 
            8 => {  
                    let op = read3(&ic.prog, mode, ic.pos); 
                    ic.prog[op.v3 as usize] = if op.v1 == op.v2 { 1 } else { 0 };
                    //println!("m[{}] = {} == {} => {}",op.v3,op.v1,op.v2,prog[op.v3 as usize]);
                    ic.pos += 4;
                  }, 
            99 => break,
            _ => panic!("Failed"),
        }
    }
    return None;
}

fn run_chained(prog: Vec<i32>, params: Vec<i32>) -> i32 {
    let mut machs : Vec<IntCode> = vec![];
    for p in params.clone() {
        let m = IntCode { prog: prog.clone(), input: vec![ p ], pos: 0, ipos: 0, ccnt: 0 };
        machs.push(m);
    }
    machs[0].input.push(0);
    let mut mp = 0;
    let mut icnt = 0;
    loop {
        icnt += 1;
        let poutput = run_program(&mut machs[mp]);
        if poutput.is_none() { break; }
        mp = (mp + 1) % machs.len();
        machs[mp].input.push(poutput.unwrap());
    }
    let output = machs[mp].input.pop().unwrap();
    let mut ccnt = 0;
    for m in machs {
        ccnt += m.ccnt;
    }
    println!("params: {:?} output: {} icnt: {}, ccnt: {}", params, output, icnt, ccnt);
    return output;
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Failure");
    let split = buf.trim().split(",");
    let memory : Vec<_> = split.filter_map(|x| x.parse::<i32>().ok()).collect();
    let prog = memory.to_vec();
    let mut list = vec![ 5, 6, 7, 8, 9 ];
    let mut max = 0;
    loop {
        let output = run_chained(prog.clone(), list.clone());
        if output > max { max = output; }
        if !list.next_permutation() { break; }
    }
    println!("Maximum throttle: {}", max);
}
