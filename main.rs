use std::io::{self, Write};
use std::process;

mod blockchain;

fn main() {
    let mut miner_addr = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    // Input miner address
    print!("Input a miner address: ");
    io::stdout().flush().expect("Error flushing stdout");
    io::stdin().read_line(&mut miner_addr).expect("Failed to read line");

    // Input difficulty
    print!("Difficulty: ");
    io::stdout().flush().expect("Error flushing stdout");
    io::stdin().read_line(&mut difficulty).expect("Failed to read line");

    let diff = difficulty
        .trim()
        .parse::<u32>()
        .expect("Difficulty must be an integer");

    println!("Generating genesis block!");
    let mut chain = blockchain::Chain::new(miner_addr.trim().to_string(), diff);

    loop {
        // Menu
        println!("Menu");
        println!("1) New Transaction");
        println!("2) Mine block");
        println!("3) Change Difficulty");
        println!("4) Change Reward");
        println!("0) Exit");
        print!("Enter your choice: ");
        io::stdout().flush().expect("Error flushing stdout");
        
        choice.clear();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        println!("");

        match choice.trim().parse::<u32>() {
            Ok(0) => {
                println!("Exiting!");
                process::exit(0);
            }
            Ok(1) => {
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                print!("Enter sender address: ");
                io::stdout().flush().expect("Error flushing stdout");
                io::stdin().read_line(&mut sender).expect("Failed to read line");

                print!("Enter receiver address: ");
                io::stdout().flush().expect("Error flushing stdout");
                io::stdin().read_line(&mut receiver).expect("Failed to read line");

                print!("Enter amount: ");
                io::stdout().flush().expect("Error flushing stdout");
                io::stdin().read_line(&mut amount).expect("Failed to read line");

                let res = chain.new_transaction(
                    sender.trim().to_string(),
                    receiver.trim().to_string(),
                    amount.trim().parse().expect("Amount must be a valid float"),
                );

                if res {
                    println!("Transaction added");
                } else {
                    println!("Transaction failed");
                }
            }
            Ok(2) => {
                println!("Generating block...");
                let res = chain.generate_new_block();
                if res {
                    println!("Block generated successfully");
                } else {
                    println!("Block generation failed");
                }
            }
            Ok(3) => {
                let mut new_diff = String::new();
                print!("Enter new difficulty: ");
                io::stdout().flush().expect("Error flushing stdout");
                io::stdin().read_line(&mut new_diff).expect("Failed to read line");

                let res = chain.update_difficulty(new_diff.trim().parse().expect("Difficulty must be a valid integer"));
                if res {
                    println!("Updated difficulty");
                } else {
                    println!("Failed to update difficulty");
                }
            }
            Ok(4) => {
                let mut new_reward = String::new();
                print!("Enter new reward: ");
                io::stdout().flush().expect("Error flushing stdout");
                io::stdin().read_line(&mut new_reward).expect("Failed to read line");

                let res = chain.update_reward(new_reward.trim().parse().expect("Reward must be a valid float"));
                if res {
                    println!("Updated reward");
                } else {
                    println!("Failed to update reward");
                }
            }
            _ => println!("Invalid option, please retry"),
        }
    }
}
