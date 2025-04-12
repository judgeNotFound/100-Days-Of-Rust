use std::io;

fn main() {
    loop {
        println!("How many days did you run? ");
        let mut days: String = String::new();

        let _ = io::stdin()
            .read_line(&mut days);

        if days == "Stop" {
            return;
        }

        let days: u32 =  match days.trim().parse() {
            Ok(num) => num, 
            Err(_) => continue,
        };

        println!("Put in the km per day ran:");

        let mut day_nums: Vec<u32> = Vec::new();
        for _ in 0..days {
            let mut input: String = String::new();
            let _ = io::stdin().
                read_line(&mut input);

            let input: u32 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            day_nums.push(input);
        }

        let mut progress_days = 0;
        let mut prev_day = day_nums[0];
        for idx in 1..day_nums.len() {
            if prev_day < day_nums[idx] {
                progress_days += 1;
            }
            prev_day = day_nums[idx];
        }

        println!("Johnny had {} progress days.", progress_days);
    }
}
