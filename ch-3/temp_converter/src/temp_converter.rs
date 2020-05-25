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

        let mut celciusInput = String::new();

        io::stdin()
            .read_line(&mut celciusInput)
            .expect("Error with the celcius input");

        let value = convert_to_farenheit(celciusInput.trim().parse::<f32>().unwrap());

        println!("That is {} degrees farenheit!", value);
    }
}

fn convert_to_celcuis() {}

fn convert_to_farenheit(temp: f32) -> f32 {
    let converted_temp = (&temp - 32.0) / 1.8;
    converted_temp
}
