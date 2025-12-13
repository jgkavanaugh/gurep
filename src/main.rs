use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use regex::{RegexBuilder};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    ///Pattern to look for
    #[arg(short, long)]
    pattern: String,
    ///Path to file
    #[arg(short, long)]
    file_path: Option<String>,
    ///Case insensitive flag
    #[arg(short, long)]
    case_insensitive: bool,
    //invert match
    //v: ,
    //stop after N matches
    //m: ,
    //print line numbers
    //n: ,
}

fn line_reader(path: Option<&String>) -> io::Result<Box<dyn BufRead>> {
    match path {
        Some(p) => Ok(Box::new(BufReader::new(File::open(p)?))),
        None => Ok(Box::new(BufReader::new(io::stdin().lock()))),
    }
}

fn setup_regex(pattern: &str, case_insensitive: bool) -> regex::Regex {
    if case_insensitive {
        RegexBuilder::new(pattern).case_insensitive(true).build().unwrap()
    }
    else {
        RegexBuilder::new(pattern).build().unwrap()
    }
}

fn main() {   
    //parse commmand line arguments
    let args = Args::parse();

    //setup reader
    let reader = line_reader(args.file_path.as_ref());

    //setup regex
    let re = setup_regex(&args.pattern, args.case_insensitive);

    //iterate through file looking for pattern
    for line in reader.expect("Errored reading line").lines() {
        if re.is_match(line.as_ref().unwrap()) {
            println!("{}", line.expect("error"));
        }
    }
}
