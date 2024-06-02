use futures::{stream, StreamExt};
use reqwest::Client;
use std::env;
use std::path::Path;
use std::fs;
const TXT_EXTENTION : &str = "txt";

fn check_args_validness(args : &[String]) -> bool{
    if args.len() != 3{
        println!("Usage: cargo run -URL -wordlist.txt");
        return false;
    }

    if Path::new(&args[2]).extension().unwrap() != TXT_EXTENTION{
        println!("The tool accepts only .txt extention");
        return false;
    }
    return true;
}
async fn build_requests(args : &[String]){
    let uri = &args[1];

    let wordlist = &args[2];
    let mut urls : Vec<String> = Vec::new();  

    let mut parallelThreads : usize = 0;
    match args.get(3){
        Some(x) => {parallelThreads = x.parse::<usize>().unwrap()},
        None => {
            parallelThreads = 3; 
            println!("Using default amount of threads - 3");
        }
    }


    for line in fs::read_to_string(wordlist).unwrap().lines(){
        let mut url = uri.to_string();
        url.push_str(line);
        urls.push(url);
    }

    let client = Client::new();

    let responses = stream::iter(urls).map(|url| {
        let client = client.clone();
        tokio::spawn(async move {
            let resp = client.get(url).send().await;
            resp.unwrap()
        })
    }).buffer_unordered(parallelThreads);
    
    responses.for_each(|response| async{
        let resp = response.unwrap();
        match resp.status(){
            reqwest::StatusCode::OK => {
                println!("Found: {}", resp.url());
            },
            _ => {}
        };
    }).await;

}



#[tokio::main]
async fn main() {
    let args : Vec<String> = env::args().collect();
    if !check_args_validness(&args){
        return;
    }
    //println!("{:?}", result);
    build_requests(&args).await;
}


