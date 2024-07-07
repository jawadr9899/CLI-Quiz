pub mod reader;
use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};
pub use reader::{read_json, Question};
use std::io::{stdin, stdout, Write};
use std::thread::sleep;
use std::time::Duration;

fn clear_screen() -> std::io::Result<()> {
    if let Err(e) = execute!(stdout(), Clear(ClearType::All)) {
        eprintln!("Failed to clear screen: {}", e);
    }
    if let Err(e) = execute!(stdout(), crossterm::cursor::MoveTo(0, 0)) {
        eprintln!("Failed to move cursor: {}", e);
    }
    Ok(())
}
fn print_result(corrects: usize, total: usize) {
    println!("Questions Asked: {total}");
    println!("Correct: {corrects}");
    println!("Wrong: {}", total - corrects);
}
pub fn main() {
    match read_json() {
        Ok(data) => {
            let data: Vec<Question> = data;
            let mut i: usize = 0;
            let mut choice = String::new();
            let mut corrects = 0;
            println!("Type 'xit' to stop!\n");

            while i != data.len() {
                stdout().flush().unwrap_or_else(|e| eprintln!("{}", e));
                println!("Q: {}", data[i].question);

                for j in 0..3 {
                    println!("{j}) {}", data[i].answers[j]);
                }
                println!("Answer: ");
                choice.clear();
                stdin().read_line(&mut choice).unwrap_or_else(|e| {
                    eprintln!("{}", e);
                    0
                });
                if choice.trim() == "xit" {
                    print_result(corrects, i);
                    break;
                }
                match choice.trim().replace("\r\n", "").parse::<usize>() {
                    Ok(u) => {
                        if u == data[i].correct_answer {
                            corrects += 1;
                            println!("Correct ✔");
                        } else {
                            println!("Wrong ❌");
                        }
                        sleep(Duration::from_secs(1));
                    }
                    Err(_) => {
                        eprintln!("Invalid choice! ❌");
                        sleep(Duration::from_secs(1));
                    }
                }
                clear_screen().unwrap();
                i += 1;
            }

            print_result(corrects, i);
        }
        Err(e) => {
            eprintln!("Error reading JSON: {}", e);
            std::process::exit(1);
        }
    }
}
