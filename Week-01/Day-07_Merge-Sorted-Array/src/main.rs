use std::io;

fn scan_array() -> Vec<i32> {
    println!("How many integers does you array have?");

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Could not read from stdin.");

    let input: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    let mut numbers: Vec<i32> = Vec::new();
    for _ in 0..input {
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Could not read number from stdin.");
        let result = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };
        numbers.push(result);
    }

    numbers
}

fn main() {
    loop {
        let num1 = scan_array();
        let num2 = scan_array();

        println!("Num1: {:?}", num1);
        println!("Num2: {:?}", num2);

        let mut merged_nums: Vec<i32> = Vec::new();

        let mut num1_idx: usize = 0;
        let mut num2_idx: usize = 0;

        while num1_idx < num1.len() || num2_idx < num2.len() {
            if num1_idx < num1.len() && num2_idx < num2.len() {
                if num1[num1_idx] < num2[num2_idx] {
                    merged_nums.push(num1[num1_idx]);
                    num1_idx += 1;
                } else {
                    merged_nums.push(num2[num2_idx]);
                    num2_idx += 1;
                }
            } else {
                if num1_idx < num1.len() {
                    for i in num1_idx..num1.len() {
                        merged_nums.push(num1[i]);
                    }
                    break;
                } else {
                    for i in num2_idx..num2.len() {
                        merged_nums.push(num2[i]);
                    }
                    break;
                }
            }
        }

        println!("Result: {:?}", merged_nums);
    }
}   
