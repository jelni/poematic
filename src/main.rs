use crossterm::style::{Color, ResetColor, SetForegroundColor, Stylize};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{cursor, execute, QueueableCommand};
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
        .unwrap_or_else(|err| panic!("failed reading \"{FILENAME}\": {err}"));
    let file = io::BufReader::new(file);
    let lines = file
        .lines()
        .enumerate()
        .map(|(n, l)| l.unwrap_or_else(|err| panic!("line {} is invalid: {}", n, err)))
        .collect::<Vec<_>>();

    lines
        .iter()
        .enumerate()
        .fold(0, |correct_answers, (i, line)| {
            let (line, hidden_word) = hide_word(line);
            print!("{}\n> ", line.blue());
            stdout.flush().unwrap();
            let input = stdin.next().unwrap().unwrap();
            let input = input.trim();
            let is_valid = input.to_lowercase() == hidden_word.to_lowercase();
            let (correct_answers, foreground_color, message) = if is_valid {
                (
                    correct_answers + 1,
                    Color::Green,
                    String::from("Correct answer!"),
                )
            } else {
                (
                    correct_answers,
                    Color::Red,
                    format!("Wrong answer! Correct: \"{hidden_word}\""),
                )
            };
            stdout.queue(SetForegroundColor(foreground_color)).unwrap();
            println!("{message} {correct_answers}/{}", i + 1);
            stdout.queue(ResetColor).unwrap();
            println!("{}", "-".repeat(TEXT_WIDTH as usize));
            correct_answers
        });
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
