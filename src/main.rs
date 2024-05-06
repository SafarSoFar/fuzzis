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
    let mut words : Vec<String> = Vec::new();

    for line in fs::read_to_string(wordlist).unwrap().lines(){
        words.push(line.to_string());
    }

    while !&words.is_empty(){
        let cur_word = words.pop().unwrap();
        let mut url = String::from(uri);
        url.push_str(&cur_word);
        //println!("{}", url);
        let req = reqwest::get(url).await.unwrap();
        match req.status(){
            reqwest::StatusCode::OK => {
                println!("Founded {}", &req.url().domain().unwrap());
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                println!("Founded, requires a token: {}", &req.url().domain().unwrap());
            }
            _ => {
                println!("Couldn't find: {}", &req.url().domain().unwrap());
                //panic!("something went wrong with the response status code");
            }
        }

    }
    //println!("{:?}",req);
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


