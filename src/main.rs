use crossterm::style::{Color, Stylize};
use std::error;
use std::fs;
use std::io::{self, BufRead, Write};

use poematic::*;

const FILENAME: &str = "poem.txt";
const TEXT_WIDTH: u8 = 64;

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut stdin = io::BufReader::new(io::stdin()).lines();
    let mut stdout = io::stdout();

    let file = fs::File::open(FILENAME)?;
    let file = io::BufReader::new(file);
    let lines = file.lines().map(Result::unwrap).collect::<Vec<_>>();

    let mut words_to_hide = 1;
    loop {
        let mut correct_answers = 0;
        for (i, line) in lines.iter().enumerate() {
            let (line, hidden_words) = hide_words(&line, words_to_hide as usize);
            print!("{}\n> ", line.with(Color::Blue));
            stdout.flush()?;

            let input = stdin.next().unwrap()?;

            if is_valid_guess(&input, &hidden_words) {
                correct_answers += 1;
                print!("{}", "Correct answer!".with(Color::Green));
            } else {
                print!(
                    "{}",
                    format!("Wrong answer! Correct: {}", hidden_words.join(", ")).with(Color::Red),
                );
            };

            println!(" {correct_answers}/{}", i + 1);
            println!("{}", "-".repeat(TEXT_WIDTH as usize));
        }

        if correct_answers >= lines.len() {
            words_to_hide += 1;
        }
    }
}

fn is_valid_guess(guess: &str, hidden_words: &[&str]) -> bool {
    let guess = guess.split_human().collect::<Vec<_>>();

    if guess.len() != hidden_words.len() {
        return false;
    }

    hidden_words
        .iter()
        .zip(guess)
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
        assert!(!is_valid_guess("hello", &["hello", "world"]));
        assert!(!is_valid_guess("hello world foo", &["hello", "world"]));
    }
}
