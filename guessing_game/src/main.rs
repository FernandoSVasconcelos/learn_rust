use rand::Rng;
use std::io;

fn main() 
{
    println!("Guess the number!\n");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut x = 0;
    loop
    {
        x = x + 1;
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line\n");

        let guess: u32 = match guess.trim().parse() 
        {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess > secret_number
        {
            println!("Too big!\n");
        }
        else if guess < secret_number
        {
            println!("Too small!\n");
        }
        else if guess == secret_number
        {
            println!("Correct answer on the {} try!", x);
            println!("The secret number is {}\n", secret_number);
            break;
        }
    } 
}