use std::io;

fn find_nemo(sentence: String) -> String {
    let seq = sentence.split(" ");

    let mut idx = 1;
    for val in seq {
        if val == "Nemo" {
            return format!("I found Nemo at {}!", idx);
        }

        idx = idx + 1;
    }

    return String::from("I can't find Nemo :(");
}

fn main() {

    loop {

        println!("Please input the sentence:");

        let mut sentence = String::new();
        
        io::stdin()
            .read_line(&mut sentence)
            .expect("Could not read sentence. Make sure an integer is used.");
        
        let sentence: String = match sentence.trim().parse() {
            Ok(s) => s, 
            Err(_) => continue,
        };

        println!("{}", find_nemo(sentence));
    }

}
