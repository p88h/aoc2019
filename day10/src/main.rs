use std::io;

struct Angle {
    dx: usize,
    dy: usize,
    deg: f32,
}

fn los(map: &Vec<Vec<u8>>, x: usize, y: usize, w: usize, h: usize, angles: &Vec<Angle>) -> usize {
    let mut cnt = 0;
    // +dx, +dy
    for a in angles {
        let mut m = 1;
        while x + a.dx * m < w && y + a.dy * m < h {
            if map[y + a.dy * m][x + a.dx * m] == b'#' { 
                cnt += 1; break;
            };
            m += 1;
        }
    }
    // +dx, -dy (swapped)
    for a in angles {
        let mut m = 1;
        while x + a.dy * m < w && y >= a.dx * m {
            if map[y - a.dx * m][x + a.dy * m] == b'#' { 
                cnt += 1; break;
            };
            m += 1;
        }
    }
    // -dx, -dy
    for a in angles {
        let mut m = 1;
        while x >= a.dx * m && y >= a.dy * m {
            if map[y - a.dy * m][x - a.dx * m] == b'#' { 
                cnt += 1; break;
            };
            m += 1;
        }
    }
    // -dx, +dy (swapped)
    for a in angles {
        let mut m = 1;
        while x >= a.dy * m && y + a.dx * m < h {
            if map[y + a.dx * m][x - a.dy * m] == b'#' { 
                cnt += 1; break;
            };
            m += 1;
        }
    }
    println!("{}x{} sees {}", x, y, cnt);
    return cnt;
}

fn chk(map: &mut Vec<Vec<u8>>, px: usize, py: usize, iter: &mut usize) -> Option<usize> {
    if map[py][px] == b'#' { 
        map[py][px] = b'x';
        println!("{}: {}x{}", iter, px, py); 
        *iter -= 1;
        if *iter == 0 { return Some(px * 100 + py); }
        return Some(0);
    };
    return None;
}

fn fire(mut map: Vec<Vec<u8>>, x: usize, y: usize, w: usize, h: usize, angles: &Vec<Angle>, mut iter: usize) -> usize {
    // +dx, -dy 
    for a in angles {
        let mut m = 1;
        while x + a.dx * m < w && y >= a.dy * m {
            let r = chk(&mut map, x + a.dx * m, y - a.dy * m, &mut iter);
            if r.is_some() { if iter > 0 { break; } else { return r.unwrap(); } }
            m += 1;
        }
    }
    // +dx, +dy
    for a in angles {
        let mut m = 1;
        while x + a.dy * m < w && y + a.dx * m < h {
            let r = chk(&mut map, x + a.dy * m, y + a.dx * m, &mut iter);
            if r.is_some() { if iter > 0 { break; } else { return r.unwrap(); } }
            m += 1;
        }
    }
    // -dx, +dy (swapped)
    for a in angles {
        let mut m = 1;
        while x >= a.dx * m && y + a.dy * m < h {
            let r = chk(&mut map, x - a.dx * m, y + a.dy * m, &mut iter);
            if r.is_some() { if iter > 0 { break; } else { return r.unwrap(); } }
            m += 1;
        }
    }
    // -dx, -dy
    for a in angles {
        let mut m = 1;
        while x >= a.dy * m && y >= a.dx * m {
            let r = chk(&mut map, x - a.dy * m, y - a.dx * m, &mut iter);
            if r.is_some() { if iter > 0 { break; } else { return r.unwrap(); } }
            m += 1;
        }
    }
    return 0;
}

fn main() {
    let mut map = Vec::new();
    let mut w = 0;
    loop {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).expect("Failure");
        if buf.trim().len() < 2 { break; }
        w = buf.len();
        map.push(buf.as_bytes().to_vec());
    }
    let mut vv = Vec::new();
    let h = map.len();
    // generate angles
    let d = if w > h { w } else { h };
    for y in 1..d {
        for x in 0..d {
            vv.push(Angle{dx: x, dy: y, deg: x as f32 / y as f32 }); 
        }
    }
    vv.sort_by(|a, b| a.deg.partial_cmp(&b.deg).unwrap());
    // de-duplicate
    let mut vvd = Vec::new();
    let mut prev = -1.0;
    for a in vv { 
        if a.deg != prev { 
            prev = a.deg;
            // println!("{} {}/{}", a.deg, a.dx, a.dy);
            vvd.push(a); 
        }
    }
    // find best position
    let mut best = 0;
    let mut bx = 0;
    let mut by = 0;
    for y in 0..h {
        for x in 0..w {
            if map[y][x] != b'#' { continue; }
            let l = los(&map, x, y, w, h, &vvd);
            if l > best {
                best = l;
                bx = x;
                by = y;
            }
        }
    }
    println!("{} at {}x{}", best, bx, by);
    let prod = fire(map, bx, by, w, h, &vvd, 200);
    println!("Result: {}", prod);
}

/*
.#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....#...###..
..#.#.....#....##
*/