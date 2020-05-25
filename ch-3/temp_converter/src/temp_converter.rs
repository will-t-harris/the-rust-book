use std::io;

fn main() {
    println!("Please input C to convert farenheit to celcius.");
    println!("Please input F to convert celcius to farenheit.");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error with your input");

    if input.trim() == "F" {
        println!("Please input the celcius value you'd like to convert to farenheit:");

        let mut celcius_input = String::new();

        io::stdin()
            .read_line(&mut celcius_input)
            .expect("Error with the celcius input");

        let value = convert_to_farenheit(celcius_input.trim().parse::<f32>().unwrap());

        println!("That is {} degrees farenheit!", value);
    }

    if input.trim() == "C" {
        println!("Please input the farenheit value you'd like to convert to celcius:");

        let mut farenheit_input = String::new();

        io::stdin()
            .read_line(&mut farenheit_input)
            .expect("Error with the farenheit input");

        let value = convert_to_celcuis(farenheit_input.trim().parse().unwrap());

        println!("That is {} degrees celcius!", value);
    }
}

fn convert_to_celcuis(temp: f32) -> f32 {
    let converted_temp = (&temp - 32.0) * (5.0 / 9.0);
    converted_temp
}

fn convert_to_farenheit(temp: f32) -> f32 {
    let converted_temp = (&temp - 32.0) / 1.8;
    converted_temp
}
