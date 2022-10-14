//importing in execute! macro
#[macro_use]
extern crate crossterm;

use crossterm::cursor;
use crossterm::event::{read, Event, KeyCode, KeyEvent};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crossterm::style::{Print, SetForegroundColor, Color};
use crossterm::style::Stylize;

use std::io::{stdout};
use std::time::{Duration, Instant};


fn main() {
    let mut stdout = stdout();
    enable_raw_mode().unwrap();

    // what the user enters
    let mut characters: Vec<char> = Vec::new();

    // make this random
    let challenge = String::from("this is an example sentence, dont fuck it up");

    // start message
    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0), Print("Press enter to start")).unwrap(); 
    // wait for user input
    read().unwrap();

    // clear terminal and print challenge text
    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0), SetForegroundColor(Color::Blue), Print(&challenge), cursor::MoveTo(0, 0)).unwrap(); 

    // start timer
    let now = Instant::now();

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
                
                // if character is incorrect, make it red otherwise white
                if char != challenge.chars().nth(characters.len()-1).unwrap() {
                    // if the correct character in the sequence is a space, place the incorrect character, otherwise replace with correct character
                    if challenge.chars().nth(characters.len()-1).unwrap() == ' ' {
                        println!("{}", format!("{}", char).red())
                    } else {
                        execute!(stdout, cursor::MoveTo((characters.len()-1).try_into().unwrap(), 0)).unwrap();
                        println!("{}", format!("{}", challenge.chars().nth(characters.len()-1).unwrap()).red());
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

    // check if this is correct
    let elapsed: f32 = now.elapsed().as_secs() as f32 / 60.0;
    let wpm: f32 = (characters.len() as f32 / 5.0) / elapsed;

    println!("\n{:.0}", wpm);

    disable_raw_mode().unwrap();
}
