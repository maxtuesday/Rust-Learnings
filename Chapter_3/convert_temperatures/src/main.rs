use std::io;

fn main() {
    println!("Convert Temperatures!");
    let temperature = read_temperature();

    loop {
        println!("Convert temperature to:");
        println!("(1) Fahrenheit");
        println!("(2) Celsius");

        let mut degree_type = String::new();
        io::stdin().read_line(&mut degree_type)
            .expect("Read line failed!");

        match degree_type.trim().parse::<i32>() {
            Ok(num) => {
                if num == 1 {
                    let f = convert_c_to_f(temperature);
                    println!("{}C -> {}F", temperature, f);
                    break
                } else if num == 2 {
                    let c = convert_f_to_c(temperature);
                    println!("{}F -> {}C", temperature, c);
                    break
                } else {
                    println!("Not a valid selection");
                    continue
                }
            },
            Err(_) => continue
        };
    }
}

fn read_temperature() -> i32 {
    loop {
        println!("Submit temperature:");
        let mut temperature = String::new();
        io::stdin().read_line(&mut temperature)
            .expect("Read line failed!");
        let temperature: i32 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        break temperature;
    }
}

fn convert_f_to_c(fahrenheit: i32) -> f32 {
    let f = fahrenheit as f32;
    (f - 32.0) * (5.0 / 9.0)
}

fn convert_c_to_f(celsius: i32) -> f32 {
    let c = celsius as f32;
    (c * (9.0 / 5.0)) + 32.0
}
