fn main() {
    println!("Hello, enum!");
    {
        enum IpAddrKind {
            V4,
            V6,
        }

        enum IpAddr {
            V4(String),
            V6(String),
        }
        fn route(ip_kind: IpAddrKind) {}

        let four = IpAddrKind::V4;
        let six = IpAddrKind::V6;

        route(IpAddrKind::V4);
        route(IpAddrKind::V6);

        let home = IpAddr::V4(String::from("127.0.0.1"));

        let loopback = IpAddr::V6(String::from("::1"));
    }

    {
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }

        impl Message {
            fn call(&self) {
                // method body would be defined here
            }
        }

        let m = Message::Write(String::from("hello"));
        m.call();
    }

    {
        let some_number = Some(5);
        let some_string = Some("a string");

        let absent_number: Option<i32> = None;

    }
}

