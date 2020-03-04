fn main() {
    println!("References!");

    {
        let s1 = String::from("hello");

        let len = calculate_length(&s1);

        println!("The length of '{}' is {}.", s1, len);
    }

    {
        let mut s = String::from("hello");
        change(&mut s);

        println!("{}", s);
    }

    {
        let mut s = String::from("hello");

        let r1 = &mut s;

        println!("{}", r1);
    }
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}