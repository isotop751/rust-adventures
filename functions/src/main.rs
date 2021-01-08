use std::io;
use std::io::Write;

fn main() {

    println!("Type exit or quit to quit the program");
    loop
    {
        print!("Factorial of: ");
        io::stdout().flush().expect("Failed to flush Display");

        let mut number: String = String::new();
        io::stdin().read_line(&mut number).expect("Could not read input");

        if ["exit", "quit"].contains(&(&number.trim().to_lowercase()[..]))
        {
            break;
        }

        let number: u64 = match number.trim().parse::<u64>()
        {
            Ok(number) => number,
            Err(error) =>
            {
                println!("Failed with error: {}", error);
                println!("That does not seem like a number ...");
                continue;
            },
        };

        println!("Factorial of {} is {}", number, factorial(number));
    }

}

fn factorial(number: u64) -> u64
{
    if number == 1 || number == 0
    {
        return 1;
    }
    else
    {
        return number * factorial(number - 1);
    }
}
