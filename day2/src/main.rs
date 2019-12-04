extern crate itertools; // 0.7.8
use std::convert::TryInto;
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Failure");
    let split = buf.trim().split(",");
    let memory : Vec<_> = split.filter_map(|x| x.parse::<i32>().ok()).collect();
    for i in 0..100 {
        for j in 0..100 {
            let mut prog = memory.to_vec();
            let mut pos : usize = 0;
            prog[1] = i;
            prog[2] = j;
            loop {
                match prog[pos] {
                    1 => { let i1 : usize = prog[pos+1].try_into().unwrap(); 
                           let i2 : usize = prog[pos+2].try_into().unwrap();
                           let i3 : usize = prog[pos+3].try_into().unwrap();
                           prog[i3]=prog[i1]+prog[i2];
                           //println!("m[{}] = m[{}] + m[{}]; // {} + {} => {}",i3,i1,i2,prog[i1],prog[i2],prog[i3]);
                           pos += 4 
                         }, 
                    2 => { let i1 : usize = prog[pos+1].try_into().unwrap(); 
                           let i2 : usize = prog[pos+2].try_into().unwrap();
                           let i3 : usize = prog[pos+3].try_into().unwrap();
                           prog[i3]=prog[i1]*prog[i2];
                           //println!("m[{}] = m[{}] * m[{}]; // {} * {} => {}",i3,i1,i2,prog[i1],prog[i2],prog[i3]);
                           pos += 4
                        }, 
                    99 => break,
                    _ => panic!("Failed"),
                }
            }
            if prog[0] == 19690720 {
                println!("{},{}, {:?}", i, j, prog);
            }
        }
    }
}
