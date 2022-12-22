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

fn day4() {
    let reader = load_file("src/day4_input.txt");
    let mut full_overlaps = 0; 
    let mut partial_overlaps: i32 = 0;
    for line in reader.lines() {
        match line {
            Ok(val) => {
                let chunks: Vec<&str> = val.split(',').collect();
                let first: Vec<i32> = chunks[0].split('-').map(|s| s.parse().unwrap()).collect();
                let second: Vec<i32> = chunks[1].split('-').map(|s| s.parse().unwrap()).collect();
                
                if (first[0] >= second[0] && first[1] <= second[1]) || (first[0] <= second[0] && first[1] >= second[1]) {
                    full_overlaps += 1;
                }
                else if (first[0] >= second[0] && first[1] >= second[1] && first[0] <= second[1]) ||
                        (first[0] <= second[0] && first[1] <= second[1] && first[1] >= second[0])  {
                    partial_overlaps += 1;
                }
            },
            Err(e) => println!("{e}")
        }
    }
    println!("Full overlaps: {full_overlaps}");
    println!("Total overlaps: {}", full_overlaps + partial_overlaps);
}

fn day5() {
    let reader = load_file("src/day5_input.txt");

    let mut stacks = Vec::new(); 
    let num_stacks = 9;
    let mut is_initialised: bool = false; 

    for i in 0..num_stacks{
        let v: Vec<char> = Vec::new();
        stacks.push(v);
    }

    for line in reader.lines() {
        let val = line.unwrap();
        if val == "" {
            continue;
        }
        // initialise stacks
        if !is_initialised {
            for i in 0..num_stacks {
                let c = val.as_bytes()[1 + i * 4] as char; 
                if c == '1' {
                    is_initialised = true; 
                    break
                }
                else if c != ' ' {
                    stacks[i].insert(0, c); 
                }
            
            }
        }
        else {
            // apply moves
            // println!("{}", val);
            let chunks: Vec<&str> = val.split(' ').collect();
            let num_crates: i32 = chunks[1].parse().unwrap();
            let origin: usize  = chunks[3].parse::<usize>().unwrap() - 1;
            let destination: usize  = chunks.last().unwrap().parse::<usize>().unwrap() - 1;

            // part 1
            // for _ in 0..num_crates {
            //     let item = stacks[origin].pop().unwrap(); 
            //     stacks[destination].push(item); 
            // }

            // part 2
            for i in 0..num_crates {
                let idx = stacks[origin].len() - num_crates as usize + i as usize;
                let item = stacks[origin][idx]; 
                stacks[destination].push(item); 
            }

            for _ in 0..num_crates {
                stacks[origin].pop().unwrap();
            }

        }
        
    }
    println!("Top items:"); 
    for i in 0..num_stacks {
        let top = stacks[i].pop().unwrap();
        print!("{top}");
    }
}

fn main() {
    
}