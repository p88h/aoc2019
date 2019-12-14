use std::io;
use std::collections::HashMap;  

struct Chem {
    prod: i64,
    deps: HashMap<String, i64>
}

fn transmogrify(graph: &HashMap<String, Chem>, stock: &mut HashMap<String, i64>, n: &String, v: i64) -> i64 {
    if n == "ORE" { return v; }
    let mut o = 0;
    let c = graph.get(n).unwrap();
    let s = stock.get_mut(n).unwrap();
    let r = (v - *s + c.prod - 1) / c.prod;
    *s += r * c.prod - v;
    if r == 0 { return 0 };
    for (d,q) in c.deps.iter() {
        o += transmogrify(graph, stock, d, r * q);
    }
    return o;
}

fn main() {
    let mut graph = HashMap::new();
    let mut stock = HashMap::new();
    loop {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).expect("Failure");
        let split = buf.trim().split(" ").collect::<Vec<&str>>();
        if split.len() < 2 { break; }
        let mut deps = HashMap::new();
        let mut si = 0;
        loop {
            if split[si] == "=>" {
                let v : i64 = split[si + 1].parse().expect("Not a number");
                let n : String = split[si + 2].into();
                stock.insert(n.clone(), 0);
                graph.insert(n, Chem { prod: v, deps: deps});     
                break;     
            } else {
                let v : i64 = split[si].parse().expect("Not a number");
                let n : String = split[si + 1].replace(",", "");
                deps.insert(n, v);
                si += 2;
            }
        }
    }
    let f = String::from("FUEL");
    let mut cap = 1000000000000;
    let mut n = 1;
    let mut tot = 0;
    while cap > 0 {
        let p = transmogrify(&graph, &mut stock, &f, n);
        tot += n;
        println!("ORE for {} fuel: {} (cap {}, total fuel {})", n, p, cap, tot);
        cap -= p;
        if n == 1 { n = cap / 1582325 } else { n = n * (cap / p); }
        if n == 0 { n = 1; }
    }
}
