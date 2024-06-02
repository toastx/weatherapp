use std::io;
use serde::Deserialize;
use colored::*;

#[derive(Deserialize, Debug)]
struct WeatherResponse{
    weather: Vec<Weather>,
    main:Main,
    wind:Wind,
    name:String, 
}
#[derive(Deserialize, Debug)]
struct Weather{
    description: String,
}
#[derive(Deserialize, Debug)]
struct Main{
    temp:f64,
    humidity:f64,
    pressure:f64
}
#[derive(Deserialize, Debug)]
struct Wind{
    speed:f64,
}

fn get_weather_info(city:&str, country_code: &str, api_key: &str) -> 
Result<WeatherResponse, reqwest::Error>{

    let url : String =  format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&,{}&units=metric&appid={}", 
        city,country_code,api_key);
    let response = reqwest::blocking::get(&url)?;
    let response_json: WeatherResponse = response.json::<WeatherResponse>()?;
    Ok(response_json)
}

fn display_weather_info(response: &WeatherResponse){
    let description:&String =&response.weather[0].description;
    let temperature:f64 =   response.main.temp;
    let humidity:f64 =  response.main.humidity;
    let pressure:f64 =  response.main.pressure;
    let wind_speed:f64 =    response.wind.speed;

    let weather_text : String = format!("
    Weather in {}: {} {}
    >TemperatureL {:.1})c,
    >Humidity: {:.1}%,
    >PressureL {:.1}hPa,
    >wind Speed: {:.1}m/s",
    response.name,
    description,
    get_temp_emoji(temperature),
    temperature,
    humidity,
    pressure,
    wind_speed
    );

    let weather_text_colored = match description.as_str() {
        "clear sky" => weather_text.bright_yellow(),
        "few clouds" | "scattered clouds" | "broken clouds" => weather_text.bright_blue(),
        "overcast clouds" | "mist" | "haze" | "smoke" | "sand" | "dust" | "fog" | "squalls" => weather_text.dimmed(),
        "shower rain" | "rain" | "thunderstorm" | "snow" => weather_text.bright_cyan(),
        _ => weather_text.normal(),
    };

    fn get_temp_emoji(temperature:f64)-> &'static str{
        if temperature < 0.0{
            "❄️"
        }
        else if temperature >= 0.0 && temperature < 10.0{
            "☁️"
        }
        else if temperature>=10.0 && temperature<20.0{
            "⛅"
        }
        else if temperature>=20.0 && temperature<30.0{
            "☀️"
        }
        else{
            "🔥"
        }

    }
}

fn main() {
    println!("{}", "get your weather here".bright_blue());
    loop{
        println!("{}","Please enter the name of your city".bright_green() );
        let mut city = String::new();
        io::stdin().read_line(&mut city).expect("Please enter the name of your city");
        let city = city.trim();

        println!("{}","Please enter the name of your country".bright_green() );
        let mut country = String::new();
        io::stdin().read_line(&mut country).expect("Please enter the name of your country");
        let country = country.trim();

        let api_key = "b48061f5d150be943b848a7a26224336";
        match get_weather_info(&city, &country, api_key){
            Ok(response)=>{
                display_weather_info(&response);
            }
            Err(err)=>{
                println!("Error:{}", err);
            }
        }
    }
}
