use std::{io::{self, Write}, time::SystemTime};

mod command;
mod input;

use command::{Command, parse};

fn main() {
    loop {
        print!("Astra>>> ");
        io::stdout().flush().unwrap();

        let input = input::read_input();
        let command = parse(&input);

        match command {
            Command::Hello => {
                println!("Hello. ASTRA is online.");
            }

            Command::Status => {
                println!("Core operational.");
            }

            Command::Time => {
                let sys_time = SystemTime::now();
                println!("{:?}", sys_time);
            }

            Command::About => {
                println!("I am ASTRA ( Autonomous System for Task, Reasoning & Assistance ). A personal AI assistant/friend.")
            }

            Command::NextLine =>{}

            Command::Help => {
                println!("Available commands:");
                println!("hello");
                println!("status");
                println!("help");
                println!("exit");
            }

            Command::Exit => {
                println!("Good By...");
                break;
            }

            Command::Unknown(value) => {
                println!("Unknown command: {}", value);
            }
        }
    }
}
