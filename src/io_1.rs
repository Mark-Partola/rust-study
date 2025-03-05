use std::io;
use std::io::Write;

pub fn run_example(enabled: bool) {
    if !enabled {
        return;
    }

    let mut text = String::new();
    loop {
        let line = read_line();
        match line {
            Ok(line) => {
                text.push_str(&line);
                text.push('\n');

                if line.is_empty() {
                    break;
                }
            }
            Err(err) => {
                println!("{}", err);
                break;
            }
        }
    }

    println!("{}", text);
}

fn read_line() -> io::Result<String> {
    print!(">> ");
    io::stdout().flush()?;
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}
