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

fn fibonacci(n: u32) -> u64 {
    let mut ans: (u64, u64, u64) = (0, 1, 0);
    for _i in 1..n {
        ans.2 = ans.0 + ans.1;
        ans.0 = ans.1;
        ans.1 = ans.2;
    }
    ans.2
}
