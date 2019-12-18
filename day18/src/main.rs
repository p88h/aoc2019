use std::env;
use std::io;
use std::io::Read;
use std::fs::File;
use std::collections::HashMap;

#[derive(Default, Debug)]
struct Point {
    v : u8,
    x : usize,
    y : usize
}

#[derive(Default, Debug, Clone)]
struct Edge {
    l : char,
    i : usize,
    d : usize,
    c : u32,
    s : u32
}

#[derive(Default, Debug, Clone, Hash, Eq, PartialEq)]
struct Quant {
    i : u32,
    s : u32
}

fn empty_node() -> Vec<Edge> {
    let mut ret = Vec::<Edge>::new();
    ret.resize_with(26, Default::default);
    return ret;
}

fn explore(map: &Vec<u8>, w: usize, i: &Point) -> Vec<Edge> {
    let mut ret = empty_node();
    let mut visited = vec![false; 10000];
    let mut distance = vec![0; 10000];
    let mut cond = vec![0 as u32; 10000];
    let mut state = vec![0 as u32; 10000];
    let moves = vec![1, -1, w as i32, -(w as i32)];
    let o = i.y * w + i.x;
    visited[o] = true;
    distance[o] = 0;
    let mut stack = vec![ o ];
    let mut pos = 0;
    println!("Exploring paths from {:?}", i);
    'bfs:
    while pos < stack.len() {
        let p = stack[pos];
        let mut c = cond[p];
        let mut s = state[p];
        let d = distance[p];

        pos += 1;
        match map[p] {
            b'#' => continue 'bfs,
            b'.' | b'@' => {}
            b'a'..=b'z' => {
                let idx = (map[p] - b'a') as usize; 
                if c & (1 << idx) != 0 {
                    println!("Unreachable key {} distance {} conditions {} state {}", map[p] as char, d, c, s);                    
                } else {
                    s |= 1 << idx; 
                    println!("Add key {} distance {} conditions {} state {}", map[p] as char, d, c, s);                    
                }
                ret[idx] = Edge{l: map[p] as char, i: idx, d: d, c: c, s: s};
            }
            b'A'..=b'Z' => { 
                let idx = (map[p] - b'A') as usize;
                if s & (1 << idx) != 0 {
                    // s ^= 1 << idx;
                    println!("Unlocked door {} distance {} conditions {} state {}", map[p] as char, d, c, s);
                } else {
                    c |= 1 << idx;
                    println!("Locked door {} distance {} conditions {} state {}", map[p] as char, d, c, s);
                }
            }
            _ => println!("Invalid point: {} at {}x{}", map[p] as char, p%w,p/w)
        }
        for n in &moves {
            let np = (p as i32 + n) as usize;
            if map[np] == b'#' { continue; }
            if !visited[np] {
                stack.push(np);
                visited[np] = true;
                distance[np] = d + 1;
                cond[np] = c;
                state[np] = s;
            } else if distance[np] == d + 1 {
                if cond[np] != c || state[np] != s {
                    println!("State inconsistency at {}x{}=>{}x{} ({}) cur {}&{} was {}&{}", p%w,p/w,np%w, np/w, map[np] as char, c,s, cond[np],state[np]);
                }
            } else if distance[np] != d - 1 {
                println!("Distance inconsistency at {}x{}=>{}x{} ({}) cur {} was {}", p%w,p/w,np%w, np/w, map[np] as char,d, distance[np]);
            }          
        }
    }
    return ret;
}

fn build_matrix(map: &Vec<u8>, w: usize, keys : &Vec<Point>, doors : &Vec<Point>) -> Vec<Vec<Edge>> {
    let mut mat = vec![];
    let mut exp = 0;
    for p in 0..26 {
        if keys[p].x > 0 {
            exp = exp | ( 1 << p );
            println!("{}: key {:?} door {:?}", (b'a' + p as u8) as char, keys[p], doors[p]);
            mat.push(explore(&map, w, &keys[p]));
        } else {
            mat.push(empty_node());
        }
    }
    return mat;
}

fn improves(ij: &Edge, ik: &Edge, kj: &Edge) -> bool {
    // can't get to k - well, shoot.
    if ik.c != 0 { return false; }
    // clear keys that are set in ik.path and check if can get to j.
    if kj.c & !ik.s != 0 { return false; }
    // if either state or distance is worse - it's not better
    if ij.c == 0 && (ij.s < ik.s | kj.s || ij.d < ik.d + kj.d) { return false; }
    // if both are equal - it's not better
    if ij.c == 0 && ij.s == ik.s | kj.s && ij.d == ik.d + kj.d { return false; }
    return true;
}

