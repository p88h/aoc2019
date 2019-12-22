use std::io;
use std::env;
use std::boxed::Box;

const DECK_SIZE : usize = 119_315_717_514_047;
const TRACE : usize = 2020;

fn mod_inverse(a: i128, base: i128) -> i128 {
    let mut exp = base - 2;
    let mut mul = a;
    let mut ret = 1;
    while exp > 0 {
        if exp % 2 == 1 {
            ret = (ret * mul) % base;
        }
        exp /= 2;
        mul = (mul * mul) % base;
    }
    return ret;
}

trait Deck {
    fn reverse(&mut self);
    fn cut(&mut self, shift: usize);
    fn deal(&mut self, ofs: usize);
    fn find(&mut self) -> usize;
}

#[derive(Debug, Default)]
struct SlowDeck {
    cards: Vec<usize>,
    trace: usize,
    size: usize
}

impl SlowDeck {
    fn new(size: usize, trace: usize) -> SlowDeck {
        let mut cards = vec![0; size];
        for i in 0..size {
            cards[i] = i;
        }
        SlowDeck{ cards: cards, trace: trace % size, size: size }
    }
}

impl Deck for SlowDeck {
    fn reverse(&mut self) {
        for i in 0..self.size/2 {
            self.cards.swap(i, (self.size - i) - 1);
        }
    }
    fn cut(&mut self, shift: usize) {
        let mut tmp = vec![0; self.size];
        for i in 0..self.size {
            tmp[(i + shift) % self.size] = self.cards[i];
        }
        self.cards = tmp;
    }   
    fn deal(&mut self, ofs: usize) {
        let mut tmp = vec![0; self.size];
        for i in 0..self.size {
            tmp[(i * ofs) % self.size] = self.cards[i];
        }
        self.cards = tmp;
    }
    fn find(&mut self) -> usize {
        for i in 0..self.size {
            if self.cards[i] == self.trace { return i }
        }
        return 0;
    }
}

// represents only the traced card position
#[derive(Debug, Default)]
struct FastDeck {
    trace: usize,
    size: usize,
    value: usize
}

impl FastDeck {
    fn new(size: usize, trace: usize) -> FastDeck {
        FastDeck{ trace: trace % size, size: size, value: trace % size }
    }
}

impl Deck for FastDeck {
    fn reverse(&mut self) {
        self.trace = (self.size - self.trace) - 1;
    }
    fn cut(&mut self, shift: usize) {
        self.trace = (self.trace + shift) % self.size;
    }
    fn deal(&mut self, ofs: usize) {
        self.trace = (self.trace * ofs) % self.size;
    }
    fn find(&mut self) -> usize {
        return self.trace;
    }
}

// represent operations as f(x) = a*x + b
#[derive(Debug, Default, Clone)]
struct PolyDeck {
    trace: i64,
    size: i128,
    a: i128,
    b: i128,
}

impl PolyDeck {
    fn new(size: usize, trace: usize) -> PolyDeck {
        PolyDeck{ trace: trace as i64 % size as i64, size: size as i128, a: 1, b: 0 }
    }

    // only really needed at the end
    fn norm(&mut self) {
        if self.a < 0 { self.a += self.size; }
        if self.b < 0 { self.b += self.size; }
        // println!("a: {} b: {}", self.a, self.b);
    }

    // f[i](x)= a*f[i-1](x)+b =>
    // f[i](x)= i%2 ? a*f[i-1](x)+b : f[i/2](f[i/2](x))
    fn apply(&self, iter: usize) -> (i128, i128) {
        if iter == 0 { return (1,0) }
        if iter == 1 { return (self.a, self.b) }
        let (a1, b1) = self.apply(iter / 2);
        // a1*(a1*x + b1)+b1
        let mut a2 = (a1 * a1) % self.size;
        let mut b2 = (a1 * b1 + b1) % self.size;
        // a*(a2*x + b2)+b
        if iter % 2 == 1 {
            a2 = (self.a * a2 ) % self.size;
            b2 = (self.a * b2 + self.b ) % self.size;
        }
        return (a2, b2);
    }

