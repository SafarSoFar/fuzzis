use futures::{stream, StreamExt};
use reqwest::Client;
use std::collections::LinkedList;
use std::env;
use std::path::Path;
use std::fs;
use std::thread::sleep;
use std::time::Duration;
const TXT_EXTENTION : &str = "txt";

fn print_help(){
    println!(
        "Usage:
            To define where to fuzz - add [] (square brackets) inside uri"
    );
    println!(
        "Example: 
            ./fuzzis -uri https://example.com/[] -w home/my-directory/fuzz-list.txt -t 5 \n"
    );
    println!(
        "Flags:
            Required:
                -uri : URL to buzz, brute-force
                -w : wordlist *.txt only*
    
            Optional:
                -t : amount of parallel threads

            Help:
                -h, -help : prints help"
    );
    std::process::exit(0);
}


fn check_wordlist_path(wordlist_path : &String){
    if Path::new(wordlist_path).extension().unwrap() != TXT_EXTENTION{
        panic!("Error: The tool accepts only .txt wordlist extention");
    }
}

fn parse_args(args : &mut LinkedList<String>, uri : &mut String, wordlist_path : &mut String, parallel_threads : &mut usize){


    while !args.is_empty(){
        let cur : String = args.pop_front().unwrap();
        match cur.as_str(){
            "-help" => {
                print_help();
            },
            "-h" => {
                print_help();
            },
            "-uri" => {
                *uri = args.pop_front().expect("-uri wasn't provided");
            },
            "-w" => {
                *wordlist_path = args.pop_front().expect("-w wasn't provided");
                check_wordlist_path(wordlist_path);
            }
            "-t" => {
                *parallel_threads = args.pop_front().expect("-t wasn't provided")
                .parse::<usize>().expect("Error: -t flag receives ONLY positive numeric value");
            },
            _ => {},
        };
    }
    if uri == ""{
        print_help();
        panic!("Error: -uri wasn't provided. Aborting");
    }
}

async fn build_requests(uri : String, wordlist_path : String, parallel_threads : usize){

    let mut uris : Vec<String> = Vec::new();  

    let fuzz_index : usize = uri.find("[]").expect("Error: Nothing to fuzz. Add [] to url ");
    let trimmed_brackets_uri = uri.replace("[]", "");

    for line in fs::read_to_string(wordlist_path).unwrap().lines(){
        let mut uri = trimmed_brackets_uri.to_string();
        uri.insert_str(fuzz_index, line);
        uris.push(uri);
    }

    let client = Client::new();


    println!("Using threads - {}", &parallel_threads);
    println!("Fuzzing & brute forcing: {} starts in 5 seconds", &uri);
    sleep(Duration::from_secs(5));
    println!("Fuzzing & brute forcing: {} started.", &uri);
    println!("OK & Forbidden statuses found: ");
    let responses = stream::iter(uris).map(|url| {
        let client = client.clone();
        tokio::spawn(async move {
            let resp = client.get(url).send().await;
            resp.unwrap()
        })
    }).buffer_unordered(parallel_threads);
    
    responses.for_each(|response| async{
        let resp = response.expect("Error: not possible to retrieve GET request. Try set less threads.");
        match resp.status(){
            reqwest::StatusCode::OK => {
                println!("Found: {}", resp.url());
            },
            reqwest::StatusCode::FORBIDDEN => {
                println!("Found, access forbidden {}", resp.url())
            },
            _ => {}
        };
    }).await;

}


#[tokio::main]
async fn main() {
    let mut args : LinkedList<String> = env::args().collect();
    let mut uri : String = String::new();
    let mut wordlist_path: String = String::new();
    let mut parallel_threads: usize = 3; // Default amount of threads
    parse_args(&mut args, &mut uri, &mut wordlist_path, &mut parallel_threads);
    println!("Starting: ");
    println!("  _____              _     ");
    println!(" |  ___|   _ _______(_)___ ");
    println!(" | |_ | | | |_  /_  / / __|");
    println!(" |  _|| |_| |/ / / /| \\__ \\");
    println!(" |_|   \\__,_/___/___|_|___/");
    println!("                           ");
    sleep(Duration::from_secs(3));    
    build_requests(uri, wordlist_path, parallel_threads).await;
    //parse_args(&mut args);
}


