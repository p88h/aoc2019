use std::env;
use std::io;
use std::io::Read;
use std::fs::File;
use std::collections::HashMap;
use std::ops::Add;

use ggez;
use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};

const CELL_SIZE: (i32, i32) = (4, 4);

#[derive(Default, Debug, Clone, Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

// convert Point into Rect
impl From<Point> for graphics::Rect {
    fn from(pos: Point) -> Self {
        graphics::Rect::new_i32(pos.x * CELL_SIZE.0, pos.y * CELL_SIZE.1, CELL_SIZE.0, CELL_SIZE.1)
    }
}

// convert loop coordinate pair into Point
impl From<(usize, usize)> for Point {
    fn from(pos: (usize, usize)) -> Self {
        Point { x: pos.0 as i32, y: pos.1 as i32}
    }
}

// adding Points together
impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Default, Debug, Clone, Hash, Eq, PartialEq)]
struct State {
    p: Point,
    d: usize,
    a: bool
}

#[derive(Default, Clone)]
struct GameBoard {
    map: Vec<Vec<u8>>,
    width: usize,
    height: usize,
    portals: HashMap<Point, Point>,
    labels: HashMap<Point, String>,
    distmap: HashMap<String, usize>,
    mesh: HashMap<Point, Vec<State>>,
    start: Point,
    end: Point
}

impl GameBoard {
    fn valid(&self, p: &Point) -> bool {
        if p.x < 0 || p.y < 0 { return false; }
        if p.x as usize >= self.width || p.y as usize >= self.height { return false; }
        match self.map[p.y as usize][p.x as usize] {
            b'.' | b'o' => true,
            _ => false
        }
    }

    fn outer(&self, p: &Point) -> bool {
        if p.x == 0 || p.y == 0 || p.x == (self.width as i32 - 1) || p.y == (self.height as i32 - 1) { return true }
        return false;
    }

    fn scan(&mut self, i: &Point) {
        let mut visited = HashMap::new();
        let mut stack = vec![];
        let mut sp = 0;
        let prefix = self.labels.get(i).unwrap().clone();
        let pc = if self.outer(i) { 'o' } else { 'i' };
        println!("Scan {}{}",pc,prefix);
        stack.push(State{ p: i.clone(), d: 0, a: false});
        visited.insert(i.clone(), 1);
        let mut mv = vec![];
        while sp < stack.len() {
            let s = stack[sp].clone();
            sp += 1;
            let moves = vec![Point{ x: 1, y: 0}, Point{ x: -1, y: 0 }, Point{ x: 0, y: 1 }, Point{ x: 0, y: -1 }];
            for m in moves {
                let np = s.p.clone() + m;
                if !self.valid(&np) { continue; } 
                if visited.contains_key(&np) { continue; }
                visited.insert(np.clone(), 1);
                let ou = self.outer(&np);
                let ns = State{ p: np, d: s.d + 1, a: ou};
                stack.push(ns.clone());
            }
            // store distance at portal
            if self.labels.contains_key(&s.p) {
                let nl = self.labels.get(&s.p).unwrap();
                // loops are not that useful
                if *nl == prefix { continue; }
                let c = if self.outer(&s.p) { 'o' } else { 'i' };
                let pnl = format!("{}{}{}", prefix, c, nl);
                let nlp = format!("{}{}{}", nl, pc, prefix);
                let pd = self.distmap.get(&pnl);
                if pd.is_some() && *pd.unwrap() != s.d {
                    println!("Previous path with distance {} vs {}", pd.unwrap(), s.d);
                } else if pd.is_none() {
                    println!("{} {}", pnl, s.d);
                    println!("{} {}", nlp, s.d);
                    self.distmap.insert(pnl, s.d.clone());
                    self.distmap.insert(nlp, s.d.clone());
                }
                mv.push(s.clone());
            }            
        }
        self.mesh.insert(i.clone(), mv);
    }

