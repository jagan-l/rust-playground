use std::io;

fn main() {
    loop {
    println!("please Enter the conversion type:");
    println!("1. Fahrenheit to Celsius");
    println!("2. Celsius to Fahrenheit");
    
    let mut conversion_type = String::new();
    io::stdin().read_line(&mut conversion_type)
        .expect("Failed to read the conversion type");
    let conversion_type : u32 = match conversion_type.trim() {
        "1" => 1,
        "2" => 2,
        _ => {
            println!("please input either 1 or 2!");
            break;
        } 
    };
   
    println!("Please input the temperature");
    let mut temperature = String::new();
    io::stdin().read_line(&mut temperature)
        .expect("failed to read temperature");
    let temperature: f64 = match temperature.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please input a valid temperature!");
            continue;
        }

    };

    let converted_temperature = if conversion_type == 1 {
        (temperature - 32.) / 1.8
    } else {
        temperature * 1.8 + 32.
    };
    println!("The converted temperature is {}", converted_temperature);
}
}

