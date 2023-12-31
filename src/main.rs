use std::io::{BufRead, BufReader};
use std::fs::File;
use std::env;
use rand::Rng;
fn main() {
    let args: Vec<String> = env::args().collect();
    let quote = get_quote();
    let option = args.get(1);
    if option != None {
        if option.unwrap() == "-q"{
            println!("{quote}");
            return;
        }
    }

    let quote_len = quote.chars().count();
    print!("  ");
    for _ in 0..quote_len + 1 {
        print!("-")
    }
    print!("\n");
    println!("<  {quote}  >");
    print!("  ");
    for _ in 0..quote_len + 1 {
        print!("-")
    }
    println!();

    render_taras();
}

fn get_quote() -> String {
    let fopen = File::open("quotes.txt").expect("Unable to open file");
    let file = BufReader::new(fopen);
    let mut vec = Vec::new();
    for line in file.lines() {
        vec.push(line.unwrap().to_string());
    }
    let num = rand::thread_rng().gen_range(0..vec.len());

    let quote = vec[num].to_string();

    return quote;
}

fn render_taras() {
    let fopen = File::open("pattern").expect("Unable to open file");
    let file = BufReader::new(fopen);
    for line in file.lines() {
        println!("{}", line.unwrap().to_string())
    }
}


