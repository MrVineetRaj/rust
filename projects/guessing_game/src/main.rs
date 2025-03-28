use rand::Rng;
use std::io;

fn main() {
    
    let guess_list:[String;5] = ["apple".to_string(), "banana".to_string(), "cherry".to_string(), "date".to_string(), "elderberry".to_string()];
    let mut rng = rand::thread_rng();

    let mut guess_matched = true;
    let mut computer_guess =  String::new();

    loop {

        if guess_matched == true {
            let  random_index = rng.gen_range(0..guess_list.len());
            computer_guess = guess_list[random_index].clone();
            guess_matched = false;
            println!("Enter your guess {} . \nTo exit type exit.",computer_guess);
        }

        let mut user_guess = String::new();
        io::stdin()
            .read_line(&mut user_guess)
            .expect("Failed to read line");
        

        println!("{}",user_guess);
        if user_guess.trim() == "exit" {
            break;
        }

        
        if user_guess.trim() == computer_guess {
            println!("Perfect match !!!");
            guess_matched = true;
        }
        else if user_guess != computer_guess {
            println!("Oops incorrect guess!!!");
            println!("To retry write your guess. \nTo exit the game write exit.");
        }

    }
}
