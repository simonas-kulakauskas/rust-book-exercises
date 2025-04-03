fn main() {
    #[derive(Debug)]
    enum IpAddr {
        V4(String),
        V6(String),
    }
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    println!("{:?}", home);
    println!("{:?}", loopback);


    // Other Example
    enum Message {
        Quit,
        Move { x: i32, y: i32},
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    // impl Message {
    //     fn call(&self) {
    //         // Method for calling would be defined in here...
    //     }
    // }


    // Example of Option - There are no 'nulls' in Rust!
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
}
