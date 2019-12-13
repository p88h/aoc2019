use std::io;

fn update_d(a: &mut Vec<i32>, b: &mut Vec<i32>) {
    for d in 0..3 {
        if a[d] > b[d] { b[d+3] += 1; a[d+3] -= 1; }
        if a[d] < b[d] { a[d+3] += 1; b[d+3] -= 1; }    
    }
}

fn update_v(a: &mut Vec<i32>) {
    for d in 0..3 {
        a[d] += a[d + 3];
    }
}

fn dump(a: &Vec<i32>) -> i32 {
    let pot = a[0].abs()+a[1].abs()+a[2].abs();
    let kin = a[3].abs()+a[4].abs()+a[5].abs();
    println!("pos=<x={}, y={}, z={}>, vel=<x={}, y={}, z={}> pot={} kin={}",a[0],a[1],a[2],a[3],a[4],a[5],pot,kin);
    return pot * kin;
}

fn four_body_problem(a: &mut Vec<i32>, b: &mut Vec<i32>, c: &mut Vec<i32>, d: &mut Vec<i32>) {
    update_d(a, b);
    update_d(a, c);
    update_d(a, d);
    update_d(b, c);
    update_d(b, d);
    update_d(c, d);
    update_v(a);
    update_v(b);
    update_v(c);
    update_v(d);
}

fn compare_systems(step1: i32, step2: i32, res: &mut Vec<i32>,
                   a1: &Vec<i32>, b1: &Vec<i32>, c1: &Vec<i32>, d1: &Vec<i32>,
                   a2: &Vec<i32>, b2: &Vec<i32>, c2: &Vec<i32>, d2: &Vec<i32>) -> i32 {
        let mut ret = 0;
//        println!("cmp {}-{} d{}",step1, step2, step2-step1);
        for i in 0..3 {
            if a1[i] == a2[i] && b1[i] == b2[i] && c1[i] == c2[i] && d1[i] == d2[i]
            && a1[i+3] == a2[i+3] && b1[i+3] == b2[i+3] && c1[i+3] == c2[i+3] && d1[i+3] == d2[i+3] {
                println!("dim: {} cycle at: {}:{} len? {}", i, step1, step2, step2 - step1);
                if res[i] == 0 {
                    res[i] = step2 - step1;
                    ret += 1;
                } else if res[i+3] == -1 {
                    res[i + 3] = step1;
                    ret += 1;
                }
            }    
        }
        return ret;
    }

fn gcd(a: i64, b: i64) -> i64 {
    if a < b { return gcd(b, a); }
    if a % b == 0 { return b; }
    return gcd (b, a - b);
}

fn lcm(a: i64, b: i64) -> i64 {
    let g = gcd(a, b);
    return (a * b) / g;    
}

fn main() {
   let mut m1 = vec![ -7,  -8,  9, 0, 0, 0];
    let mut m2 = vec![-12,  -3, -4, 0, 0, 0];
    let mut m3 = vec![  6, -17, -9, 0, 0, 0];
    let mut m4 = vec![  4, -10, -6, 0, 0, 0]; 
/*   let mut m1 = vec![ -1,   0,  2, 0, 0, 0];
    let mut m2 = vec![  2, -10, -7, 0, 0, 0];
    let mut m3 = vec![  4,  -8,  8, 0, 0, 0];
    let mut m4 = vec![  3,   5, -1, 0, 0, 0]; */
/*    let mut m1 = vec![ -8, -10,  0, 0, 0, 0];
    let mut m2 = vec![  5,   5, 10, 0, 0, 0];
    let mut m3 = vec![  2,  -7,  3, 0, 0, 0];
    let mut m4 = vec![  9,  -8, -3, 0, 0, 0]; */
    let mut f1 = m1.clone(); let mut b1 = m1.clone();
    let mut f2 = m2.clone(); let mut b2 = m2.clone();
    let mut f3 = m3.clone(); let mut b3 = m3.clone();
    let mut f4 = m4.clone(); let mut b4 = m4.clone();
/*    let mut m1 = Moon{x:-1, y:0, z:2, dx:0, dy:0, dz:0};
    let mut m2 = Moon{x:2, y:-10, z:-7, dx:0, dy:0, dz:0};
    let mut m3 = Moon{x:4, y:-8, z:8, dx:0, dy:0, dz:0};
    let mut m4 = Moon{x:3, y:5, z:-1, dx:0, dy:0, dz:0}; */
    let mut step = 0;
    let mut cycles = vec![0, 0, 0, 0, 0, 0];
    let mut cnt = 0;
    while cnt != 3 {
        //println!("After {} steps:", step);
        //let mut tot = 0;
        //tot += dump(&m1); tot += dump(&m2); tot += dump(&m3); tot+= dump(&m4);
        //println!("Total: {}", tot);
        four_body_problem(&mut f1, &mut f2, &mut f3, &mut f4);
        cnt += compare_systems(step, step * 2 + 1, &mut cycles, &m1, &m2, &m3, &m4, &f1, &f2, &f3, &f4);
        four_body_problem(&mut f1, &mut f2, &mut f3, &mut f4);
        cnt += compare_systems(step, step * 2 + 2, &mut cycles, &m1, &m2, &m3, &m4, &f1, &f2, &f3, &f4);
        four_body_problem(&mut m1, &mut m2, &mut m3, &mut m4);
        step += 1;
    }
    for i in 0..3 {
        m1 = b1.clone(); m2 = b2.clone(); m3 = b3.clone(); m4 = b4.clone();
        f1 = b1.clone(); f2 = b2.clone(); f3 = b3.clone(); f4 = b4.clone();
        for l in 0..cycles[i] {
            //println!("[{}.{}] {} {} {} {} d {} {} {} {}", i, l, f1[i], f2[i], f3[i], f4[i], f1[i+3], f2[i+3], f3[i+3], f4[i+3]);
            four_body_problem(&mut f1, &mut f2, &mut f3, &mut f4);
        }
        cycles[i+3] = -1;
        let mut step = 0;
        loop {
            if compare_systems(step, step + cycles[i], &mut cycles, &m1, &m2, &m3, &m4, &f1, &f2, &f3, &f4) > 0 {
                println!("dim: {} cycle offset: {}", i, step);
                break;
            }
            if step > cycles[i] {
                println!("bad cycle len {} in dim {}", cycles[i], i);
                break; 
            }
            if cycles[i+3] != 0 { break; }
            //println!("[{}.{}] {} {} {} {} d {} {} {} {}", i, step+cycles[i], f1[i], f2[i], f3[i], f4[i], f1[i+3], f2[i+3], f3[i+3], f4[i+3]);
            four_body_problem(&mut f1, &mut f2, &mut f3, &mut f4);
            four_body_problem(&mut m1, &mut m2, &mut m3, &mut m4);
            step += 1;
        }
    }
    let l1 = lcm(cycles[0] as i64, cycles[1] as i64);
    let l2 = lcm(l1, cycles[2] as i64);
    println!("dim 0: {} dim 1: {} lcm: {} dim 2: {} lcm: {}", cycles[0], cycles[1], l1, cycles[2], l2);
}
