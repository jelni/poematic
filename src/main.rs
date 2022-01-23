use crossterm::style::{Color, Stylize};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{cursor, execute};
use std::io::{self, BufRead, Write};
use std::{fs, iter};

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

        if is_valid_guess(&input, &hidden_words) {
            correct_answers += 1;
            print!("{}", "Correct answer! ".with(Color::Green));
        } else {
            print!(
                "{}",
                format!("Wrong answer! Correct: {} ", hidden_words.join(", ")).with(Color::Red),
            );
        };

        println!("{correct_answers}/{}", i + 1);
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

fn is_valid_guess(guess: &str, hidden_words: &[&str]) -> bool {
    hidden_words
        .iter()
        .zip(guess.split_human().chain(iter::repeat("")))
        .all(|(&h, g)| h.eq_unicode_insensitive(g))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_valid_guess() {
        assert!(is_valid_guess("foo", &["foo"]));
        assert!(!is_valid_guess("foo", &["bar"]));
        assert!(is_valid_guess("hello world", &["hello", "world"]));
        assert!(!is_valid_guess("world hello", &["hello", "world"]));
        assert!(!is_valid_guess("hello", &["hello", "world"]));
        assert!(is_valid_guess("hello world foo", &["hello", "world"]));
    }
}
