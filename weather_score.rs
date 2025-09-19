use std::io;

fn main() {
    let mut wind = String::new();
    println!("Enter your wind speed (m/s): ");
    io::stdin().read_line(&mut wind).unwrap();
    let wind_speed: f32 = wind.trim().parse().unwrap();

    let mut input = String::new();
    println!("Enter your temperature (°C): ");
    io::stdin().read_line(&mut input).unwrap();
    let temperature: f32 = input.trim().parse().unwrap();

    let weather_score = calculate_weather_score(temperature, wind_speed);
    println!("The weather score is: {:.2}", weather_score);
}

fn calculate_weather_score(temperature: f32, wind_speed: f32) -> f32 {
    let temp_score = if temperature < 0.0 || temperature > 35.0 {
        1.0
    } else {
        100.0 - (temperature - 22.5).abs() * 4.0
    };

    let wind_score = if wind_speed > 30.0 {
        1.0
    } else {
        100.0 - (wind_speed - 5.0).abs() * 2.0
    };

    (temp_score + wind_score) / 2.0
}
