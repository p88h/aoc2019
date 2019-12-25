use std::io;
use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::io::prelude::*;

struct IntCode {
    prog : Vec<i64>,
    input : Vec<u8>,
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
                    ic.prog[v as usize] = ic.input[ic.ipos] as i64;
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

struct Automap {
    map: Vec<u8>,
    pos: (usize, usize),
    bbox: (usize, usize, usize, usize),
}

impl Automap { 
    fn new() -> Self {
        Automap { map: vec![b' '; 100000], pos: (50, 50), bbox: (50, 50, 50, 50) }
    }
    fn go(&mut self, dir: &[u8]) {
        let ofs = self.pos.0*100+self.pos.1;
        self.map[ofs] = b'#';
        match dir[0] {
            b'n' => { self.map[ofs-100] = b'|'; self.pos.0 -= 2 }
            b's' => { self.map[ofs+100] = b'|'; self.pos.0 += 2 }
            b'e' => { self.map[ofs+1] = b'-'; self.pos.1 += 2 }
            b'w' => { self.map[ofs-1] = b'-'; self.pos.1 -= 2 }
            _ => {}
        }
        if self.pos.0 < self.bbox.0 { self.bbox.0 = self.pos.0 }
        if self.pos.1 < self.bbox.1 { self.bbox.1 = self.pos.1 }
        if self.pos.0 > self.bbox.2 { self.bbox.2 = self.pos.0 }
        if self.pos.1 > self.bbox.3 { self.bbox.3 = self.pos.1 }
        self.map[self.pos.0*100+self.pos.1] = b'@';
    }
    fn draw(&self) {
        println!("Map: ");
        for y in self.bbox.0..=self.bbox.2 {
            for x in self.bbox.1..=self.bbox.3 {
                print!("{}", self.map[y*100+x] as char);
            }
            println!("");
        }
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
    let mut hist = File::open("space.history")?;
    let mut hdata : Vec<u8> = Vec::new();
    hist.read_to_end(&mut hdata)?;
    // rewrite history ? 
    hist = File::create("space.history")?;
    hist.write(&hdata)?;
    let mut m = IntCode { prog: memory.to_vec(), input: vec![], pos: 0, ipos: 0, ccnt: 0, relbase: 0 };
    let mut hp = 0;
    let mut mm = Automap::new();
    loop {
        loop {
            let r = run_program(&mut m);
            if r.is_none() { 
                while hp < hdata.len() {
                    m.input.push(hdata[hp]);
                    print!("{}", hdata[hp] as char);
                    hp += 1;
                    if hdata[hp - 1] == b'\n' { break; }
                }
                // still no input?
                if m.ipos == m.input.len() {
                    // mm.draw();
                    let mut buf = String::new();
                    io::stdin().read_line(&mut buf).expect("Failure");
                    let mut data = buf.into_bytes();
                    hist.write(&data)?;    
                    m.input.append(&mut data);
                }
                // record move on the map
                mm.go(&m.input[m.ipos..m.ipos+4]);
                break; 
            }
            print!("{}", (r.unwrap() as u8) as char);
        }
    }
}
