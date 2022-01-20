use crossterm::style::{Color, Stylize};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{cursor, execute};
use std::borrow::Cow;
use std::fs;
use std::io::{self, BufRead, Write};

use poematic::*;

const FILENAME: &str = "poem.txt";
const DIFFICULTY: u8 = 3;
const TEXT_WIDTH: u8 = 64;

fn main() {
    let mut stdin = io::BufReader::new(io::stdin()).lines();
    let mut stdout = io::stdout();

    let file = fs::File::open(FILENAME)
        .unwrap_or_else(|err| panic!("Failed reading \"{FILENAME}\": {err}"));
    let file = io::BufReader::new(file);
    let lines = file
        .lines()
        .map(|l| l.expect("Failed parsing file"))
        .collect::<Vec<_>>();

    let mut correct_answers = 0;

    for (i, line) in lines.iter().enumerate() {
        let (line, hidden_words) = hide_words(&line, "___", DIFFICULTY as usize);
        print!("{}\n> ", line.with(Color::Blue));
        stdout.flush().unwrap();

        let input = stdin.next().unwrap().unwrap();
        let input_words = input.split_human().collect::<Vec<_>>();

        let is_valid = hidden_words
            .iter()
            .zip(input_words.iter())
            .all(|(&h, &i)| h.eq_unicode_insensitive(i));

        let (color, message) = if is_valid {
            correct_answers += 1;
            (Color::Green, Cow::Borrowed("Correct answer!"))
        } else {
            (
                Color::Red,
                format!("Wrong answer! Correct: {}", hidden_words.join(", ")).into(),
            )
        };
        println!(
            "{}",
            format!("{message} {correct_answers}/{}", i + 1).with(color)
        );
        println!("{}", "-".repeat(TEXT_WIDTH as usize));
    }
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
