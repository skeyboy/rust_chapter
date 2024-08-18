#[derive(Debug, Clone, Copy)]
pub struct User<'a> {
    name: &'a str,
    aage: i32,
    addr: &'a str,
}

#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

 impl Rectangle {
   pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn init(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }

   pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
#[test]
fn test_chapter5() {
    let user: User = User { name: "Hi", aage: 32, addr: "China" };
    assert_eq!(user.name, "Hi");
    assert_eq!(user.aage, 32);
    assert_eq!(user.addr, "China");

    let user2: User = User {
        name: "World",
        ..user // user other properties
    };
    assert_eq!(user2.aage, 32);
    assert_eq!(user2.name, "World");

    let rect = Rectangle::init(1);
    assert_eq!(rect.height, 1);
    assert_eq!(rect.width, 1);
    assert_eq!(rect.area(), 1);

    let r1 = Rectangle::init(2);
    let r2 = Rectangle::init(3);
    assert_eq!(r1.can_hold(&r2), false);
    assert_eq!(r2.can_hold(&r1), true);
}
