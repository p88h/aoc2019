extern crate pancurses;

use std::env;
use std::io;
use std::u64;
use std::io::Read;
use std::fs::File;
use std::collections::HashMap;

use pancurses::Input;

#[derive(Debug, Clone)]
struct Board {
    tiles: Vec<Vec<u8>>,
    adj: Vec<Vec<u32>>,
    walls: Vec<u32>,
    pop: u32,
    value: u32
}

impl Board {
    fn new() -> Self {
        let mut a = vec![];
        let mut t = vec![];
        for i in 0..5 {
            a.push(vec![0; 5]);
            t.push(vec![b'.'; 5]);
        }
        t[2][2] = b'X';
        Board { tiles: t, adj: a, walls: vec![0; 4], value: 0, pop: 0 }
    }

    fn load(&mut self, d: Vec<u8>, s: usize) {
        for i in 0..5 {
            for j in 0..5 {
                self.tiles[i][j] = d[i * s + j];
            }
        }
        self.count();
    }

    fn count(&mut self) {
        let mut k = 1;
        self.value = 0;
        self.pop = 0;
        self.walls = vec![0; 4];
        for i in 0..5 {
            if self.tiles[0][i] == b'#' { self.walls[0] += 1 } // upper wall
            if self.tiles[i][0] == b'#' { self.walls[1] += 1 } // left wall
            if self.tiles[4][i] == b'#' { self.walls[2] += 1 } // lower wall
            if self.tiles[i][4] == b'#' { self.walls[3] += 1 } // right wall
            for j in 0..5 {
                if self.tiles[i][j] == b'#' {
                    self.value += k;
                    self.pop += 1;
                }
                k *= 2;
            }
        }
    }

    fn update(&mut self, inner: &Board, outer: &Board) {
        for i in 0..5 {
            for j in 0..5 {
                let mut a = 0;
                // look down
                match i {
                    0|2|3 => if self.tiles[i+1][j] == b'#' { a += 1; },
                    1 => if j == 2 { a += inner.walls[0]} else if self.tiles[i+1][j] == b'#' { a += 1; },
                    4 => if outer.tiles[3][2] == b'#' { a +=1; },
                    _ => {}
                }
                // look right
                match j {
                    0|2|3 => if self.tiles[i][j+1] == b'#' { a += 1; },
                    1 => if i == 2 { a += inner.walls[1] } else if self.tiles[i][j+1] == b'#' { a += 1; },
                    4 => if outer.tiles[2][3] == b'#' { a +=1; },
                    _ => {}
                }
                // look up
                match i {
                    1|2|4 => if self.tiles[i-1][j] == b'#' { a += 1; },
                    3 => if j == 2 { a += inner.walls[2] } else if self.tiles[i-1][j] == b'#' { a += 1; },
                    0 => if outer.tiles[1][2] == b'#' { a +=1; },
                    _ => {}
                }
                // look left
                match j {
                    1|2|4 => if self.tiles[i][j-1] == b'#' { a += 1; },
                    3 => if i == 2 { a += inner.walls[3] } else if self.tiles[i][j-1] == b'#' { a += 1; },
                    0 => if outer.tiles[2][1] == b'#' { a +=1; },
                    _ => {}
                }
                self.adj[i][j] = a;
            }
        }
    }

    fn mutate(&mut self) {
        for i in 0..5 {
            for j in 0..5 {
                match self.adj[i][j] {
                    1 => self.tiles[i][j] = b'#',
                    2 => if self.tiles[i][j] == b'.' { self.tiles[i][j] = b'#' } else { self.tiles[i][j] = b'.' },
                    _ => self.tiles[i][j] = b'.',
                }
            }
        }
        self.tiles[2][2]=b'X';
    }
}

impl Default for Board {
    fn default() -> Self { Board::new() }
}

const MAX_SIZE : usize = 320;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut file = File::open(&args[1])?;
    let mut data: Vec<u8> = Vec::new();
    file.read_to_end(&mut data)?;
    let mut s = data.len();
    for i in 0..data.len() {
        if data[i] == b'\n' {
            s = i + 1;
            break;
        }
    }
    let mut levels : Vec<Board> = Vec::new();
    levels.resize_with(MAX_SIZE, Default::default);
    levels[MAX_SIZE / 2].load(data, s);
    let mut lmin = MAX_SIZE / 2 - 1;
    let mut lmax = MAX_SIZE / 2 + 1;
    let mut cnt = 0;
    // screen setup

    let window = pancurses::initscr();
    let mw = window.get_max_x() / 6;
    let mh = (window.get_max_y() / 6) + 1;
    let mo = mw * mh / 2 - MAX_SIZE as i32 / 2;
    println!("mw: {} mh: {} mo: {} center: {}", mw, mh, mo, MAX_SIZE / 2);
    pancurses::nl();
    pancurses::noecho();
    pancurses::curs_set(0);
    window.timeout(0);
    window.keypad(true);
    window.nodelay(true);
    //let mut cache = HashMap::new();
    loop {
        let mut tot = 0;
        for l in lmin..=lmax {
            let mut tmp = levels[l].clone();
            tmp.update(&levels[l-1], &levels[l+1]);
            levels[l] = tmp;
        }
        for l in lmin..=lmax {
            levels[l].mutate();
        }
        for l in lmin..=lmax {
            levels[l].count();
            tot += levels[l].pop;
        }
        if levels[lmin].pop > 0 { lmin -= 1; }
        if levels[lmax].pop > 0 { lmax += 1; }
        cnt += 1;
        for l in lmin+1..lmax {
            let o = l as i32 + mo;
            if o < 0 || o >= mw*mh { continue; }
            for i in 0..5 {
                for j in 0..5 {
                    window.mvaddch((o / mw) * 6 + i as i32 + 1, (o % mw) * 6 + j as i32, levels[l].tiles[i][j] as char);
                }
            }
        }
        window.mvaddstr(0, 0, format!("Round {} Levels: {}-{} Population: {}", cnt, lmin+1,lmax-1, tot));
        window.refresh();
        pancurses::napms(50);
        if cnt == 200 {
            pancurses::napms(10000);
            break;
        }
    }
    pancurses::curs_set(1);
    pancurses::endwin();
    Ok(())
}
