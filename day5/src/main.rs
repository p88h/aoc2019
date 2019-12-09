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
    println!("@{} opcode {} mode {} : {}, {}, {}", pos, v[pos], mode, op.v1, op.v2, op.v3);
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
    println!("@{} opcode {} mode {} : {}, {}", pos, v[pos], mode, op.v1, op.v2);
    if mode1 == 0 { op.v1 = v[op.v1 as usize]; }
    if mode2 == 0 { op.v2 = v[op.v2 as usize]; }
    return op;
} 

fn read1(v : &Vec<i32>, mode: i32, pos: usize) -> i32 {
    let mut v1 = v[pos+1]; 
    println!("@{} opcode {} mode {} : {}", pos, v[pos], mode, v1);
    if mode == 0 { v1 = v[v1 as usize]; }
    return v1;
} 

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Failure");
    let split = buf.trim().split(",");
    let memory : Vec<_> = split.filter_map(|x| x.parse::<i32>().ok()).collect();
    let input = vec![ 5 ];
    let mut ipos = 0;
    let mut prog = memory.to_vec();
    let mut pos : usize = 0;
    loop {
        let opcode = prog[pos] % 100;
        let mode = prog[pos] / 100;
        match opcode {
            1 => { 
                    let op = read3(&prog, mode, pos);
                    prog[op.v3 as usize]= op.v1 + op.v2;
                    println!("m[{}] = {} + {} => {}",op.v3,op.v1,op.v2,prog[op.v3 as usize]);
                    pos += 4;
                }, 
            2 => { 
                    let op = read3(&prog, mode, pos);
                    prog[op.v3 as usize]= op.v1 * op.v2;
                    println!("m[{}] = {} * {} => {}",op.v3,op.v1,op.v2,prog[op.v3 as usize]);
                    pos += 4;
                }, 
            3 => { 
                    let v = prog[pos + 1]; 
                    prog[v as usize] = input[ipos];
                    println!("Input: {} => {} (mode {})", v, prog[v as usize], mode);
                    ipos += 1;
                    pos += 2;
                 }, 
            4 => { 
                    let v = read1(&prog, mode, pos); 
                    println!("Output: {}", v);
                    pos += 2;
                 }, 
            5 => {  
                    let op = read2(&prog, mode, pos); 
                    pos += 3;
                    if op.v1 != 0 { pos = op.v2 as usize; }
                    println!("@ = {}? => {}",op.v1,op.v2);
                 }, 
            6 => {  
                    let op = read2(&prog, mode, pos); 
                    pos += 3;
                    if op.v1 == 0 { pos = op.v2 as usize; }
                    println!("@ = !{}? => {}",op.v1,op.v2);
                  }, 
            7 => {  
                    let op = read3(&prog, mode, pos); 
                    prog[op.v3 as usize] = if op.v1 < op.v2 { 1 } else { 0 };
                    println!("m[{}] = {} < {} => {}",op.v3,op.v1,op.v2,prog[op.v3 as usize]);
                    pos += 4;
                 }, 
            8 => {  
                    let op = read3(&prog, mode, pos); 
                    prog[op.v3 as usize] = if op.v1 == op.v2 { 1 } else { 0 };
                    println!("m[{}] = {} == {} => {}",op.v3,op.v1,op.v2,prog[op.v3 as usize]);
                    pos += 4;
                  }, 
            99 => break,
            _ => panic!("Failed"),
        }
    }
}
