use regex::Regex;
// use std::env;
use chrono::{Local};
use std::fs::File;
use std::io::prelude::*;
use std::process;
use std::time::Instant;
//todo: add tests
//todo: add args if needed

const RVOCAB_VERSION: &str = "0.1.0";

const ERR_OPEN: &str = ">>>>>>>> Can't open file!\n";
const ERR_READ: &str = ">>>>>>>> Can't read the file!\n";
const ERR_EMPTY: &str = "> The source file is empty.";
const ERR_NOT_HAVE: &str = "> The source file does not have English words.";
const ERR_CREATE: &str = ">>>>>>>> Can't create file!\n";
const ERR_WRITE: &str = ">>>>>>>> Can't write file!\n";
const ERR_NOT_DONE: &str = "> NOT DONE!";

const MSG_OBTAINED: &str = "> The text has been obtained from a source file.";
const MSG_CLEARED: &str = "> The source text has been cleared.";
const MSG_CREATED: &str = "> The vocabulary has been created.";
const MSG_WRITTEN: &str = "> The vocabulary has been written in a file.";
const MSG_DONE: &str = "> DONE!";

fn main() {
    let now = Instant::now();

    println!("RVocab {RVOCAB_VERSION}");

    // let args: Vec<String> = env::args().collect();
    // dbg!(&args);
    // let arg1 = &args[1];
    // let arg2 = &args[2];

    // Obtaining text from a text.txt.
    let mut file = File::open("text.txt").expect(ERR_OPEN);
    let mut raw_vocab = String::new();
    file.read_to_string(&mut raw_vocab).expect(ERR_READ);
    if raw_vocab.len() == 0 {
        println!("{ERR_EMPTY}");
        println!("{ERR_NOT_DONE}");
        process::exit(0);
    }
    println!("{MSG_OBTAINED}");

    // Clearing text and reducing spaces.
    let re1 = Regex::new(r"[^A-Za-z ]").unwrap();
    let re2 = Regex::new(r" {2,}").unwrap();
    raw_vocab = re1.replace_all(&mut raw_vocab, " ").to_string();
    raw_vocab = re2.replace_all(&mut raw_vocab, " ").trim().to_string();
    if raw_vocab.len() == 0 {
        println!("{ERR_NOT_HAVE}");
        println!("{ERR_NOT_DONE}");
        process::exit(0);
    }

    // Converting text to a vector of words.
    let mut raw_vocab_vec: Vec<String> = raw_vocab
        .split(" ")
        .map(|s| s.to_string().to_lowercase())
        .collect();

    raw_vocab_vec.sort();

    // Deleting short words.
    let mut raw_vocab_vec_clear = vec![];
    let min_word_long = 2;
    for el in raw_vocab_vec {
        if el.len() >= min_word_long {
            raw_vocab_vec_clear.push(el);
        }
    }
    println!("{MSG_CLEARED}");
    if raw_vocab_vec_clear.len() == 0 {
        println!(
            "> The source file does not have English words with a length of {min_word_long} or more letters.");
        println!("{ERR_NOT_DONE}");
        process::exit(0);
    }

    // MAIN LOGIC BEGIN

    // Removing duplicate words and counting the number for each word.
    let mut count = 0;
    let mut word = raw_vocab_vec_clear.get(0).unwrap().to_string();
    let mut rvocab_vec = vec![];
    for el in raw_vocab_vec_clear {
        if word == el {
            count += 1;
        } else {
            let count_word = (count, word);
            rvocab_vec.push(count_word);
            word = el;
            count = 1
        }
    }

    // Sort words by their number.
    rvocab_vec.sort_by(|a, b| b.0.cmp(&a.0));

    // Alignment by spaces of length of quantity strings and making of a vector of HTML-list elements.
    let mut rvocab_vec_string = vec![];
    let count_max_length = rvocab_vec[0].0.to_string().len();
    let leveling_space = "&nbsp;".to_string();
    for el in rvocab_vec {
        let length_diff = count_max_length - el.0.to_string().len();
        let el0 = if length_diff != 0 {
            let leveling_spaces = leveling_space.repeat(length_diff);
            leveling_spaces + &el.0.to_string()
        } else {
            el.0.to_string()
        };
        let el0 = "<li>".to_string() + el0.as_str();
        let el1 =
            "<a href='https://translate.yandex.com/en/?source_lang=en&target_lang=ru&text="
                .to_string()
                + &el.1
                + "' target='_blank'>"
                + &el.1
                + "</a></li>";
        rvocab_vec_string.push(format!("{el0}&nbsp;-&nbsp;{el1}"));
    }
    println!("{MSG_CREATED}");

    // MAIN LOGIC END

    // Making a ready-made vector with a title and an HTML-list of unique words.
    let count = rvocab_vec_string.len();
    let rvocab_version = "<p>RVocab ".to_string() + RVOCAB_VERSION + "</p>";
    let title = "<p>Total unique words: ".to_string() + &count.to_string() + "</p>";
    let date_time_local = Local::now();
    let date_time_format = date_time_local.format("%Y-%m-%d %H:%M:%S");
    let date_time = "<p>".to_string() + &date_time_format.to_string() + "</p>";
    let mut rvocab_vec_full = vec![];
    rvocab_vec_full.push(rvocab_version);
    rvocab_vec_full.push(title);
    rvocab_vec_full.push(date_time);
    rvocab_vec_full.push("<ul>".to_string());
    rvocab_vec_full.append(&mut rvocab_vec_string);
    rvocab_vec_full.push("</ul>".to_string());

    // Making ready-made HTML-markup and writing it to a rvocab.html.
    let rvocab = rvocab_vec_full.join("\n");
    let mut file = File::create("rvocab.html").expect(ERR_CREATE);
    file.write_all(rvocab.as_bytes()).expect(ERR_WRITE);
    println!("{MSG_WRITTEN}");
    println!("{MSG_DONE}");

    let elapsed = now.elapsed();

    println!("Total time: {elapsed:.2?}")
}
