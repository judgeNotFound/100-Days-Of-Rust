use std::io;
use std::collections::HashMap;

fn main() {
    let mut socks: String = String::new();
    println!("Please input your socks.");
    io::stdin().read_line(&mut socks).expect("Could not read from stdin");

    let mut socks_drawer = HashMap::new();

    for c in socks.chars() {
        let num = match socks_drawer.get(&c) {
            Some(num) => num + 1,
            None => 1,
        };
        
        socks_drawer.insert(c, num);
    }

    let mut count: u32 = 0;
    for (_sock, sock_count) in &socks_drawer {
        count += (sock_count / 2) as u32;
    }

    println!("Those are {} sock.", count);
}
