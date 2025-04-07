use std::io;

fn is_vegetarian(skewer: String) -> bool {
    let chars = skewer.split("");

    for c in chars {
        if c == "x" {
            return false
        }
    }
    return true
}

fn main() {
    loop {
        println!("Please tell me how many skewers you have:");
        let mut num_skewers = String::new();
        let _ = io::stdin().read_line(&mut num_skewers);
        let num_skewers = match num_skewers.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let mut veggie: u32 = 0;
        let mut total: u32 = 0;

        for i in 0..num_skewers {
            println!("Skewer {i}: ");
            let mut skewer = String::new();
            let _ = io::stdin().read_line(&mut skewer);

            let skewer: String = match skewer.trim().parse() {
                Ok(s) => s,
                Err(_) => continue,
            };

            total += 1;
            if is_vegetarian(skewer) {
                veggie += 1;
            }
        }

        println!("[{}, {}]", veggie, total - veggie);
    }
}

