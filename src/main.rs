use std::{
    fs::File,
    io::BufReader,
    io::BufRead,
    collections::BinaryHeap,
};

fn main() {
    let filepath = "src/day1_input.txt";
    let file = File::open(filepath).unwrap(); // .expect("File open failed");
    let reader = BufReader::new(file);

    let mut calories = BinaryHeap::from([0,0,0]);
    let mut counter: i32 = 0;

    for line in reader.lines() {
        let value = line.unwrap();
        if value == "" {
            let current_min: i32  = *calories.peek().unwrap();
                if counter > -1 * current_min {
                    calories.pop();
                    calories.push(-1 * counter);
                }
            counter = 0;
        }
        else {
            let c: i32 = value.parse().unwrap();
            counter = counter + c;
        }
    }
    
    let mut total_calories = 0;
    let mut c = 0;
    while calories.len() > 0 {
        c = -1 * calories.pop().unwrap();
        total_calories = total_calories + c;
        
    }
    println!("Max calories per elf: {c}");
    println!("Total calories: {total_calories}");

}