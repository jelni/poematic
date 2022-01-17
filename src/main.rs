use crossterm::style::{Color, Print, ResetColor, SetForegroundColor, Stylize};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{cursor, execute, QueueableCommand};
use rand::Rng;
use std::fs;
use std::io::{self, BufRead, BufReader, Write};

const FILENAME: &str = "poem.txt";
const TEXT_WIDTH: u8 = 64;

fn main() {
    let mut stdin = BufReader::new(io::stdin()).lines();
    let mut stdout = io::stdout();

    let file = fs::File::open(FILENAME).expect("failed loading file");
    let file = BufReader::new(file);
    let lines = file
        .lines()
        .enumerate()
        .map(|(n, s)| s.unwrap_or_else(|err| panic!("line {} is invalid: {}", n, err)))
        .collect::<Vec<_>>();

    lines
        .iter()
        .enumerate()
        .fold(0, |correct_answers, (i, line)| {
            // clear_console(&mut stdout);
            let (line, hidden_word) = hide_word(line);
            stdout.queue(Print(line.blue())).unwrap();
            stdout.queue(Print("> ")).unwrap();
            stdout.flush().unwrap();
            let input = stdin.next().unwrap().unwrap();
            let input = input.trim();
            let is_valid = input.eq_ignore_ascii_case(&hidden_word);
            let (correct_answers, foreground_color, message) = if is_valid {
                (
                    correct_answers + 1,
                    Color::Green,
                    format!("Correct answer!"),
                )
            } else {
                (
                    correct_answers,
                    Color::Red,
                    format!("Wrong answer! Correct = \"{hidden_word}\""),
                )
            };
            stdout.queue(SetForegroundColor(foreground_color)).unwrap();
            stdout.queue(Print(format!("{message} "))).unwrap();
            stdout
                .queue(Print(format!("{correct_answers}/{}\n", i + 1)))
                .unwrap();
            stdout.queue(ResetColor).unwrap();
            stdout
                .queue(Print("-".repeat(TEXT_WIDTH as usize)))
                .unwrap();
            stdout.queue(Print("\n\n")).unwrap();
            stdout.flush().unwrap();
            correct_answers
        });
}

// here we're using AsRef<str> instead of &str or String, because AsRef<str> can accept both of these.
fn hide_word(line: impl AsRef<str>) -> (String, String) {
    let line = line.as_ref();
    let words = line.split_whitespace().collect::<Vec<_>>();
    let n = rand::thread_rng().gen_range(0..words.len());
    let hidden_word = words.get(n).unwrap().to_string();
    // eventually replace it with some constant if you want to make it harder for user to recognize the hidden word
    let censored_word = "_".repeat(hidden_word.len());

    // encapsulate mutable access into a block, so that it's clearly visible where it gets mutated.
    let words = {
        let mut words = words;
        words[n] = &censored_word;
        words
    };

    let line = words.join(" ");
    (line, hidden_word)
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
