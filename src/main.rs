use std::{io, env};
use std::io::Write;
use std::process::{Command, Stdio};
use std::path::Path;
use rand::Rng;



fn main(){
    loop {
        print!("> ");
        io::stdout().flush();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let mut args = parts;

        match command {
            "cd" => {
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            },
            "exit" => return,

            "add" => {
                let mut result: i32 = 0;

                for arg in args {
                    let num: i32 = arg.parse().expect("Invalid number");
                    result += num
                    
                }
                println!("Result: {}", result);
            },

            "quote" => {
                random_quote()
            },

            "convertToF" => {
                match args.next() {
                    Some(arg) => {
                        match arg.parse::<i32>() {
                            Ok(celsius) => {
                                let fahrenheit = (celsius * 9 / 5) + 32;
                                println!("{}°C is {}°F", celsius, fahrenheit);
                            },
                            Err(e) => {
                                eprintln!("Error: {}", e);
                            }
                        }
                    },
                    None => {
                        eprintln!("Missing temperature in Celsius");
                    }
                }
            },
            "convertToC" => {
                match args.next() {
                    Some(arg) => {
                        match arg.parse::<i32>() {
                            Ok(fahrenheit) => {
                                let celcius = (fahrenheit - 32) * 5 / 9;
                                println!("{}°F is {}°C", fahrenheit, celcius);
                            },
                            Err(e) => {
                                eprintln!("Error: {}", e);
                            }
                        }
                    },
                    None => {
                        eprintln!("Missing temperature in Celsius");
                    }
                }
            },

            "mem" => {
                let mem_total = execute_shell_command("cat /proc/meminfo | grep MemTotal");
                let mem_free = execute_shell_command("cat /proc/meminfo | grep MemFree");
                let buffers = execute_shell_command("cat /proc/meminfo | grep Buffers");
                let cached = execute_shell_command("cat /proc/meminfo | grep Cached");
            
                if mem_total.status.success() && mem_free.status.success() && buffers.status.success() && cached.status.success() {
                    let mem_total_str = String::from_utf8_lossy(&mem_total.stdout);
                    let mem_free_str = String::from_utf8_lossy(&mem_free.stdout);
                    let buffers_str = String::from_utf8_lossy(&buffers.stdout);
                    let cached_str = String::from_utf8_lossy(&cached.stdout);
            
                    println!("{}", mem_total_str);
                    println!("{}", mem_free_str);
                    println!("{}", buffers_str);
                    println!("{}", cached_str);
                } else {
                    eprintln!("Command failed with an error code.");
                }
            }
            
            command => {
                let child = Command::new(command)
                    .args(args)
                    .spawn();

                // gracefully handle malformed user input
                match child {
                    Ok(mut child) => { child.wait(); },
                    Err(e) => eprintln!("{}", e),
                };
            }
        }
    }
}


fn random_quote() {
    let motivational_quotes = [
        "The only way to do great work is to love what you do. - Steve Jobs",
        "Don't watch the clock; do what it does. Keep going. - Sam Levenson",
        "Believe you can, and you're halfway there. -Theodore Roosevelt",
        "Don't count the days, make the days count. - Muhammad Ali",
        "Success is not final, failure is not fatal: It is the courage to continue that counts. - Winston Churchill",
        "The only limit to our realization of tomorrow will be our doubts of today. - Franklin D. Roosevelt",
        "The only place where success comes before work is in the dictionary. - Vidal Sassoon",
        "Your time is limited, don't waste it living someone else's life. - Steve Jobs",
        "Success is walking from failure to failure with no loss of enthusiasm. - Winston S. Churchill",
        "The road to success and the road to failure are almost exactly the same. - Colin R. Davis",
        "The only thing standing between you and your goal is the story you keep telling yourself as to why you can't achieve it. - Jordan Belfort",
        "Don't be afraid to give up the good to go for the great. - John D. Rockefeller",
        "I find that the harder I work, the more luck I seem to have. - Thomas Jefferson",
        "The future depends on what you do today. - Mahatma Gandhi",
        "Opportunities don't happen. You create them. - Chris Grosser",
        "Success is not the key to happiness. Happiness is the key to success. If you love what you are doing, you will be successful. - Albert Schweitzer",
        "You are never too old to set another goal or to dream a new dream. - C.S. Lewis",
        "The only person you are destined to become is the person you decide to be. - Ralph Waldo Emerson",
        "The best way to predict the future is to create it. - Peter Drucker",
        "The only thing that stands between you and your dream is the will to try and the belief that it is actually possible. - Joel Brown",
        "The harder you work for something, the greater you'll feel when you achieve it. - Unknown",
        "Success is not in what you have, but who you have become. - Bo Bennett",
        "Believe in yourself and all that you are. Know that there is something inside you that is greater than any obstacle. - Christian D. Larson",
        "Don't let yesterday take up too much of today. - Will Rogers",
        "Your time is now. Start where you stand and never back down. - Tom Hopkins",
    ];

    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..motivational_quotes.len());

    let random_quote = motivational_quotes[random_index];
    println!("{}", random_quote);

}

fn execute_shell_command(command: &str) -> std::process::Output {
    Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to read command.")
}