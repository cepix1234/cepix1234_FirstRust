use rand::Rng;
use std::io::{stdin};


static  MIN_GUESS: i32 = 0;
static  MAX_GUESS: i32 = 100;

fn main() {
    let mut guess_question: String = String::from("Please enter the number you want to guess (min: ");
    guess_question.push_str(&MIN_GUESS.to_string());
    guess_question.push_str(", max: ");
    guess_question.push_str(&MAX_GUESS.to_string());
    guess_question.push_str(")");

    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(MIN_GUESS,MAX_GUESS);

    let mut guesed_number: i32 = -1;
    let input_line = &mut String::new();

    while guesed_number != random_number {
        println!("{}", guess_question);
        input_line.clear();
        let _ = stdin().read_line(input_line);
        guesed_number = input_line.trim().parse::<i32>().unwrap();

        if guesed_number < random_number {
            println!("The number is higher");
        } else if guesed_number > random_number {
            println!("The number is lower");
        }
    }
    let mut win_message : String = String::from("Congradulations the number to guess was ");
    win_message.push_str(&random_number.to_string());
    println!("{}", win_message);
}
