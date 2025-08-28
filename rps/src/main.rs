use std::io;
use rand::Rng;

fn main() {
    let mut score_human = 0;
    let mut score_comp = 0;
   
    println!("Enter number of rounds");
    let mut rounds = String::new();
    
    io::stdin()
        .read_line(&mut rounds)
        .expect("Failed to read input");

    let trimmed = rounds.trim();

    let number: i32 = trimmed.parse().expect("Invalid");


    for i in 0..number{
        play(&mut score_human, &mut score_comp, number);
    }
    
    println!("Final scores: comp: {score_comp}, human: {score_human}");
    
}

fn play( score_human: &mut i32,score_comp: &mut i32,  rounds: i32) -> (i32, i32) {


    println!("Enter a move: r for rock, p for paer, s for scissors");
    let mut choice = String::new();

    io::stdin()
    .read_line(&mut choice)
    .expect("Failed to read line");


    let comp_choice_num: u32 = rand::thread_rng().gen_range(1..4);
    let comp_choice = match comp_choice_num {
        1 => "Rock",
        2 => "Paper",
        3 => "Scissors",
        _ => "Rock" // fallback, though this should never happen with range 1..4
    };

    let refined_choice = match choice.trim() {
        "r"|"R" => "Rock",
        "p"|"P" => "Paper",
        "s"|"S" => "Scissors",
        _ => "Invalid"
    };

    if refined_choice == "Rock" {
        if comp_choice == "Paper"{
            *score_human += 1;
            println!("Win!")
        }
        else {
            *score_comp += 1;
            println!("Lose!")
        }
    }

    if refined_choice == "Paper" {
        if comp_choice == "Scissors"{
            *score_human += 1;
            println!("Win!")
        }
        else {
            *score_comp += 1;
            println!("Lose!")
        }
    }

    if refined_choice == "Scissors" {
        if comp_choice == "Rock"{
            *score_human += 1;
            println!("Win!")
        }
        else {
            *score_comp += 1;
            println!("Lose!")
        }
    }

    (*score_human, *score_comp)
}