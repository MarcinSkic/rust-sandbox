fn main() {
    #[derive(Debug)]
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));

    println!("{:#?}", home);
    println!("{:#?}", loopback);
}

fn match_can_consume_ownership() {
    #[derive(Debug)]
    enum Either {
        Left(usize),
        Right(String),
    }

    let x = Either::Right(String::from("Hello world"));
    // if it was x then it couldn't be used in println
    let value = match &x {
        Either::Left(n) => *n,
        Either::Right(s) => s.len(),
    };
    println!("{x:?} {value}");
}
