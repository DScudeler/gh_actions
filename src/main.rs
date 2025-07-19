use clap::{Arg, Command};
use gh_actions::{User, UserManager, calculate_fibonacci, validate_email};

fn main() {
    let matches = Command::new("gh_actions")
        .version("0.1.0")
        .author("DScudeler")
        .about("A sample Rust application for testing GitHub Actions")
        .subcommand(
            Command::new("user")
                .about("User management operations")
                .subcommand(
                    Command::new("add")
                        .about("Add a new user")
                        .arg(Arg::new("id").required(true).help("User ID"))
                        .arg(Arg::new("name").required(true).help("User name"))
                        .arg(Arg::new("email").required(true).help("User email")),
                )
                .subcommand(Command::new("list").about("List all users")),
        )
        .subcommand(
            Command::new("fib")
                .about("Calculate Fibonacci number")
                .arg(Arg::new("number").required(true).help("Number to calculate")),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("user", user_matches)) => handle_user_command(user_matches),
        Some(("fib", fib_matches)) => handle_fib_command(fib_matches),
        _ => {
            println!("Welcome to gh_actions!");
            println!("Use --help to see available commands.");
        }
    }
}

fn handle_user_command(matches: &clap::ArgMatches) {
    let mut user_manager = UserManager::new();
    
    match matches.subcommand() {
        Some(("add", add_matches)) => {
            let id: u32 = add_matches.get_one::<String>("id").unwrap().parse().unwrap_or_else(|_| {
                eprintln!("Error: Invalid user ID");
                std::process::exit(1);
            });
            
            let name = add_matches.get_one::<String>("name").unwrap().clone();
            let email = add_matches.get_one::<String>("email").unwrap().clone();
            
            if !validate_email(&email) {
                eprintln!("Error: Invalid email format");
                std::process::exit(1);
            }
            
            let user = User {
                id,
                name,
                email,
                active: true,
            };
            
            match user_manager.add_user(user) {
                Ok(_) => println!("User added successfully!"),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Some(("list", _)) => {
            println!("Users:");
            for user in user_manager.get_users() {
                println!("  ID: {}, Name: {}, Email: {}, Active: {}", 
                         user.id, user.name, user.email, user.active);
            }
        }
        _ => {
            println!("Use 'user --help' to see available user commands.");
        }
    }
}

fn handle_fib_command(matches: &clap::ArgMatches) {
    let number: u32 = matches.get_one::<String>("number").unwrap().parse().unwrap_or_else(|_| {
        eprintln!("Error: Invalid number");
        std::process::exit(1);
    });
    
    match calculate_fibonacci(number) {
        Ok(result) => println!("Fibonacci of {} is: {}", number, result),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

