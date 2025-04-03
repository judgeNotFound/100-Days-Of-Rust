use std::io;

fn calc_age(age: u32) -> u32 {
    age * 365
}

fn main() {

    loop {

    println!("Please input your age in years!");

    let mut guess = String::new();

    
        io::stdin()
            .read_line(&mut guess)
            .expect("Could not read age. Make you an integer is used.");
        
        let guess: u32 =  match guess.trim().parse() {
            Ok(num) => num, 
            Err(_) => continue,
        };

        let days = calc_age(guess);

        println!("You are roughly {days} days old!")

    }

}
