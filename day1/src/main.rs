use std::io;
fn main() {
    println!("Hello, world!");
    let mut total : i64 = 0;
    loop {
      let mut buf = String::new();
      io::stdin().read_line(&mut buf).expect("Failure");
      if buf.is_empty() {
          break;
      }
      //println!("Buf: {}", buf);
      let num : i64 = buf.trim().parse().expect("Not a number");
      let mut fuel = (num / 3) - 2;
      while fuel > 0 {
        total += fuel;
        fuel = (fuel / 3) - 2;
      }
      println!("Weight: {} Fuel: {} Total: {}", num, fuel, total);
    }
    println!("Total: {}", total);
}
