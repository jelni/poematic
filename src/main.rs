#![feature(stdin_forwarders)]
use crossterm::style::{Color, ResetColor, SetForegroundColor, Stylize};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{cursor, execute, QueueableCommand};
use lazy_static::lazy_static;
use rand::Rng;
use regex::Regex;
use std::fs;
use std::io::{self, Write};

const FILENAME: &str = "poem.txt";
const TEXT_WIDTH: u8 = 64;

fn main() {
    let mut stdin = io::stdin().lines();
    let mut stdout = io::stdout();

    let mut good_answers = 0;
    let mut bad_answers = 0;

    let contents = fs::read_to_string(FILENAME).expect(format!("{} not found!", FILENAME).as_str());
    let lines: Vec<&str> = contents.lines().map(|l| l.trim()).collect();

    for line in lines.iter() {
        // clear_console(&mut stdout);
        let (line, hidden_word) = hide_word(line);
        println!("{}", line.blue());
        print!("> ");
        stdout.flush().unwrap();
        if let Ok(input) = stdin.next().unwrap() {
            let input = input.trim().to_string();

            if input.to_lowercase() == hidden_word.to_lowercase() {
                good_answers += 1;
                stdout.queue(SetForegroundColor(Color::Green)).unwrap();
                print!("Good!");
            } else {
                bad_answers += 1;
                stdout.queue(SetForegroundColor(Color::Red)).unwrap();
                print!("Wrong! ({hidden_word})");
            }
        }
        let all_answers = good_answers + bad_answers;
        println!(" {good_answers}/{all_answers}");
        stdout.queue(ResetColor).unwrap();
        println!("{}", "-".repeat(TEXT_WIDTH as usize));
    }
}

fn hide_word(line: &str) -> (String, String) {
    lazy_static! {
        static ref WORD_RE: Regex = Regex::new(r"(?P<start>.*?)(?P<word>\w+)(?P<end>.*)").unwrap();
    }
    let mut rand = rand::thread_rng();
    let words = line.split_whitespace();
    let word_count = words.clone().count();
    let mut words: Vec<&str> = words.collect();

    let index = rand.gen_range(0..word_count);
    let word = words[index];
    let captures = WORD_RE.captures(word).unwrap();
    let hidden_word: String = captures.name("word").unwrap().as_str().to_string();
    let mut word = String::new();
    word.push_str(captures.name("start").unwrap().as_str());
    word.push_str("___");
    word.push_str(captures.name("end").unwrap().as_str());
    words[index] = word.as_str();
    let new_line = words.join(" ");

    return (new_line, hidden_word);
}

#[allow(dead_code)]
fn clear_console(stdout: &mut io::Stdout) {
    execute!(
        stdout,
        cursor::MoveTo(0, 0),
        Clear(ClearType::All),
        Clear(ClearType::Purge),
    )
    .unwrap();
}