    fn solve(self) {
        let mut visited = HashMap::new();
        let mut stack = vec![];
        let mut sp = 0;
        let mut maxd = 0;
        let is = State{ p: self.start.clone(), d: 0, a: true};
        let fin = State { p: self.end.clone(), d: 0, a: true};
        stack.push(is.clone());
        visited.insert(is, 0);
        while sp < stack.len() {
            let s = stack[sp].clone();
            let sd = visited.get(&s).unwrap().clone();
            for n in self.mesh.get(&s.p).unwrap() {
                let np = self.portals.get(&n.p);
                // outer portal on topmost level
                if s.d == 0 && np.is_some() && n.a { continue; }
                // AA/ZZ on non-top levels
                if s.d > 0 && np.is_none() { continue; }
                let nd = if n.a { s.d - 1 } else { s.d + 1 };
                let ns = if np.is_some() { State { p: np.unwrap().clone(), d: nd, a: !n.a } } else 
                                         { State { p: n.p.clone(), d: 0, a: n.a } };
                let nsd = visited.get(&ns);
                let sdn = if np.is_some() { sd + n.d + 1 } else { sd + n.d };
                if maxd > 0 && sdn >= maxd { continue; }
                if nsd.is_some() && *nsd.unwrap() <= sdn { continue; }
                println!("{}@{:?}.{} => {}@{:?}->{:?}.{} #{}", self.labels.get(&s.p).unwrap(), s.p, s.d,
                                                               self.labels.get(&n.p).unwrap(), n.p, ns.p, ns.d, sdn);
                if ns == fin { maxd = sdn }
                visited.insert(ns.clone(), sdn);
                stack.push(ns);
            }
            sp += 1;
        }
        let fv = visited.get(&fin);
        if fv.is_some() {
            println!("Final dist: {:?}", fv.unwrap())
        }
    }
}

struct MainState {
    board: GameBoard,
    drawn: i32,
    stack: Vec<State>,
    sp: usize,
    visited: HashMap<Point, usize>,
    disp: Vec<State>,
    clear: Vec<State>,
    dist: usize
}

impl MainState {
    fn new(board: GameBoard) -> GameResult<MainState> {
        let mut s = MainState { 
            board: board, 
            drawn: 0, 
            stack: vec![], 
            sp: 0, 
            visited: HashMap::new(), 
            disp: vec![], 
            clear: vec![], 
            dist: 0 };
        // initialize search
        s.stack.push(State{ p: s.board.start.clone(), d: 0, a: true});
        s.visited.insert(s.board.start.clone(), 1);
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        //self.pos_x = self.pos_x % 800.0 + 1.0;
        let mut nstack = vec![];
        self.disp.clear();
        while self.sp < self.stack.len() {
            let s = &mut self.stack[self.sp];
            let p = &s.p;
            let d = s.d.clone();
            self.sp += 1;
            s.a = false;
            self.disp.push(s.clone());
            let moves = vec![Point{ x: 1, y: 0}, Point{ x: -1, y: 0 }, Point{ x: 0, y: 1 }, Point{ x: 0, y: -1 }];
            for m in moves {
                let np = p.clone() + m;
                if !self.board.valid(&np) { continue; }
                let mut v = self.visited.get_mut(&np);
                let pd = if v.is_none() { 0 } else { *v.unwrap() };
                if pd & (1 << d) == 0 {
                    self.visited.insert(np.clone(), pd | (1 << d));
                    // println!("Go {:?} => {:?} d {}|{}", p, np, pd, d);
                    let ns = State{ p: np, d: d, a: true};
                    self.disp.push(ns.clone());
                    nstack.push(ns);
                }
            }
            // or move through portal
            if self.board.portals.contains_key(&p) {
                let np = self.board.portals.get(&p).unwrap();
                let mut dd = d + 1;
                if self.board.outer(p) {
                    if d == 0 { continue; }
                    dd = d - 1;
                }
                let mut v = self.visited.get_mut(&np);
                let pd = if v.is_none() { 0 } else { *v.unwrap() };
                if pd & (1 << dd) == 0 {
                    self.visited.insert(np.clone(), pd | (1 << dd));
                    let ns = State{ p: np.clone(), d: d, a: true};
                    self.disp.push(ns.clone());
                    nstack.push(ns);
                    //println!("Bzzm {}: {:?} -> {:?} level {}=>{} was {}", self.board.labels.get(&p).unwrap(), p, np, d, dd, pd);
                }
            }
        }
        self.dist += 1;
        if self.stack.len() > 0 {
            let ed = self.visited.get(&self.board.end);
            if ed.is_some() && *ed.unwrap() & 1 == 1 {
                println!("END in {} at {:?}", self.dist, self.board.end);
                nstack.clear();
            }
        }
        self.stack = nstack;
        self.sp = 0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        if self.drawn < 2 {
            graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
            for y in 0..self.board.height {
                for x in 0..self.board.width {
                    // Skip hall tiles
                    //print!("{}", self.board.map[y][x] as char);
                    //if self.board.map[y][x] == b'.' { continue; }
                    let color = match self.board.map[y][x] {
                        b'o' => [0.5, 0.5, 1.0, 1.0],
                        b'#' => [0.5, 0.5, 0.5, 1.0],
                        _ => [0.0, 0.0, 0.0, 1.0]
                    };
                    let pos = Point::from((x, y));
                    let rectangle = graphics::Mesh::new_rectangle(
                        ctx, graphics::DrawMode::fill(),
                        pos.into(),
                        color.into(),
                    )?;
                    graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
                }
                //println!();
            } 
            self.drawn += 1;
        }
        if self.disp.len() > 0 || self.clear.len() > 0 {
            for s in &self.clear {
                let p = &s.p;
                let color = if s.a { [ 1.0, 0.5, 0.5, 1.0] } else { [0.5, 0.1, 0.1, 1.0] };
                let rectangle = graphics::Mesh::new_rectangle(
                    ctx, graphics::DrawMode::fill(),
                    p.clone().into(),
                    color.into(),
                )?;
                graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
            }
            self.clear.clear();
            for s in &self.disp {
                let p = &s.p;
                let color = if s.a { [ 1.0, 0.5, 0.5, 1.0] } else { [0.5, 0.1, 0.1, 1.0] };
                let rectangle = graphics::Mesh::new_rectangle(
                    ctx, graphics::DrawMode::fill(),
                    p.clone().into(),
                    color.into(),
                )?;
                graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
                if !s.a { self.clear.push(s.clone()); }
            }
            self.disp.clear();
        }
        graphics::present(ctx)?;
        ggez::timer::yield_now();
        Ok(())
    }
}

