use std::io;
use std::io::Read;
use std::io::Write;

static WIDTH : usize = 25;
static HEIGHT : usize = 6;
static BLOCK : usize = WIDTH * HEIGHT;

fn main() -> io::Result<()> {
    let mut data: Vec<u8> = Vec::new();
    let mut image = vec![b'2'; BLOCK + HEIGHT];
    io::stdin().read_to_end(&mut data)?;
    let mut min0 = BLOCK;
    let mut minr = 0;
    for j in 1..=HEIGHT {
        image[j*(WIDTH+1)-1] = b'\n';
    }
    for i in 0..data.len() / BLOCK {
        let mut digits = vec![0; 3];
        for j in 0..HEIGHT {
            for k in 0..WIDTH {
                let v = data[i*BLOCK + j*WIDTH + k];
                let d = (v - b'0') as usize;
                digits[d] += 1;
                let p = image[j*(WIDTH+1)+k];
                if p == b'2' {
                    image[j*(WIDTH+1)+k] = v;
                }
            }
        }
        println!("Layer {} Digits: {:?}", i, digits);
        if digits[0] < min0 {
            min0 = digits[0];
            minr = digits[1]*digits[2];
        }
    }
    println!("Min: {} Res: {}", min0, minr);
    io::stdout().write_all(&image)?;
    Ok(())
}
