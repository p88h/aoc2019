use std::env;

fn check(mut x: u32) -> u32 {
    let mut last = x % 10;
    let mut chain : u32 = 0;
    let mut same : u32 = 0;
    for _p in 1..6 {
        x = x / 10;
        let digit = x % 10;
        //println!("Testing: {}-{} chain: {}", digit, last, chain);
        if digit == last {
            chain += 1;
        } else {
            if chain == 1 { same = 1; }
            chain = 0;
        }
        if digit > last { return 0; }
        last = digit;
    }
    if chain == 1 { same = 1; } 
    return same;
}
fn main() {
    let mut cntr : u32 = 0;
    let mut rmin = 359282;
    let mut rmax = 820401;
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        rmin = args[1].parse().expect("Invalid argument");
        rmax = rmin;
    }
    if args.len() > 2 {
        rmax = args[2].parse().expect("Invalid argument");
    }
    println!("Range: {}-{}", rmin, rmax);
    for x in rmin..=rmax {
//        println!("Checking: {}", x);
        cntr += check(x);
    }
    println!("Result: {}", cntr);
}
