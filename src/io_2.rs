use std::io;
use std::io::Write;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

enum Command {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

pub fn run_example(enabled: bool) {
    if !enabled {
        return;
    }

    run();
}

fn read_line(invitation: &str) -> io::Result<String> {
    let mut input = String::new();

    print!("{}", invitation);
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;

    Ok(input)
}

fn parse_command(command: &str) -> Option<Command> {
    use Command::*;

    match command.trim().to_lowercase().as_str() {
        "off" => Some(Off),
        "sleep" => Some(Sleep),
        "reboot" => Some(Reboot),
        "shutdown" => Some(Shutdown),
        "hibernate" => Some(Hibernate),
        _ => None,
    }
}

fn execute_command(command: Command) {
    use Command::*;

    match command {
        Off => {
            println!("Off");
            exit(0);
        }
        Sleep => {
            println!("Sleep");
            sleep(Duration::from_secs(5))
        }
        Reboot => {
            println!("Shutting down..");
            sleep(Duration::from_secs(5));
            println!("Booting up..");
            sleep(Duration::from_secs(2));
        }
        Shutdown => {
            println!("Shutting down..");
            sleep(Duration::from_secs(5));
            exit(0);
        }
        Hibernate => {
            println!("Hibernate..");
            match read_line("") {
                Ok(_) => println!("Awaking.."),
                Err(err) => handle_error(err),
            }
        }
    }
}

fn run() {
    loop {
        let line = read_line(">> ");

        match line {
            Ok(command) => match parse_command(&command) {
                Some(command) => execute_command(command),
                None => println!("Invalid command. Repeat."),
            },
            Err(err) => {
                handle_error(err);
                continue;
            }
        }
    }
}

fn handle_error(err: io::Error) {
    println!("Something went wrong! Repeat., {:?}", err);
}
