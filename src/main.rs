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

fn parse_args(args : &mut LinkedList<String>, url : &mut String, wordlist_path : &mut String, parallel_threads : &mut usize){


    while !args.is_empty(){
        let cur : String = args.pop_front().unwrap();
        match cur.as_str(){
            "-help" => {
                print_help();
                return;
            },
            "-h" => {
                print_help();
                return;
            },
            "-url" => {
                *url = args.pop_front().expect("-url wasn't provided");
            },
            "-w" => {
                *wordlist_path = args.pop_front().expect("-w wasn't provided");
            }
            "-t" => {
                *parallel_threads = args.pop_front().expect("-t wasn't provided")
                .parse::<usize>().expect("Error: -t flag receives ONLY positive numeric value");
            },
            _ => {},
        };
    }
    if url == ""{
        panic!("Error: -url wasn't provided. Aborting");
    }
    //println!("url {}", url);
    //build_requests(url, wordlist_path, parallel_threads);
}

async fn build_requests(uri : String, wordlist_path : String, parallel_threads : usize){

    let mut urls : Vec<String> = Vec::new();  

    let fuzz_index : usize = uri.find("[]").expect("Error: Nothing to fuzz.");
    let trimmed_brackets_uri = uri.replace("[]", "");

    for line in fs::read_to_string(wordlist_path).unwrap().lines(){
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

    /*let mut args : Vec<String> = env::args().collect();
    if !check_args_validness(&args){
        return;
    }*/
    let mut args : LinkedList<String> = env::args().collect();
    let mut url : String = String::new();
    let mut wordlist_path: String = String::new();
    let mut parallel_threads: usize = 3; // Default amount of threads
    parse_args(&mut args, &mut url, &mut wordlist_path, &mut parallel_threads);
    build_requests(url, wordlist_path, parallel_threads).await;
    //parse_args(&mut args);
}


