use std::io;
use std::collections::HashMap;  

fn paint(p: i32, map: &mut HashMap<i64, i32>) {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Failure");
    let split = buf.trim().split(",");
    let mut cx : i32 = 50000;
    let mut cy : i32 = 50000;
    let mut td : i32 = 0;
    let mut minx : i32 = 0;
    let mut miny : i32 = 0;
    let mut mind : i32 = 100000;
    for elem in split {
        let (dir, dst) = elem.split_at(1);
        let mut val = dst.parse::<i32>().expect("Conversion failure");
        let mut mx : i32 = 0;
        let mut my : i32 = 0;
        match dir {
            "D" => my = 1,
            "U" => my = -1,
            "L" => mx = -1,
            "R" => mx = 1,
            _ => panic!("Unknown direction")
        }
        while val > 0 {
            cx = cx + mx;
            cy = cy + my;
            td += 1;
            let key = cx as i64 * 100000 + cy as i64;
            if map.contains_key(&key) && p > 1 {
                let pd = map[&key] + td;
                println!("Intersection: {}x{} d={}",cx,cy,pd);
                if pd < mind { mind = pd; }
            }
            if !map.contains_key(&key) && p < 2 { 
                map.insert(key, td);
            }
            val -= 1;
        }
        if cx < minx { minx = cx; }
        if cy < miny { miny = cy; }
    }
    println!("Final position: {}x{}, Total distance: {}, Min Intersection: {}", cx, cy, td, mind);
}

fn main() {
  let mut visited : HashMap<i64, i32> = HashMap::new();
  paint(1, &mut visited);
  paint(2, &mut visited);
}
