use regex::Regex;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;
use std::time::Instant;
//todo: add comments
//todo: add tests

fn main() {
    let now = Instant::now();

    const RVOCAB_VERSION: &str = "0.1.0";
    println!("RVocab {}", RVOCAB_VERSION);

    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let arg1 = args[1];
    let arg2 = args[2];

    let mut file = File::open("text.txt").expect(">>>>>>>> Can't open file!\n");
    let mut raw_vocab = String::new();
    file.read_to_string(&mut raw_vocab)
        .expect(">>>>>>>> Can't read the file!\n");
    if raw_vocab.len() == 0 {
        println!("> Source file is empty.");
        println!("> NOT DONE!");
        process::exit(0);
    }
    println!("> The text has been obtained from a source file.");

    let re1 = Regex::new(r"[^A-Za-z ]").unwrap();
    let re2 = Regex::new(r" {2,}").unwrap();
    raw_vocab = re1.replace_all(&mut raw_vocab, " ").to_string();
    raw_vocab = re2.replace_all(&mut raw_vocab, " ").trim().to_string();
    if raw_vocab.len() == 0 {
        println!("> The source file does not have English words.");
        println!("> NOT DONE!");
        process::exit(0);
    }

    let mut raw_vocab_vec: Vec<String> = raw_vocab
        .split(" ")
        .map(|s| s.to_string().to_lowercase())
        .collect();

    raw_vocab_vec.sort();

    let mut raw_vocab_vec_clear = vec![];
    let min_word_long = 2;
    for el in raw_vocab_vec {
        if el.len() >= min_word_long {
            raw_vocab_vec_clear.push(el);
        }
    }
    println!("> The source text has been cleared.");
    if raw_vocab_vec_clear.len() == 0 {
        println!(
            "> The source file does not have English words with a length of {} or more letters.",
            min_word_long
        );
        println!("> NOT DONE!");
        process::exit(0);
    }

    // MAIN LOGIC BEGIN

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

    rvocab_vec.sort_by(|a, b| b.0.cmp(&a.0));

    let mut rvocab_vec_string = vec![];
    let count_max_length = rvocab_vec[0].0.to_string().len();
    for el in rvocab_vec {
        let length_diff = count_max_length - el.0.to_string().len();
        let leveling_space = "&nbsp".to_string();
        let el0 = if length_diff != 0 {
            let leveling_spaces = leveling_space.repeat(length_diff);
            leveling_spaces + &el.0.to_string()
        } else {
            el.0.to_string()
        };
        let el1 = "<a href='https://translate.yandex.com/be/?source_lang=en&target_lang=ru&text=".to_string() + &el.1 + " 'target='_blank'>" + &el.1 + "</a></br>";
        rvocab_vec_string.push(format!("{el0}  {el1}"));
    }
    println!("> The vocabulary has been created.");

    // MAIN LOGIC END

    let count = rvocab_vec_string.len();
    let title = "Total unique words: ".to_string() + &count.to_string() + "\n";
    let mut rvocab_vec_full = vec![];
    rvocab_vec_full.push(title);
    rvocab_vec_full.append(&mut rvocab_vec_string);

    let rvocab = rvocab_vec_full.join("\n");
    let mut file = File::create("rvocab.html").expect(">>>>>>>> Can't create file!\n");
    file.write_all(rvocab.as_bytes())
        .expect(">>>>>>>> Can't write file!\n");
    println!("> The vocabulary has been written in a file.");
    println!("> DONE!");

    let elapsed = now.elapsed();

    println!("Total time: {:.2?}", elapsed)
}
