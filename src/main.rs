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
        "   Usage:

        Insert [] element as a fuzz location, if there is a few [] elements - only the FIRST one will be used
        ./fuzzis (required:) -URL+fuzz_location -wordlist.txt (optional:) -threads_amount"
        );
        println!(
        "   Example: 

        ./fuzzis https://example.com/[] home/my-directory/fuzz-list.txt 3 \n"
    );
}

fn check_args_validness(args : &Vec<String>) -> bool{
    if args.len() < 3{
        print_help();
        return false;
    }

    if Path::new(&args[2]).extension().unwrap() != TXT_EXTENTION{
        println!("The tool accepts only .txt wordlist extention");
        return false;
    }
    return true;
}

fn parse_args(args : &mut LinkedList<String>){
    let mut url : String = String::new();
    //let mut i = 0;
    while !args.is_empty(){
        let cur : String = args.pop_front().unwrap();
        match cur.as_str(){
            "-help" => {
                print_help();
                return;
            },
            "-url" => {
                url = args.pop_front().expect("URL wasn't provided");
            }
            _ => {},
        };
    }
    println!("url {}", url);
    if url == ""{
        panic!("Error: URL wasn't provided. Aborting");
    }
}

async fn build_requests(args : &[String]){
    let uri = &args[1];

    let wordlist = &args[2];
    let mut urls : Vec<String> = Vec::new();  

    let fuzz_index : usize = uri.find("[]").expect("Error: Nothing to fuzz.");
    let trimmed_brackets_uri = uri.replace("[]", "");

    let mut parallel_threads : usize = 0;
    match args.get(3){
        Some(x) => {
            parallel_threads = x.parse::<usize>().expect("Error: threads-amount is ONLY integer value");
            println!("Using {} threads for requests", &parallel_threads);
        },
        None => {
            // If user didn't provide threads - 3 threads by default
            parallel_threads = 3; 
            println!("Using default amount of threads - 3");
        }
    }


    for line in fs::read_to_string(wordlist).unwrap().lines(){
        let mut url = trimmed_brackets_uri.to_string();
        url.insert_str(fuzz_index, line);
        urls.push(url);
    }

    let client = Client::new();


    println!("Fuzzing & brute forcing: {} starts in 5 seconds", &uri);
    sleep(Duration::from_secs(5));
    println!("Fuzzing & brute forcing: {} started.", &uri);
    println!("OK & Forbidden statuses found: ");
    let responses = stream::iter(urls).map(|url| {
        let client = client.clone();
        tokio::spawn(async move {
            let resp = client.get(url).send().await;
            resp.unwrap()
        })
    }).buffer_unordered(parallel_threads);
    
    responses.for_each(|response| async{
        let resp = response.expect("Request: not possible to retrieve GET request");
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
    println!("Starting: ");
    println!("  _____              _     ");
    println!(" |  ___|   _ _______(_)___ ");
    println!(" | |_ | | | |_  /_  / / __|");
    println!(" |  _|| |_| |/ / / /| \\__ \\");
    println!(" |_|   \\__,_/___/___|_|___/");
    println!("                           ");
    sleep(Duration::from_secs(3));    

    //let mut args : LinkedList<String> = env::args().collect();
    let mut args : Vec<String> = env::args().collect();
    if !check_args_validness(&args){
        return;
    }
    build_requests(&args).await;
    //parse_args(&mut args);
}


