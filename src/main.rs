use rand::seq::SliceRandom;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
//print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
//☐☑☒

fn main() {
    let mut dictionary: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("./Dictionary.txt") {
        for line in lines {
            if let Ok(ip) = line {
                dictionary.push(ip);
            }
        }
    }

    let word: String = dictionary
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_string();
    let mut incorrect_letters: Vec<char> = Vec::new();
    let mut correct_letters: Vec<char> = Vec::new();
    let mut game_over = false;
    let lives = 6;
    clear_screen();
    println!("Welcome to Hangman!");
    prompt("Press enter to continue");
    while !game_over {
        clear_screen();
        println!("{} Lives remaining", lives - incorrect_letters.len());
        let progress = print_with_progress(&word, &correct_letters);
        println!("{}", progress);
        println!("Incorrect letters: {:?}", incorrect_letters);
        let guess = prompt("Enter a letter: ");
        if guess.trim().len() > 1 {
            println!("Please enter only one letter");
            continue;
        }
        if guess.trim().len() == 0 {
            println!("Please enter a letter");
            continue;
        }
        let guess = guess.trim().chars().next().unwrap();
        if word.contains(guess) {
            correct_letters.push(guess);
        } else {
            incorrect_letters.push(guess);
        }
        if incorrect_letters.len() >= lives {
            println!("\"{}\" is the word.\nYou lose.", word);
            game_over = true;
        }
        let progress = print_with_progress(&word, &correct_letters);
        if progress == word {
            println!("\"{}\" is the word!\nYou win!", word);
            game_over = true;
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn print_with_progress(s: &str, correct_letters: &Vec<char>) -> String {
    let mut full_string: String = "".to_owned();
    for c in s.chars() {
        if c.is_whitespace() {
            full_string.push_str(" ");
            // print!(" ");
        } else if correct_letters.contains(&c) {
            full_string.push_str(&c.to_string());
            // print!("{}", c);
        } else {
            full_string.push_str("_");
            // print!("_");
        }
    }
    // println!("{}", full_string);
    return full_string;
}

fn prompt(s: &str) -> String {
    println!("{}", s);
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    return line;
}

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
