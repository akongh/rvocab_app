use chrono::Local;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;
use std::time::Instant;
//todo: add tests
//todo: regex & z

const RVOCAB_VERSION: &str = "0.1.0";

const DEFAULT_INPUT_FILE_NAME: &str = "text.txt";
const DEFAULT_OUTPUT_FILE_NAME: &str = "rvocab.html";

const ERR_INPUT_NAME: &str = ">>>>>>>> No input file name given.\n";
const ERR_OPEN: &str = ">>>>>>>> Can't open file!\n";
const ERR_READ: &str = ">>>>>>>> Can't read the file!\n";
const ERR_EMPTY: &str = "> The source file is empty.";
const ERR_NOT_HAVE: &str = "> The source file does not have English words.";
const ERR_WORDS_LENGTH: &str =
    "> The source file does not have English words with a set length or more.";
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

    let mut input_args = env::args();
    let mut input_file_name = DEFAULT_INPUT_FILE_NAME.to_string();
    if input_args.len() > 1 {
        input_file_name = input_args.nth(1).expect(ERR_INPUT_NAME);
    }

    // Obtaining text from an input file.
    let mut file = File::open(input_file_name).expect(ERR_OPEN);
    let mut raw_vocab = String::new();
    file.read_to_string(&mut raw_vocab).expect(ERR_READ);
    if raw_vocab.len() == 0 {
        println!("{ERR_EMPTY}");
        println!("{ERR_NOT_DONE}");
        process::exit(0);
    }
    println!("{MSG_OBTAINED}");

    // Clearing text and reducing spaces.
    let re1 = Regex::new(r"[^A-Za-z]").unwrap();
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
        println!("{ERR_WORDS_LENGTH}");
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
        // In this loop a last "count_word" is not added to "rvocab_vec".
    }

    // Adding a last "count_word" to "rvocab_vec".
    let count_word = (count, word);
    rvocab_vec.push(count_word.clone());

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
        let el1 = "<a href='https://translate.yandex.com/en/?source_lang=en&target_lang=ru&text="
            .to_string()
            + &el.1
            + "' target='_blank'>"
            + &el.1
            + "</a></li>";
        rvocab_vec_string.push(format!("{el0}&nbsp;-&nbsp;{el1}"));
    }
    println!("{MSG_CREATED}");

    // MAIN LOGIC END

    // Making additional data for a template.
    let count = rvocab_vec_string.len().to_string();
    let date_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let rvocab = rvocab_vec_string.join("\n");

    // Obtaining markup from a template.
    let mut html_template = include_str!("template.html").to_string();

    // Filling out the template.
    let r_version = Regex::new(r"\{\{version}}").unwrap();
    let r_count = Regex::new(r"\{\{count}}").unwrap();
    let r_date_time = Regex::new(r"\{\{date_time}}").unwrap();
    let r_rvocab = Regex::new(r"\{\{rvocab}}").unwrap();
    html_template = r_version
        .replace_all(&html_template, RVOCAB_VERSION)
        .to_string();
    html_template = r_count
        .replace_all(&html_template, count)
        .to_string();
    html_template = r_date_time
        .replace_all(&html_template, date_time)
        .to_string();
    html_template = r_rvocab
        .replace_all(&html_template, rvocab)
        .to_string();

    // Making ready-made HTML-markup and writing it to an output html.
    let mut file = File::create(DEFAULT_OUTPUT_FILE_NAME).expect(ERR_CREATE);
    file.write_all(html_template.as_bytes()).expect(ERR_WRITE);

    println!("{MSG_WRITTEN}");
    println!("{MSG_DONE}");

    let elapsed = now.elapsed();

    println!("Total time: {elapsed:.2?}")
}
