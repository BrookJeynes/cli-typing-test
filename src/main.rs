//importing in execute! macro
#[macro_use]
extern crate crossterm;

use colored::Colorize;
use crossterm::cursor;
use crossterm::event::{read, Event, KeyCode, KeyEvent};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{stdout};

fn main() {
    let mut stdout = stdout();
    enable_raw_mode().unwrap();

    let mut characters: Vec<char> = Vec::new();
    let challenge = String::from("this is an example sentence, dont fuck it up");

    // clear terminal
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); 

    print!("{}", format!("{}", challenge).blue());

    //key detection
    loop {
        let character = read().unwrap(); 
       
        // Deconstruct to get wanted value
        if let Event::Key(KeyEvent{ code, .. }) = character {
            // User deletes a character 
            if code == KeyCode::Backspace {
                characters.pop();

                execute!(stdout, cursor::MoveTo(characters.len().try_into().unwrap(), 0)).unwrap();
                println!("{}", format!("{}", challenge.chars().nth(characters.len()).unwrap()).blue());
            }

            let char_code = code;
            
            // Deconstruct to get char
            if let KeyCode::Char(char) = char_code {
                characters.push(char);
                
                // if character is incorrect, make it red otherwise leave it
                if char != challenge.chars().nth(characters.len()-1).unwrap() {
                    // if character is a space make correct character red
                    if char == ' ' {
                        execute!(stdout, cursor::MoveTo((characters.len()-1).try_into().unwrap(), 0)).unwrap();
                        println!("{}", format!("{}", challenge.chars().nth(characters.len()-1).unwrap()).red());
                    } else {
                        println!("{}", format!("{}", char).red())
                    }
                } else {
                    println!("{}", format!("{}", char).white())
                }
            }

            // move curser to end of string
            execute!(stdout, cursor::MoveTo(characters.len().try_into().unwrap(), 0)).unwrap();
        }

        if characters.len() == challenge.len() {
            break;
        }
    }

    // (200 chars / 5) / 1 minute = words per minute

    disable_raw_mode().unwrap();
}
