use std::io;

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
                    let mut v = ic.prog[ic.pos + 1]; 
                    if mode == 2 { v += ic.relbase; }
                    ic.prog[v as usize] = ic.input[ic.ipos];
                    //println!("Input: {} => {} (mode {})", v, prog[v as usize], mode);
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


fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Failure");
    let split = buf.trim().split(",");
    let mut memory : Vec<_> = split.filter_map(|x| x.parse::<i64>().ok()).collect();
    memory.resize(10000000, 0);
    let mut m = IntCode { prog: memory.to_vec(), input: vec![ 0 ], pos: 0, ipos: 0, ccnt: 0, relbase: 0 };
    let mut mm = vec![2; 1000000];
    let mut px = 500;
    let mut py = 500;
    let mut dd = 0;
    let mut tot = 0;
    let mut minx = 500;
    let mut miny = 500;
    let mut maxx = 500;
    let mut maxy = 500;
    mm[py * 1000 + px] = 3;
    loop {
        m.ipos = 0;
        m.input[0] = mm[py * 1000 + px] & 1;
        let c = run_program(&mut m);
        if c.is_none() { break; }
        if mm[py * 1000 + px] == 2 { tot += 1; }
        mm[py * 1000 + px] = c.unwrap();
        let d = run_program(&mut m);
        if d.is_none() { break; }
        dd = (dd + d.unwrap() * 2 + 1) % 4;
        match dd {
            0 => { py -= 1; if py < miny { miny = py } },
            1 => { px -= 1; if px < minx { minx = px } },
            2 => { py += 1; if py > maxy { maxy = py } },
            3 => { px += 1; if px > maxx { maxx = px } },
            _ => panic!("Failed"),
        }
        println!("paint {} move {} to {}x{}", c.unwrap(), d.unwrap(), px as i32 - 500, py as i32 - 500);
    }
    for y in miny..=maxy {
        for x in minx..=maxx {
            print!("{}", if mm[y*1000 + x] == 1 { '*' } else { ' ' } );
        }
        println!();
    }
    println!("Total: {}",tot)
}
