use std::io;
use std::io::Write;
use rand::Rng;
use std::cmp::Ordering;


fn main() 
{

    let secret: u64 = rand::thread_rng().gen_range(1..101);

    println!("The secret number is {}", secret);
    let mut counter: u64 = 1;

    loop
    {
        if counter > 10
        {
            println!("\tYou have used all your chances!");
            println!("\tGame Over");
            break;
        }

        print!("Please input a number: ");
        io::stdout().flush().expect("Could not flush");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u64 = match guess.trim().parse::<u64>()
        {
            Ok(number) => number,
            Err(error) =>
            {
                println!("\tFailed with error: {}", error);
                counter = counter + 1;
                println!("\tCongratulations you wasted a chance");
                println!("\tYou have used [{}/10] chances", counter);
                continue;
            },
        };
        
        match guess.cmp(&secret)
        {
            Ordering::Equal => 
            {
                println!("\tYou Win!");
                break;
            },
            Ordering::Less => { println!("\tYou guessed too less") },
            Ordering::Greater => { println!("\tYou guessed too High") }
        }
        println!("\tYou have used [{}/10] chances", counter);
        counter = counter + 1;
    }
}
