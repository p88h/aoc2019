use std::io;
use std::collections::HashMap;  

struct Node {
    prev: String,
    direct: i32,
    indirect: i32,
    distance: i32,
    next: Vec<String>,
}

fn init_node<'a>(graph: &'a mut HashMap<String, Node>, id: &String) -> &'a mut Node {
    if !graph.contains_key(id) {
        let n = Node { prev: String::from(""), direct: 0, indirect: 0, distance: 0, next: vec![] };
        graph.insert(id.clone(), n);
    }
    return graph.get_mut(id).unwrap();
}

fn main() {
    let mut graph = HashMap::new();
    loop {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).expect("Failure");
        let split = buf.trim().split(")").collect::<Vec<&str>>();
        if split.len() < 2 { break; }
        let i = String::from(split[0]);
        let j = String::from(split[1]);
        let q = init_node(&mut graph, &i);
        q.direct += 1;
        q.next.push(j.clone());
        let r = init_node(&mut graph, &j);
        r.prev = i;
    }
    let mut stack = vec![String::from("COM")];
    let mut pos = 0;
    let mut total = 0;
    while pos < stack.len() {
        let id = stack.get(pos).unwrap().clone();
        let p = graph.get_mut(&id).unwrap();
        for k in &p.next {
            //println!("{}->{}", id, k);
            stack.push(k.clone());
        }
        pos = pos + 1;
    }
    while !stack.is_empty() {
        let id = stack.pop().unwrap();
        let p = graph.get(&id).unwrap();
        let tmp = p.indirect;
        total += tmp;
        if !p.prev.is_empty() {
          //println!("{}->{}", p.prev, p.id);
          let pid = p.prev.clone();
          let r = graph.get_mut(&pid).unwrap();
          r.indirect = r.indirect + tmp + 1;
        } 
    }
    println!("Total: {}", total);
    let you = String::from("YOU");
    let san = String::from("SAN");
    let mut ptr = you.clone();
    let mut dst = 0;
    while !ptr.is_empty() {
        let p = graph.get_mut(&ptr).unwrap();
        p.distance = dst;
        dst += 1;
        ptr = p.prev.clone();
    }
    ptr = san.clone();
    let mut dst = 0;
    while !ptr.is_empty() {
        let p = graph.get_mut(&ptr).unwrap();
        if p.distance > 0 {
            println!("Found {} at {} total {}",p.distance,dst,p.distance+dst-2);
            break;
        }
        dst += 1;
        ptr = p.prev.clone();
    }
  }