    // a*x+b=pos => pos-b=a*x => x=(pos-b)/x (mod size)
    fn compute(&self, iter: usize, pos: usize) -> usize {
        // linear inversion...
        let (a, b) = self.apply(iter);
        // println!("({}, {})^{} => ({}, {})", self.a, self.b, pow, a, b);
        let inv = mod_inverse(a, self.size);
        // println!("inv({}) => {}", self.size, inv);
        let mut fv = ((pos as i128 - b) * inv) % self.size;
        if fv < 0 { fv += self.size; }
        // println!("({} - {}) x {} % {} => {}", pos, b, inv, self.size, fv);
        return fv as usize;
    }
}

impl Deck for PolyDeck {
    fn reverse(&mut self) {
        self.a = -self.a % self.size;
        self.b = (-self.b - 1) % self.size;
        self.norm();
    }
    fn cut(&mut self, shift: usize) {
        self.b = (self.b + shift as i128) % self.size;
        self.norm();
    }
    fn deal(&mut self, ofs: usize) {
        self.a = (self.a * ofs as i128) % self.size;
        self.b = (self.b * ofs as i128) % self.size;
        self.norm();
    }
    fn find(&mut self) -> usize {
        let mut fv = ((self.a*self.trace as i128)+self.b) % self.size;
        // println!("({} * {} + {}) % {} = {}",self.a,self.trace,self.b,self.size, fv);
        // update trace position
        self.trace = fv as i64;
        if fv < 0 { fv += self.size; }
        return fv as usize;
    }
}

fn auto_deck(size: usize, trace: usize) -> Box<dyn Deck> {
    if size < 100000 {
        Box::new(SlowDeck::new(size, trace))
    } else {
        Box::new(FastDeck::new(size, trace))
    }
}

trait Op {
    fn apply(&self, deck: &mut dyn Deck);
}
struct Reverse {}
impl Op for Reverse {
    fn apply(&self, deck: &mut dyn Deck) {
        deck.reverse();
    }
}
struct Cut {
    shift: usize
}
impl Op for Cut {
    fn apply(&self, deck: &mut dyn Deck) {
        deck.cut(self.shift);
    }
}
struct Deal {
    ofs: usize
}
impl Op for Deal {
    fn apply(&self, deck: &mut dyn Deck) {
        deck.deal(self.ofs);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut trace = TRACE;
    let mut size = DECK_SIZE;
    if args.len() > 1 {
        size = args[1].parse().expect("Not a number");
        trace = trace % size;
    }
    if args.len() > 2 {
        trace = args[2].parse().expect("Not a number");
    }
    println!("Deck size: {} trace card: {}", size, trace);
    let mut ops : Vec<Box<dyn Op>> = vec![];
    loop {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).expect("Failure");
        let split = buf.trim().split(" ").collect::<Vec<&str>>();
        if split.len() < 2 { break; }
        if split[0] == "deal" && split[1] == "into" {
            ops.push(Box::new(Reverse{}));
        } else if split[0] == "cut" {
            let mut v : i64 = split[1].parse().expect("Not a number");
            if v < 0 { v = size as i64 + v; }
            let uv = size - v as usize;
            ops.push(Box::new(Cut{shift: uv}));
        } else if split[0] == "deal" && split[1] == "with" {
            let k : usize = split[split.len() - 1].parse().expect("Not a number");
            ops.push(Box::new(Deal{ofs: k}));
        }
    }
    let mut deck = auto_deck(size, trace);
    let mut poly = PolyDeck::new(size, trace);
    // Compute poly coefficients once
    for op in &ops {
        op.apply(&mut poly);
    }
    for i in 0..10 {
        // Apply regular operations every time to the normal deck
        for op in &ops {
            op.apply(deck.as_mut());
        }
        let pos = deck.find();
        println!("Result[{}]: {}", i + 1, pos);
        println!("Poly-F[{}]: {}", i + 1, poly.find());
        println!("Poly-C[{}]: {}", pos, poly.compute(i+1, pos));
        println!("");
    }
    let iter = 101_741_582_076_661;
    println!("Poly[{}]= {}", iter, poly.compute(iter, trace));
}
