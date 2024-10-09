use std::io;
use rand::Rng;

fn main(){
    println!("welcome to the guessing game");
    println!("would you like to play a game ");

    let mut is_ready_to_play = false ;
    let max_trials = 5;
    let mut current_trial_num = 0;
    let mut secret_number = rand::thread_rng().gen_range(1..101);

    let mut play_verification = String::new();

    io::stdin()
    .read_line( &mut play_verification)
    .expect("failed to read line");

    if play_verification.trim().to_lowercase() == "yes"{
        is_ready_to_play = true;
    }else {
        is_ready_to_play = false;
    }

    while is_ready_to_play {
        if current_trial_num == 0 {
            println!("Guess a number between 1 and 100");
        }
        if current_trial_num >0 && current_trial_num < 3 {
            println!("Guess again");
        }
        if current_trial_num >= 3 && current_trial_num < 5 {
            println!("Heres a hint the number is between {} and {}", secret_number - 1, secret_number + 1);
        }
        if current_trial_num >= max_trials {
            println!("You've reached the maximum number of trials.");
            if play_again(){
                current_trial_num = 0;
                secret_number = rand::thread_rng().gen_range(1..101);
                continue;
            }else {
                println!("Thank you for playing \nGoodbye");
                break;
            }
        }
        let mut guess_value= String::new();

        io::stdin()
        .read_line(&mut guess_value)
        .expect("failed to read line");

        if guess_value.trim() == secret_number.to_string() {
            println!("Your guess was correct: {}", guess_value);
            if play_again(){
                current_trial_num = 0 ;
                secret_number = rand::thread_rng().gen_range(1..101);
                continue;
            }else {
                println!("Thank you for playing \nGoodbye");
                break;
            }
        }else {
            current_trial_num += 1;
            println!("Oppos that was wrong");
        }
    }

    fn  play_again() -> bool {
        println!("Would you like to play again");
        let mut play_verification = String::new();

        io::stdin()
        .read_line( &mut play_verification)
        .expect("failed to read line");
        
        if play_verification.trim().to_lowercase() == "yes"{
            return true;
        }else {
            return  false;
        }
    }
} 