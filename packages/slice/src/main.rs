fn main() {
    let s = String::from("hello world");
    
    let word = first_word(&s);

    let slice = &s[..2];

    print!("{} {} {}", word, slice, s);

    let user = builder_user(String::from("4wunian@gmail.com"), String::from("Talbot3"));
    print!("{} {} {}", user.username, user.email, user.sign_in_count);

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user
    };
    print!("{} {}", user2.username, user2.email);
    println!("\n{:#?}", user2);

    let rect1 = Rectangle {
        width: 30,
        height: 50
    };
    println!(
        "The area of the rectangle is {} sequare pixels.",
        rect1.area()
    );

    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let seq = Rectangle::square(3);
    println!("{:#?}", seq);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

#[derive(Debug)] 
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {width: size, height: size}
    }
}

fn builder_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}