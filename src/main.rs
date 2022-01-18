use crossterm::style::{Color, Stylize};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{cursor, execute};
use lazy_static::lazy_static;
use rand::Rng;
use regex::Regex;
use std::fs;
use std::io::{self, BufRead, Write};

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
        let (line, hidden_word) = hide_word(line);
        print!("{}\n> ", line.with(Color::Blue));
        stdout.flush().unwrap();
        let input = stdin.next().unwrap().unwrap();
        let input = input.trim();
        let is_valid = input.to_lowercase() == hidden_word.to_lowercase();
        let (color, message) = if is_valid {
            {
                correct_answers += 1;
                (Color::Green, String::from("Correct answer!"))
            }
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

fn hide_word(line: impl AsRef<str>) -> (String, String) {
    lazy_static! {
        static ref WORD_RE: Regex = Regex::new(r"\w+").unwrap();
    }

    let mut words = line.as_ref().split_whitespace().collect::<Vec<_>>();
    let n = rand::thread_rng().gen_range(0..words.len());

    let word = words[n].to_string();
    let capture = WORD_RE.find(&word).unwrap();
    let hidden_word = capture.as_str().to_string();

    let mut censored_word = word.clone();
    censored_word.replace_range(capture.range(), "___");

    words[n] = censored_word.as_str();
    let new_line = words.join(" ");
    (new_line, hidden_word)
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
