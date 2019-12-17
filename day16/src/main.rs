use std::io;
use std::io::Read;

fn main() -> io::Result<()> {
    let mut data: Vec<u8> = Vec::new();
    io::stdin().read_to_end(&mut data)?;
    let mut nums = vec![0; data.len() * 10000];
    for i in 0..data.len()*10000 {
        nums[i] = (data[i % data.len()] - b'0') as i64;
    }
    let mut sums = vec![0; nums.len() + 1];
    let mut prod = vec![0; nums.len()];
    let mut sc : i64 = 0;
    let mut ofs = 0;
    for i in 0..7 {
        ofs = ofs*10 + nums[i];
    }
    println!("Message offset: {}", ofs);
    for l in 0..100 {
        for i in 1..=nums.len() {
            sums[nums.len() - i] = sums[nums.len() - i + 1] + nums[nums.len() - i];
        }
        for i in 0..nums.len() {
            let mut gp = i;
            let mut r = 0;
            while gp < nums.len() {
                // additions
                let ga = if gp + i + 1 < nums.len() { gp + i + 1 } else { nums.len() };
                r += sums[gp] - sums[ga];
                gp += 2 * i + 2;
                // subtractions
                if gp < nums.len() {
                    let gb = if gp + i + 1 < nums.len() { gp + i + 1 } else { nums.len() };
                    r -= sums[gp] - sums[gb];   
                }
                gp += 2 * i + 2;
                sc += 1;
            }
            prod[i] = r.abs() % 10;
        }
        nums = prod.clone();
        print!("Iter[{}]: ", l);
        for i in ofs..ofs+8 {
            print!("{}",nums[i as usize]);
        }
        println!(" @cycles = {}", sc); 
    }
    Ok(())
}