fn get_gate(p1: u8, p2: u8) -> usize {
    if p1 >= b'A' && p1 <= b'Z' && p2 >= b'A' && p2 <= b'Z' {
        return 1 + ((p1 as i32 - b'A' as i32) * 32 + (p2 as i32 - b'A' as i32)) as usize;
    }
    return 0;
}

fn find_gate(data: &Vec<u8>, pos: usize, stride: usize) -> usize{
    if data[pos] == b'.' {
        let up = get_gate(data[pos - 2 * stride], data[pos - stride]);
        let left = get_gate(data[pos - 2], data[pos - 1]);
        let right = get_gate(data[pos + 1], data[pos + 2]);
        let down = get_gate(data[pos + stride], data[pos + 2 * stride]);
        return up | left | right | down;
    }
    return 0;
}

fn load_board(filename: &String) -> io::Result<GameBoard> {
    let mut data: Vec<u8> = Vec::new();
    let mut file = File::open(filename)?;
    file.read_to_end(&mut data)?;
    let mut board : GameBoard = Default::default();
    let mut px = 0;
    let mut py = 0;
    let mut line = vec![];
    for p in 0..data.len() {
        if data[p] == b'\n' {
            if board.width == 0 {
              board.width = p - 4;
            }
            if line.len() > 0 {
                board.map.push(line);
                line = vec![];
            }
            py += 1; px = 0;
        } else {
            // real pixel!
            if px > 1 && py > 1 && px - 2 < board.width {
                line.push(data[p]);
            }
            px += 1;
        }
    }
    board.map.pop();
    board.height = board.map.len();
    println!("Board size: {}x{}", board.width, board.height);
    let mw = board.width + 5;
    let mut gates = vec![0; 1024];
    for p in mw*2..data.len()-mw*2 {
        let px = p % mw;
        let py = p / mw;
        let gid = find_gate(&data, p, mw);
        if gid != 0 {
            let l = String::from_utf8(vec![(((gid - 1) / 32) as u8 + b'A'), (((gid - 1) % 32) as u8 + b'A')]).unwrap();
            let p1 = Point{x: px as i32 - 2, y: py as i32 - 2};
            println!("gate {} ({}) at {:?}", gid, l, p1);
            board.labels.insert(p1.clone(), l);
            if gates[gid] != 0 {
                let lx = gates[gid] % mw;
                let ly = gates[gid] / mw;                
                let p2 = Point{x: lx as i32 - 2, y: ly as i32 - 2};
                println!("Portal {:?} <=> {:?}", p1, p2);
                board.map[p1.y as usize][p1.x as usize] = b'o';
                board.map[p2.y as usize][p2.x as usize] = b'o';
                board.portals.insert(p1.clone(), p2.clone());
                board.portals.insert(p2, p1);
            } else if gid == 1 {
                println!("Start at {:?}", p1);
                board.start = p1;
            } else if gid == 826 {
                println!("End at {:?}", p1);
                board.end = p1;
            }
            gates[gid] = p;
        }
    }
    let v : Vec<Point> = board.labels.keys().cloned().collect();
    for l in v {
        board.scan(&l);
    }
    Ok(board)
}

pub fn main() -> GameResult {
    let args: Vec<String> = env::args().collect();
    let board = load_board(&args[1])?;
    /*
    let cb = ggez::ContextBuilder::new("super_simple", "ggez")
        .window_setup(ggez::conf::WindowSetup::default().title("Advent of Code"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(
            (board.width * CELL_SIZE.0 as usize) as f32, (board.height * CELL_SIZE.1 as usize) as f32));
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(board)?;
    event::run(ctx, event_loop, state)
    */
    board.solve();
    Ok(())
}