// This actually makes things worse. And has a bug. Somewhere.
fn optimize_matrix(mat: &mut Vec<Vec<Edge>>, keys : &Vec<Point>) {
    loop {
        let mut cnt = 0;
        for k in 0..26 { if keys[k].x > 0 {
            for i in 0..26 { if i != k && keys[i].x > 0 {
                let il = (b'a' + i as u8) as char;
                for j in 0..26 {
                    if j != i && j != k && keys[j].x > 0 {
                        let ij = &mat[i][j];
                        let ik = &mat[i][k];
                        let kj = &mat[k][j];        
                        if !improves(&ij, &ik, &kj) { continue; }                
                        println!("kij {}-{} {:?} via {} {:?} + {:?}", il, ij.l, ij, ik.l, ik, kj);
                        mat[i][j] = Edge { l: ij.l, i: ij.i, d: ik.d + kj.d, c: 0, s: ik.s | kj.s };
                        cnt += 1;
                    }
                }
            }}
        }}
        println!("Optimized {}", cnt);
        if cnt == 0 { break; }
    }
}

fn dora_la_exploradora(mut sp: Vec<Edge>, mat: &Vec<Vec<Edge>>) -> usize {
    let mut spi = 0; 
    let mut mind = 0;
    let mut best = HashMap::new();
    let mut smask = 0;
    for p in &sp {
        if p.d != 0 { smask |= p.s; }
    }
    while spi < sp.len() {
        let p = sp[spi].clone();
        spi += 1;
        if p.d == 0 || (p.c & smask) != 0 { continue; }
        if p.s == smask && (mind == 0 || mind > p.d) {
            println!("Path @-{} contains most keys {} at cost {} (spi {})", p.l, p.s, p.d, spi);
            mind = p.d;
        }
        for k in 0..26 { if sp[k].d > 0 {
            let pik = &mat[p.i][k];
            if (pik.c & smask) & !p.s == 0 && pik.s | p.s > p.s {
                let q = Quant { i: pik.i as u32, s: pik.s | p.s };
                if best.contains_key(&q) && p.d + pik.d >= *best.get(&q).unwrap() {
                    continue;
                }
                //println!("{}..{} {}=>{} @d {}", p.l, pik.l, p.s, p.s | pik.s, p.d + pik.d);
                sp.push(Edge{l:pik.l, i:pik.i, d: p.d + pik.d, s: p.s | pik.s, c: 0});
                best.insert(q, p.d + pik.d);
            }
        }}    
    }
    println!("Best local result: {}", mind);
    return mind;
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut file = File::open(&args[1])?;
    let mut data: Vec<u8> = Vec::new();
    file.read_to_end(&mut data)?;
    let mut w = 0;
    let mut h = 0;
    let mut s : Point = Default::default();
    let mut doors = Vec::<Point>::new();
    let mut keys = Vec::<Point>::new();
    doors.resize_with(26, Default::default);
    keys.resize_with(26, Default::default);
    for i in 0..data.len() {
        if data[i] == b'\n' && w == 0{
            w = i + 1;
            h = data.len() / w;
        }
        if data[i] == b'#' || data[i] == b'.' || data[i] == b'\n' {
            continue;
        }
        let p = Point{v: data[i], x: i % w, y: i / w};
        match data[i] {
            b'@' => s = p,
            b'a'..=b'z' => keys[(data[i] - b'a') as usize] = p,
            b'A'..=b'Z' => doors[(data[i] - b'A') as usize] = p,
            _ => println!("Invalid point: {:?}", p)
        }
    }
    println!("map size: {}x{}", w, h);
    println!("Start: {:?}", s);
    let mat = build_matrix(&data, w, &keys, &doors);
    //don't...
    //optimize_matrix(&mut mat, &keys);
    let answer1 = dora_la_exploradora(explore(&data, w, &s), &mat);
    let cpos = s.y * w + s.x;
    let walls = vec![1, -1, w as i32, -(w as i32)];
    let bots = vec![w as i32 + 1, w as i32 -1, -(w as i32) -1, -(w as i32) + 1];
    for w in walls {
        data[((cpos as i32) + w) as usize] = b'#';
    }
    let mat2 = build_matrix(&data, w, &keys, &doors);
    let mut answer2 = 0;
    for b in bots {
        let bpos = (cpos as i32 + b) as usize;
        let bp = Point { v: b'@', y: bpos/w, x: bpos%w };
        answer2 += dora_la_exploradora(explore(&data, w, &bp), &mat2);
    }
    println!("Answer 1: {} and 2: {}", answer1, answer2);
    Ok(())
}
