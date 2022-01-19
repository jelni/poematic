use crossterm::style::{Color, Stylize};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{cursor, execute};
use rand::Rng;
use std::fs;
use std::io::{self, BufRead, Write};
use poematic::*;

const FILENAME: &str = "poem.txt";
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
        let (line, hidden_word) = hide_word(&line[..]);
        print!("{}\n> ", line.with(Color::Blue));
        stdout.flush().unwrap();
        let input = stdin.next().unwrap().unwrap();
        let input = input.trim();
        let is_valid = input.eq_unicode_insensitive(&hidden_word);
        let (color, message) = if is_valid {
            correct_answers += 1;
            (Color::Green, String::from("Correct answer!"))
        } else {
            (
                Color::Red,
                format!("Wrong answer! Correct: \"{hidden_word}\""),
            )
        };
        println!(
            "{}",
            format!("{message} {correct_answers}/{}", i + 1).with(color)
        );
        println!("{}", "-".repeat(TEXT_WIDTH as usize));
    }
}

/// Selects a random word in the given string and replaces it with a blank: `___`.
/// Returns the resulting string and the selected word
fn hide_word(sentence: &str) -> (String, &str) {
    let words = sentence.split_whitespace().collect::<Vec<_>>();

    let idx = rand::thread_rng().gen_range(0..words.len());
    let hidden_word = words[idx].trim_matches(|ch: char| !ch.is_alphabetic());

    // Safe because `hidden_word` always points to a subslice of `sentence`
    let byte_offset = unsafe { hidden_word.as_ptr().offset_from(sentence.as_ptr()) as usize };
    let mut sentence = sentence.to_string();
    sentence.replace_range(byte_offset..(byte_offset + hidden_word.len()), "___");

    (sentence, hidden_word)
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
