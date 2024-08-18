#[derive(Debug)]
// no data
 pub enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
pub struct IpAddr<'a> {
    kind: IpAddrKind,
    addr: &'a str,
}

#[derive(Debug)]
// with data
pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self){}
}

// simple optional enum
#[derive(Debug)]
pub enum  MyOption<T> {
    Some(T),
    None
}

#[test]
fn test_chapter6() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("{:?}  {:?}", four, six);
    let home = IpAddr { kind: IpAddrKind::V4, addr: "127.0.01" };
    let loopback = IpAddr { kind: IpAddrKind::V6, addr: "::1" };
    println!("{:?} {:?}", home, loopback);

    let m: Message = Message::Write("Hello".to_string());
    m.call();
    let other: Message = Message::Write("Other".to_string());
    m.call();
    other.call();

    let op: Option<i32> = None;
    let op1: Option<String> = Option::Some("Hi".to_string());
    eprintln!("{:?}  {:?}", op, op1);
    let my_op: MyOption<i32> = MyOption::None;
    let my_op1: MyOption<String> = MyOption::Some("Hi".to_string());
    eprintln!("myOp {:?} {:?}", my_op, my_op1);
}
