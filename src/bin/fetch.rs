use reqwest::blocking::{get, Response};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Quotes {
    id: u8,
    quote: String,
    author: String
}

#[derive(Deserialize, Debug)]
struct ResponseData {
    quotes: Vec<Quotes>,
    total: u32,
    skip: u32,
    limit: u32
}

fn main() {
    let url = "https://dummyjson.com/quotes?limit=4";

    let res: Response = get(url).expect("Fail to fetch data");
    let data: ResponseData  = res.json().expect("fail to json json");

    println!("-------------------------------");
    for quote in data.quotes {
        println!("id: {}", quote.id);
        println!("quote: {}", quote.quote);
        println!("author: {}", quote.author);
        println!("-------------------------------");
    }
    println!("total: {}", data.total);
    println!("skip: {}", data.skip);
    println!("limit: {}", data.limit);
    println!("-------------------------------");
}