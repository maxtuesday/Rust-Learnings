use std::io;

fn main() {
    println!("Generate the nth Fibonacci number");

    let result = fibonacci(get_index());

    println!("The Fibonacci number is: {}", result);
}

fn get_index() -> u32 {
    loop {
        println!("Submit index n:");
        let mut n = String::new();
        io::stdin().read_line(&mut n)
            .expect("Read line failed!");
        match n.trim().parse() {
            Ok(num) => return num,
            Err(_) => continue
        };
    }
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2)
    }
}
