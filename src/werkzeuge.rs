use std::io::{self,  Write};
use rand::Rng;

pub fn erzeuge_zufallszahl(min: usize, max: usize) -> usize {
    let mut rng = rand::thread_rng();
    return rng.gen_range(min..=max);
}

pub fn lese_text(anweisung: &str) -> String {
    let mut eingabe = String::new();
    loop {
        if anweisung.len() > 0 {
            print!("{} ", anweisung);
        }
        let _ = std::io::stdout().flush();
        match io::stdin().read_line(&mut eingabe) {
            Ok(_) => {
                return remove_trailing_newline(&eingabe);
            },
            Err(_) => {
                println!("Es ist ein Fehler aufgetreten. Bitte noch einmal.");
            }
        }
    }
}

pub fn lese_zahl(anweisung: &str, min: usize, max: usize) -> usize {
    loop {
        let text = lese_text(anweisung);
        match text.parse::<usize>() {
            Ok(zahl) => {
                if min <= zahl && zahl <= max {
                    return zahl;
                }
                else {
                    println!("Bitte eine Zahl zwischen {} und {} eingeben.", min, max);
                }
            }
            Err(_) => {
                println!("Es ist ein Fehler aufgetreten. Bitte eine Zahl zwischen {} und {} eingeben.", min, max);
            }
        }
    }
}

fn remove_trailing_newline(input: &str) -> String {
    let mut result = input.to_string(); 
    while result.ends_with('\n') || result.ends_with('\r') {
        result.pop();
    }
    result
}
