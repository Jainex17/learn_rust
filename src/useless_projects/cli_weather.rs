use std::io::stdin;
use reqwest::blocking::get;
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    main: Main,
    weather: Vec<Weather>,
    name: String,
}

#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    humidity: u64,
}

#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}

fn get_weather(url: &String) -> Result<WeatherResponse, reqwest::Error>{
    let res =  get(url)?.json::<WeatherResponse>()?;
    Ok(res)
}



pub fn main(){
    
    println!("Welcome to the weather app!");
    println!("Enter the city name: ");

    let mut city = String::new();
    stdin().read_line(&mut city).unwrap_or_else(|_| {
        println!("error while taking input from user");
        std::process::exit(1);
    });
    
    let api_key = env::var("API_KEY").unwrap_or_else(|_| {
        println!("can't get api key");
        std::process::exit(1);
    });

    let url =  format!("http://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric", city, api_key);

    let data = match get_weather(&url) {
        Ok(data) => data,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    println!("------------------------------");
    println!("city: {}", data.name);
    println!("temperature: {}°C", data.main.temp);
    println!("humidity: {}%", data.main.humidity);
    println!("weather: {}", data.weather[0].description);
    println!("------------------------------");
}