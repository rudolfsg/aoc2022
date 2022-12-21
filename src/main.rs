use std::{collections::BinaryHeap, collections::HashMap, fs::File, io::BufRead, io::BufReader};

fn load_file(filepath: &str) -> BufReader<File> {
    let file = File::open(filepath).unwrap(); // .expect("File open failed");
    let reader = BufReader::new(file);
    reader
}

fn day1() {
    let filepath = "src/day1_input.txt";
    let reader = load_file(filepath);

    let mut calories = BinaryHeap::from([0, 0, 0]);
    let mut counter: i32 = 0;

    for line in reader.lines() {
        let value = line.unwrap();
        if value == "" {
            let current_min: i32 = *calories.peek().unwrap();
            if counter > -1 * current_min {
                calories.pop();
                // use negative values to obtain a min heap
                calories.push(-1 * counter);
            }
            counter = 0;
        } else {
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

fn day2() {
    let reader = load_file("src/day2_input.txt");
    // would be nice to use a circular datastructure
    // but rust ownership structure makes it problematic
    let move_scores = [1, 2, 3]; // rock paper scissors
    let outcome_scores = [0, 3, 6]; // lose draw win

    let mut total_score = 0;

    // part 1
    // for line in reader.lines(){
    //     let val = line.unwrap();
    //     let my_move = val.as_bytes()[2] - 88;
    //     let opponent_move = val.as_bytes()[0] - 65;

    //     total_score += move_scores[my_move as usize];

    //     if my_move == opponent_move {
    //         // draw
    //         total_score += 3
    //     }
    //     else if (my_move == 0 && opponent_move == 2) ||
    //             (my_move == 1 && opponent_move == 0) ||
    //             (my_move == 2 && opponent_move == 1) {
    //         total_score += 6
    //     }
    // }
    // part 2
    for line in reader.lines() {
        let val = line.unwrap();
        let outcome = val.as_bytes()[2] - 88;
        let opponent_move = val.as_bytes()[0] - 65;

        total_score += outcome_scores[outcome as usize];

        let mut my_move: u8;

        if outcome == 1 {
            my_move = opponent_move; // draw
        } else if outcome == 2 {
            // win
            my_move = (opponent_move + 1) % 3;
        } else {
            // lose
            if opponent_move == 0 {
                my_move = 2;
            } else {
                my_move = opponent_move - 1;
            }
        }

        total_score += move_scores[my_move as usize];
    }
    println!("Total score: {total_score}");
}

fn day3() {
    let mut reader = load_file("src/day3_input.txt");
    let mut total_priority: i32 = 0;
    // part 1
    // for line in reader.lines(){
    //     let val = line.unwrap();
    //     let first = &val[..val.len()/2].as_bytes();
    //     let second = &val[val.len()/2..].as_bytes();

    //     let mut common_item: i32 = 0;
    //     for item in first.into_iter() {
    //         if second.contains(item){
    //             common_item = item.clone() as i32;
    //             break
    //         }
    //     }
    //     // println!("{common_item}");
    //     if common_item < 91 {
    //         common_item += (26 - 64);
    //     } else {
    //         common_item -= 96;
    //     }
    //     total_priority += common_item;

    //  }

    loop {
        let mut first = String::new();
        let mut second = String::new();
        let mut third = String::new();

        let bytes = reader.read_line(&mut first).unwrap();
        if bytes == 0 {
            println!("EOF reached");
            break;
        }

        reader.read_line(&mut second);
        reader.read_line(&mut third);

        let mut common_item: i32 = 0;

        for item in first.as_bytes().into_iter() {
            if second.as_bytes().contains(item) && third.as_bytes().contains(item) {
                common_item = item.clone() as i32;
                break;
            }
        }

        if common_item < 91 {
            common_item += (26 - 64);
        } else {
            common_item -= 96;
        }
        total_priority += common_item;
    }

    println!("Total priority: {total_priority}")
}

fn main() {
    
}
